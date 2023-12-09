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
use tukosmo_domain::core::language::model::I18nTranslationSearchCriteria;
use tukosmo_domain::core::language::model::I18nTranslationSearchFilterCriteria;
use tukosmo_domain::core::shared::error;
use tukosmo_domain::core::shared::model::DomainError;
use uuid::Uuid;

use crate::core::shared::diesel_orm::schema::i18n_translation;
use crate::core::shared::diesel_orm::statement::ModelSqlExecutor;
use super::super::model::DbI18nTranslation;

pub struct I18nTranslationSqlExecutor {
    connection: Rc<RefCell<PgConnection>>,
}

impl ModelSqlExecutor<
    DbI18nTranslation,
    (
        diesel::sql_types::Uuid,
        diesel::sql_types::Uuid,
        diesel::sql_types::Uuid,
        diesel::sql_types::Text,
        diesel::sql_types::Timestamptz,
        diesel::sql_types::Timestamptz,
    ),
    I18nTranslationSearchCriteria,
    I18nTranslationSearchFilterCriteria,
    i18n_translation::table
>
for I18nTranslationSqlExecutor {
    fn delete(
        &mut self,
        filter_criteria: I18nTranslationSearchFilterCriteria
    ) -> Result<(), DomainError> {
        let mut connection = self.connection.borrow_mut();
        let connection = connection.deref_mut();

        let mut statement = diesel
            ::delete(i18n_translation::table)
            .into_boxed();

        if let Some(i18n_translation_id) = filter_criteria.id {
            statement = statement.filter(
                i18n_translation::id.eq(i18n_translation_id.value().clone())
            );
        }
        if let Some(i18n_translation_ids) = filter_criteria.id_in {
            statement = statement.filter(
                i18n_translation::id.eq_any(
                    i18n_translation_ids
                        .iter()
                        .map(|v| v.value().clone())
                        .collect::<Vec<Uuid>>()
                )
            );
        }
        if let Some(i18n_translation_ids) = filter_criteria.id_not_in {
            statement = statement.filter(
                i18n_translation::id.ne_all(
                    i18n_translation_ids
                        .iter()
                        .map(|v| v.value().clone())
                        .collect::<Vec<Uuid>>()
                )
            );
        }
        if let Some(i18n_text_id) = filter_criteria.i18n_text_id {
            statement = statement.filter(
                i18n_translation::i18n_text_id.eq(i18n_text_id.value().clone())
            );
        }
        if let Some(i18n_text_ids) = filter_criteria.i18n_text_id_in {
            statement = statement.filter(
                i18n_translation::i18n_text_id.eq_any(
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

    fn get_table() -> i18n_translation::table {
        i18n_translation::table
    }

    fn init(connection: Rc<RefCell<PgConnection>>) -> Self {
        Self { connection }
    }

    fn select(
        &mut self,
        search_criteria: I18nTranslationSearchCriteria
    ) -> Result<Vec<DbI18nTranslation>, DomainError> {
        let mut connection = self.connection.borrow_mut();
        let connection = connection.deref_mut();

        let query = Self::select_query(search_criteria.filter);
        let results = query
            .select(DbI18nTranslation::as_select())
            .load(connection)
            .map_err(|_e| error::CANNOT_EXECUTE_SELECT_ON_DATABASE)?;

        Ok(results)
    }

    fn select_query<'a>(
        filter_criteria: I18nTranslationSearchFilterCriteria
    ) -> BoxedSelectStatement<
        'a,
        (
            diesel::sql_types::Uuid,
            diesel::sql_types::Uuid,
            diesel::sql_types::Uuid,
            diesel::sql_types::Text,
            diesel::sql_types::Timestamptz,
            diesel::sql_types::Timestamptz,
        ),
        FromClause<i18n_translation::table>,
        Pg
    > {
        let mut query = i18n_translation::table.into_boxed();

        if let Some(i18n_translation_id) = filter_criteria.id {
            query = query.filter(
                i18n_translation::id.eq(i18n_translation_id.value().clone())
            );
        }
        if let Some(i18n_translation_ids) = filter_criteria.id_in {
            query = query.filter(
                i18n_translation::id.eq_any(
                    i18n_translation_ids
                        .iter()
                        .map(|v| v.value().clone())
                        .collect::<Vec<Uuid>>()
                )
            );
        }
        if let Some(i18n_translation_ids) = filter_criteria.id_not_in {
            query = query.filter(
                i18n_translation::id.ne_all(
                    i18n_translation_ids
                        .iter()
                        .map(|v| v.value().clone())
                        .collect::<Vec<Uuid>>()
                )
            );
        }
        if let Some(i18n_text_id) = filter_criteria.i18n_text_id {
            query = query.filter(
                i18n_translation::i18n_text_id.eq(i18n_text_id.value().clone())
            );
        }
        if let Some(i18n_text_ids) = filter_criteria.i18n_text_id_in {
            query = query.filter(
                i18n_translation::i18n_text_id.eq_any(
                    i18n_text_ids
                        .iter()
                        .map(|v| v.value().clone())
                        .collect::<Vec<Uuid>>()
                )
            );
        }

        query
    }

    fn update(
        &mut self,
        db_i18n_translation: &DbI18nTranslation
    ) -> Result<(), DomainError> {
        let mut connection = self.connection.borrow_mut();
        let connection = connection.deref_mut();

        let result = diesel
            ::update(i18n_translation::table.find(db_i18n_translation.id))
            .set(db_i18n_translation)
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
        db_i18n_translations: Vec<DbI18nTranslation>
    ) -> Result<(), DomainError> {
        let mut connection = self.connection.borrow_mut();
        let connection = connection.deref_mut();

        use diesel::query_dsl::methods::FilterDsl;
        let result = diesel
            ::insert_into(i18n_translation::table)
            .values(db_i18n_translations)
            .on_conflict((
                i18n_translation::i18n_text_id,
                i18n_translation::language_id,
            ))
            .do_update()
            .set((
                i18n_translation::text.eq(
                    diesel::pg::upsert::excluded(i18n_translation::text)
                ),
                i18n_translation::update_date.eq(
                    diesel::pg::upsert::excluded(i18n_translation::update_date)
                ),
            ))
            .filter(
                i18n_translation::id.eq(
                    diesel::pg::upsert::excluded(i18n_translation::id)
                )
            )
            .execute(connection);

        match result {
            Ok(_inserted_rows) => Ok(()),
            Err(_e) => Err(error::CANNOT_EXECUTE_INSERT_ON_DATABASE),
        }
    }
}
