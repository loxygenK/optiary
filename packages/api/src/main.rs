mod entity;
mod repository;
mod service;
mod setup;
mod types;

use std::sync::Arc;

use crate::repository::{MockTaskRepository, MockTodoRepository};
use crate::setup::SetupConfiguration;
use crate::service::*;

fn main() {
    let config = SetupConfiguration::default();

    let todo_serv = TodoRetrieveService::new(config.todo_repository(), config.task_repository());
    println!("{:#?}", todo_serv.list());
}
