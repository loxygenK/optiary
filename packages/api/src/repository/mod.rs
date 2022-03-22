#[cfg(feature = "memory-db")]
mod memory;

#[cfg(feature = "memory-db")]
pub use memory::*;

#[cfg(all(feature = "memory-db", feature = "postgres-db"))]
compile_error!("'memory-db' and 'postgres-db' cannot be set at the same time");

#[cfg(not(any(feature = "memory-db", feature = "postgres-db")))]
compile_error!("[repo] Either of 'memory-db' and 'postgres-db' should be set");

use crate::entity::Todo;
use crate::entity::Task;
use crate::types::Id;

use thiserror::Error;

#[derive(PartialEq, Debug, Error)]
pub enum RepositoryError {
    #[error("The target object should be found in this context, but it wasn't")]
    NotFound,

    #[error("Internal error occured")]
    InternalError(String)
}
pub type RepositoryResult<T> = Result<T, RepositoryError>;

pub trait TaskRepository {
    fn fetch_by_id(&self, id: &Id) -> Option<Task>;
    fn create(&mut self, todo: &Task) -> RepositoryResult<()>;
    fn update(&mut self, task: &Task) -> RepositoryResult<Task>;
    fn remove(&mut self, task: Task) -> RepositoryResult<()>;
}

pub trait TodoRepository {
    fn fetch_by_id(&self, id: &Id) -> Option<Todo>;
    fn fetch_by_task(&self, task_id: &Id) -> RepositoryResult<Vec<Todo>>;
    fn create(&mut self, todo: &Todo) -> RepositoryResult<()>;
    fn update(&mut self, todo: &Todo) -> RepositoryResult<Todo>;
    fn remove(&mut self, todo: Todo) -> RepositoryResult<()>;
}
