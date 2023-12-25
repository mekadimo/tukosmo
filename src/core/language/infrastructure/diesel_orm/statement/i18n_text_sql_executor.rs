use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::SelectableHelper;
use diesel::pg::Pg;
use diesel::pg::PgConnection;
use diesel::query_builder::BoxedSelectStatement;
use diesel::query_builder::FromClause;
use diesel;
use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;
use tukosmo_domain::core::language::model::I18nTextSearchCriteria;
use tukosmo_domain::core::language::model::I18nTextSearchFilterCriteria;
use tukosmo_domain::core::shared::error;
use tukosmo_domain::core::shared::model::DomainError;
use uuid::Uuid;

use crate::core::shared::diesel_orm::schema::i18n_text;
use crate::core::shared::diesel_orm::statement::ModelSqlExecutor;
use super::super::model::DbI18nText;

pub struct I18nTextSqlExecutor {
    connection: Rc<RefCell<PgConnection>>,
}

impl ModelSqlExecutor<
    DbI18nText,
    (
        diesel::sql_types::Timestamptz,
        diesel::sql_types::Text,
        diesel::sql_types::Uuid,
        diesel::sql_types::Timestamptz,
    ),
    I18nTextSearchCriteria,
    I18nTextSearchFilterCriteria,
    i18n_text::table
>
for I18nTextSqlExecutor {
    fn delete(
        &mut self,
        filter_criteria: I18nTextSearchFilterCriteria
    ) -> Result<(), DomainError> {
        let mut connection = self.connection.borrow_mut();
        let connection = connection.deref_mut();

        let mut statement = diesel::delete(i18n_text::table).into_boxed();

        if let Some(i18n_text_id) = filter_criteria.id {
            statement = statement.filter(
                i18n_text::id.eq(i18n_text_id.value().clone())
            );
        }
        if let Some(i18n_text_ids) = filter_criteria.id_in {
            statement = statement.filter(
                i18n_text::id.eq_any(
                    i18n_text_ids
                        .iter()
                        .map(|v| v.value().clone())
                        .collect::<Vec<Uuid>>()
                )
            );
        }

        let result = statement.execute(connection);

        match result {
            Ok(_affected_rows) => Ok(()),
            Err(_e) => Err(error::CANNOT_EXECUTE_DELETE_ON_DATABASE),
        }
    }

    fn get_connection(&mut self) -> Rc<RefCell<PgConnection>> {
        Rc::clone(&self.connection)
    }

    fn get_table() -> i18n_text::table {
        i18n_text::table
    }

    fn init(connection: Rc<RefCell<PgConnection>>) -> Self {
        Self { connection }
    }

    fn select(
        &mut self,
        search_criteria: I18nTextSearchCriteria
    ) -> Result<Vec<DbI18nText>, DomainError> {
        let mut connection = self.connection.borrow_mut();
        let connection = connection.deref_mut();

        let query = Self::select_query(search_criteria.filter);
        let results = query
            .select(DbI18nText::as_select())
            .load(connection)
            .map_err(|_e| error::CANNOT_EXECUTE_SELECT_ON_DATABASE)?;

        Ok(results)
    }

    fn select_query<'a>(
        filter_criteria: I18nTextSearchFilterCriteria
    ) -> BoxedSelectStatement<
        'a,
        (
            diesel::sql_types::Timestamptz,
            diesel::sql_types::Text,
            diesel::sql_types::Uuid,
            diesel::sql_types::Timestamptz,
        ),
        FromClause<i18n_text::table>,
        Pg
    > {
        let mut query = i18n_text::table.into_boxed();

        if let Some(i18n_text_id) = filter_criteria.id {
            query = query.filter(
                i18n_text::id.eq(i18n_text_id.value().clone())
            );
        }
        if let Some(i18n_text_ids) = filter_criteria.id_in {
            query = query.filter(
                i18n_text::id.eq_any(
                    i18n_text_ids
                        .iter()
                        .map(|v| v.value().clone())
                        .collect::<Vec<Uuid>>()
                )
            );
        }

        query
    }

    fn update(&mut self, db_i18n_text: &DbI18nText) -> Result<(), DomainError> {
        let mut connection = self.connection.borrow_mut();
        let connection = connection.deref_mut();

        let result = diesel
            ::update(i18n_text::table.find(db_i18n_text.id))
            .set(db_i18n_text)
            .execute(connection);

        match result {
            Ok(0) => Err(error::NOTHING_TO_UPDATE_ON_DATABASE),
            Ok(1) => Ok(()),
            Ok(_) => Err(error::UNDESIRED_UPDATES_ON_DATABASE),
            Err(_e) => Err(error::CANNOT_EXECUTE_UPDATE_ON_DATABASE),
        }
    }

    fn upsert_in_bulk(
        &mut self,
        db_i18n_texts: Vec<DbI18nText>
    ) -> Result<(), DomainError> {
        let mut connection = self.connection.borrow_mut();
        let connection = connection.deref_mut();

        let result = diesel
            ::insert_into(i18n_text::table)
            .values(db_i18n_texts)
            .on_conflict(i18n_text::id)
            .do_update()
            .set((
                i18n_text::default_text.eq(
                    diesel::pg::upsert::excluded(i18n_text::default_text)
                ),
                i18n_text::update_date.eq(
                    diesel::pg::upsert::excluded(i18n_text::update_date)
                ),
            ))
            .execute(connection);

        match result {
            Ok(_inserted_rows) => Ok(()),
            Err(_e) => Err(error::CANNOT_EXECUTE_INSERT_ON_DATABASE),
        }
    }
}
