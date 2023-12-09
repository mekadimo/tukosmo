use std::cell::RefCell;
use std::rc::Rc;

use crate::core::language::repository::LanguageRepository;
use crate::core::shared::model::DomainError;

pub struct Transaction {
    pub language_repository: Rc<RefCell<dyn LanguageRepository>>,
}

pub trait TransactionExecutor {
    fn begin(&mut self) -> Result<Transaction, DomainError>;

    fn commit(&mut self) -> Result<(), DomainError>;

    fn rollback(&mut self) -> Result<(), DomainError>;
}
