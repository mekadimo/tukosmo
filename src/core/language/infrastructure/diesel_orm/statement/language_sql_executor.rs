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
use tukosmo_domain::core::language::model::LanguageSearchCriteria;
use tukosmo_domain::core::language::model::LanguageSearchCriteriaOrderBy;
use tukosmo_domain::core::language::model::LanguageSearchFilterCriteria;
use tukosmo_domain::core::shared::error;
use tukosmo_domain::core::shared::model::DomainError;

use crate::core::shared::diesel_orm::schema::language;
use crate::core::shared::diesel_orm::statement::ModelSqlExecutor;
use super::super::model::DbLanguage;

pub struct LanguageSqlExecutor {
    connection: Rc<RefCell<PgConnection>>,
}

impl ModelSqlExecutor<
    DbLanguage,
    (
        diesel::sql_types::Text,
        diesel::sql_types::Timestamptz,
        diesel::sql_types::Uuid,
        diesel::sql_types::Uuid,
        diesel::sql_types::Text,
        diesel::sql_types::Timestamptz,
        diesel::sql_types::Text,
        diesel::sql_types::Text,
    ),
    LanguageSearchCriteria,
    LanguageSearchFilterCriteria,
    language::table
>
for LanguageSqlExecutor {
    const TABLE: language::table = language::table;

    fn delete(
        &mut self,
        filter_criteria: LanguageSearchFilterCriteria
    ) -> Result<(), DomainError> {
        let mut connection = self.connection.borrow_mut();
        let connection = connection.deref_mut();

        let mut statement = diesel::delete(language::table).into_boxed();

        if let Some(language_id) = filter_criteria.id {
            statement = statement.filter(
                language::id.eq(language_id.value().clone())
            );
        }
        if let Some(code) = filter_criteria.code {
            statement = statement.filter(
                language::code.eq(code.value().to_string())
            );
        }
        if let Some(not_language_id) = filter_criteria.not_id {
            statement = statement.filter(
                language::id.ne(not_language_id.value().clone())
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

    fn init(connection: Rc<RefCell<PgConnection>>) -> Self {
        Self { connection }
    }

    fn select(
        &mut self,
        search_criteria: LanguageSearchCriteria
    ) -> Result<Vec<DbLanguage>, DomainError> {
        let mut connection = self.connection.borrow_mut();
        let connection = connection.deref_mut();

        let mut query = Self::select_query(search_criteria.filter);
        if let Some(order_by) = search_criteria.order_by {
            match order_by {
                LanguageSearchCriteriaOrderBy::CreationDate => {
                    query = query.order(language::creation_date.asc());
                }
                LanguageSearchCriteriaOrderBy::OriginalName => {
                    query = query.order(language::original_name.asc());
                }
            }
        }

        let select = query.select(DbLanguage::as_select());

        let results = (
            if let Some(pagination) = search_criteria.pagination {
                select
                    .offset((pagination.page - 1) * pagination.results_per_page)
                    .limit(pagination.results_per_page)
            } else {
                select
            }
        )
            .load(connection)
            .map_err(|_e| error::CANNOT_EXECUTE_SELECT_ON_DATABASE)?;

        Ok(results)
    }

    fn select_query<'a>(
        filter_criteria: LanguageSearchFilterCriteria
    ) -> BoxedSelectStatement<
        'a,
        (
            diesel::sql_types::Text,
            diesel::sql_types::Timestamptz,
            diesel::sql_types::Uuid,
            diesel::sql_types::Uuid,
            diesel::sql_types::Text,
            diesel::sql_types::Timestamptz,
            diesel::sql_types::Text,
            diesel::sql_types::Text,
        ),
        FromClause<language::table>,
        Pg
    > {
        let mut query = language::table.into_boxed();

        if let Some(language_id) = filter_criteria.id {
            query = query.filter(language::id.eq(language_id.value().clone()));
        }
        if let Some(code) = filter_criteria.code {
            query = query.filter(language::code.eq(code.value().to_string()));
        }
        if let Some(not_language_id) = filter_criteria.not_id {
            query = query.filter(
                language::id.ne(not_language_id.value().clone())
            );
        }

        query
    }

    fn update(&mut self, db_language: &DbLanguage) -> Result<(), DomainError> {
        let mut connection = self.connection.borrow_mut();
        let connection = connection.deref_mut();

        let result = diesel
            ::update(language::table.find(db_language.id))
            .set(db_language)
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
        db_languages: Vec<DbLanguage>
    ) -> Result<(), DomainError> {
        let mut connection = self.connection.borrow_mut();
        let connection = connection.deref_mut();

        let result = diesel
            ::insert_into(language::table)
            .values(db_languages)
            .on_conflict(language::id)
            .do_update()
            .set((
                language::code.eq(diesel::pg::upsert::excluded(language::code)),
                language::update_date.eq(
                    diesel::pg::upsert::excluded(language::update_date)
                ),
                language::original_name.eq(
                    diesel::pg::upsert::excluded(language::original_name)
                ),
                language::website_title.eq(
                    diesel::pg::upsert::excluded(language::website_title)
                ),
                language::website_subtitle.eq(
                    diesel::pg::upsert::excluded(language::website_subtitle)
                ),
            ))
            .execute(connection);

        match result {
            Ok(_inserted_rows) => Ok(()),
            Err(_e) => Err(error::CANNOT_EXECUTE_INSERT_ON_DATABASE),
        }
    }
}
