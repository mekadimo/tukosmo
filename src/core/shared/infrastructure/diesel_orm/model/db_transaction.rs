use diesel::connection::SimpleConnection;
use diesel::pg::PgConnection;
use dotenvy::dotenv;
use std::cell::RefCell;
use std::env;
use std::ops::DerefMut;
use std::rc::Rc;
use tukosmo_domain::core::shared::model::DomainError;
use tukosmo_domain::core::shared::model::Transaction;
use tukosmo_domain::core::shared::model::TransactionExecutor;
use tukosmo_domain::core::shared::error;

use crate::core::language::diesel_orm::repository::DbLanguageRepository;
use crate::core::user::diesel_orm::repository::DbUserRepository;

pub struct DbTransactionExecutor {
    connection: Rc<RefCell<PgConnection>>,
}

impl DbTransactionExecutor {
    pub fn init() -> Result<Self, DomainError> {
        dotenv().ok();
        let database_url = env
            // TODO: Use Tukosmo.toml instead of env var
            ::var("DATABASE_URL")
            .map_err(|_e| error::CANNOT_OBTAIN_DATABASE_CREDENTIALS)?;

        use diesel::Connection;
        let connection = (match PgConnection::establish(&database_url) {
            Ok(connection) => Ok(Rc::new(RefCell::new(connection))),
            Err(_e) => Err(error::CANNOT_ESTABLISH_DATABASE_CONNECTION),
        })?;
        Ok(Self { connection })
    }
}

impl TransactionExecutor for DbTransactionExecutor {
    fn begin(&mut self) -> Result<Transaction, DomainError> {
        let mut connection = self.connection.borrow_mut();
        let connection = connection.deref_mut();
        match connection.batch_execute("BEGIN TRANSACTION;") {
            Ok(_) => {
                let language_repository = DbLanguageRepository::init(
                    Rc::clone(&self.connection)
                );
                let user_repository = DbUserRepository::init(
                    Rc::clone(&self.connection)
                );

                let transaction = Transaction {
                    language_repository: Rc::new(
                        RefCell::new(language_repository)
                    ),
                    user_repository: Rc::new(RefCell::new(user_repository)),
                };
                Ok(transaction)
            }
            Err(_) => Err(error::CANNOT_BEGIN_TRANSACTION),
        }
    }

    fn commit(&mut self) -> Result<(), DomainError> {
        let mut connection = self.connection.borrow_mut();
        let connection = connection.deref_mut();
        match connection.batch_execute("COMMIT;") {
            Ok(_) => Ok(()),
            Err(_) => Err(error::CANNOT_COMMIT_TRANSACTION),
        }
    }

    fn rollback(&mut self) -> Result<(), DomainError> {
        let mut connection = self.connection.borrow_mut();
        let connection = connection.deref_mut();
        match connection.batch_execute("ROLLBACK;") {
            Ok(_) => Ok(()),
            Err(_) => Err(error::CANNOT_ROLLBACK_TRANSACTION),
        }
    }
}
