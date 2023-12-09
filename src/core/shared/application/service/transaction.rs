use std::cell::RefMut;
use tukosmo_domain::core::shared::model::DomainError;
use tukosmo_domain::core::shared::model::Transaction;
use tukosmo_domain::core::shared::model::TransactionExecutor;

pub fn run_transaction<T>(
    mut transaction_executor: RefMut<dyn TransactionExecutor>,
    transactional_function: impl FnOnce(Transaction) -> Result<T, DomainError>
) -> Result<T, DomainError>
    where T: Sized
{
    let transaction = transaction_executor.begin()?;

    match transactional_function(transaction) {
        Ok(result) => {
            transaction_executor.commit()?;
            Ok(result)
        }
        Err(domain_error) => {
            transaction_executor.rollback()?;
            Err(domain_error)
        }
    }
}
