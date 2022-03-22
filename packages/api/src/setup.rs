use std::sync::{Arc, Mutex};

use crate::repository::{TaskRepository, TodoRepository, MockTodoRepository, MockTaskRepository};

pub struct SetupConfiguration {
    todo_repository: Arc<Mutex<dyn TodoRepository>>,
    task_repository: Arc<Mutex<dyn TaskRepository>>
}
impl SetupConfiguration {
    pub fn default() -> SetupConfiguration {
        Self {
            todo_repository: with_lock(MockTodoRepository::new()),
            task_repository: with_lock(MockTaskRepository::new())
        }
    }

    pub fn todo_repository(&self) -> Arc<Mutex<dyn TodoRepository>> {
        Arc::clone(&self.todo_repository)
    }

    pub fn task_repository(&self) -> Arc<Mutex<dyn TaskRepository>> {
        Arc::clone(&self.task_repository)
    }
}

fn with_lock<T>(t: T) -> Arc<Mutex<T>> {
    Arc::new(Mutex::new(t))
}
