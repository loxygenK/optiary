use std::sync::{Arc, Mutex};
use anyhow::{Result, Context, bail};

use crate::repository::{TaskRepository, TodoRepository};
use crate::entity::Todo;
use crate::types::Id;

pub type ServiceResult<T> = anyhow::Result<T>;

pub struct TodoRetrieveService {
    todo_repository: Arc<Mutex<dyn TodoRepository>>,
    task_repository: Arc<Mutex<dyn TaskRepository>>
}
impl TodoRetrieveService {
    pub fn new(
        todo_repository: Arc<Mutex<dyn TodoRepository>>,
        task_repository: Arc<Mutex<dyn TaskRepository>>
    ) -> Self {
        Self { todo_repository, task_repository }
    }

    pub fn list(&self) -> ServiceResult<Vec<Todo>> {
        let todo_repo = self.todo_repository.lock();
        if todo_repo.is_err() {
            bail!("failed to lock");
        }
        let todo_repo = todo_repo.unwrap();

        todo_repo.fetch_by_task(&Id::new("task-1")).context("during fetching task")
    }
}

pub struct TaskRetrieveService
{
    task_repository: Arc<Mutex<dyn TaskRepository>>,
}
impl TaskRetrieveService {
    pub fn new(
        task_repository: Arc<Mutex<dyn TaskRepository>>
    ) -> Self {
        Self { task_repository }
    }

    pub fn list(&self) -> Vec<Todo> {
        todo!();
        // self.task_repository
    }
}
