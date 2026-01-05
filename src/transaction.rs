//! Transaction management helpers.

use sea_orm::{DatabaseConnection, TransactionTrait};
use tracing::{debug, warn};

use crate::Result;

/// Execute a function within a database transaction.
///
/// The transaction is automatically committed if the function returns `Ok`,
/// and rolled back if it returns `Err`.
///
/// # Example
///
/// ```no_run
/// use eywa_database::transaction;
/// use sea_orm::DatabaseConnection;
///
/// # async fn example(db: &DatabaseConnection) -> eywa_database::Result<()> {
/// transaction::with_transaction(db, |txn| async move {
///     // Your transactional logic here
///     // All database operations use `txn` instead of `db`
///     Ok(())
/// }).await?;
/// # Ok(())
/// # }
/// ```
pub async fn with_transaction<F, R>(db: &DatabaseConnection, f: F) -> Result<R>
where
    F: for<'txn> FnOnce(
        &'txn sea_orm::DatabaseTransaction,
    ) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<R>> + Send + 'txn>>
        + Send,
    R: Send,
{
    debug!("Starting transaction");

    let txn = db
        .begin()
        .await
        .map_err(|e| eywa_errors::AppError::DatabaseError(e))?;

    match f(&txn).await {
        Ok(result) => {
            txn.commit()
                .await
                .map_err(|e| eywa_errors::AppError::DatabaseError(e))?;
            debug!("Transaction committed successfully");
            Ok(result)
        }
        Err(e) => {
            if let Err(commit_err) = txn
                .rollback()
                .await
                .map_err(|err| eywa_errors::AppError::DatabaseError(err))
            {
                warn!("Failed to rollback transaction: {}", commit_err);
            } else {
                debug!("Transaction rolled back due to error");
            }
            Err(e)
        }
    }
}

/// Execute a function within a database transaction, returning a specific error type.
///
/// This is useful when you want to preserve your custom error type through
/// the transaction boundary.
///
/// # Example
///
/// ```no_run
/// use eywa_database::transaction;
/// use sea_orm::DatabaseConnection;
///
/// # #[derive(Debug)]
/// # enum MyError { Foo, Bar }
/// # impl From<eywa_errors::AppError> for MyError {
/// #     fn from(e: eywa_errors::AppError) -> Self { MyError::Bar }
/// # }
/// # async fn example(db: &DatabaseConnection) -> Result<(), MyError> {
/// transaction::with_transaction_custom_err(db, |txn| async move {
///     // Your transactional logic here
///     // Return Result<T, MyError>
///     Ok(())
/// }).await?;
/// # Ok(())
/// # }
/// ```
pub async fn with_transaction_custom_err<F, R, E>(
    db: &DatabaseConnection,
    f: F,
) -> std::result::Result<R, E>
where
    F: for<'txn> FnOnce(
        &'txn sea_orm::DatabaseTransaction,
    ) -> std::pin::Pin<Box<dyn futures::Future<Output = std::result::Result<R, E>> + Send + 'txn>>
        + Send,
    R: Send,
    E: From<eywa_errors::AppError> + Send,
{
    debug!("Starting transaction");

    let txn = db.begin().await.map_err(|e| {
        eywa_errors::AppError::DatabaseError(e).into()
    })?;

    match f(&txn).await {
        Ok(result) => {
            txn.commit().await.map_err(|e| {
                eywa_errors::AppError::DatabaseError(e).into()
            })?;
            debug!("Transaction committed successfully");
            Ok(result)
        }
        Err(e) => {
            if let Err(commit_err) = txn.rollback().await {
                warn!("Failed to rollback transaction: {}", commit_err);
            } else {
                debug!("Transaction rolled back due to error");
            }
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_helpers_exist() {
        // These tests verify the API exists
        // Actual transaction tests would require a running database
    }
}
