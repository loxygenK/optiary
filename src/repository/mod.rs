use crate::entity::Todo;
use crate::entity::Task;
use crate::types::WithId;

#[derive(PartialEq, Debug)]
pub enum RepositoryError {
    NotFound,
    InternalError(String)
}
pub type RepositoryResult<T> = Result<T, RepositoryError>;

pub trait TaskRepository {
    fn fetch_by_id(id: &str) -> Option<WithId<Task>>;
    fn update(task: &WithId<Task>) -> RepositoryResult<WithId<Task>>;
    fn remove(task: WithId<Task>) -> RepositoryResult<Task>;
}

pub trait TodoRepository {
    fn fetch_by_id(id: &str) -> Option<WithId<Todo>>;
    fn fetch_by_task(task_id: &str) -> RepositoryResult<Vec<WithId<Todo>>>;
    fn update(todo: &WithId<Todo>) -> RepositoryResult<WithId<Todo>>;
    fn remove(todo: WithId<Todo>) -> RepositoryResult<Todo>;
}
