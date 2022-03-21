use std::ops::Add;

use chrono::{Utc, Duration, DateTime};

use crate::entity::{Todo, Task, DateTimeRange, DoneStatus, DoneStatusList};
use crate::repository::{RepositoryResult, TodoRepository, RepositoryError};
use crate::types::Id;

struct MockTodoRepository {
    content: Vec<Todo>
}
impl MockTodoRepository {
    fn new() -> Self {
        let task: [Task; 5] = (0..4)
            .map(|i| { Task::new(Id::new(format!("task-{}", i)), format!("task-{}", i)).unwrap() })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Self {
            content: (0..100)
                .map(|i| { MockTodoRepository::generate_todo(&task, i) })
                .collect()
        }
    }

    fn generate_todo(task_candicate: &[Task], index: usize) -> Todo {
        Todo::new(
            Id::new(format!("todo-{}", index)),
            task_candicate[index % 5].to_owned(),
            MockTodoRepository::generate_range(index / 3, index % 3),
            DoneStatusList::new((0..3).map(|i| { MockTodoRepository::generate_done_status(index, i) }).collect())
        )
    }

    fn generate_done_status(date_index: usize, time_index: usize) -> DoneStatus {
        DoneStatus::new(
            Id::new(format!("date_status-{}-{}", date_index, time_index)),
            MockTodoRepository::generate_datetime(date_index, time_index),
            false
        )
    }

    fn generate_range(date_index: usize, time_index: usize) -> DateTimeRange {
        let start = MockTodoRepository::generate_datetime(date_index, time_index);
        let end = MockTodoRepository::generate_datetime(date_index + 1, time_index);

        DateTimeRange::new(start, end).unwrap()
    }

    fn generate_datetime(date_index: usize, time_index: usize) -> DateTime<Utc> {
        let date_index: i64 = date_index.try_into().unwrap();
        let time_index: u32 = time_index.try_into().unwrap();

        Utc::now()
            .add(Duration::days(date_index))
            .date()
            .and_hms(8 + time_index * 3, 0, 0)
    }
}
impl TodoRepository for MockTodoRepository {
    fn fetch_by_id(&self, id: &Id) -> Option<Todo> {
        self.content.iter().find(|t| t.id() == id).cloned()
    }

    fn fetch_by_task(&self, task_id: &Id) -> RepositoryResult<Vec<Todo>> {
        Ok(self.content.iter().filter(|t| t.task().id() == task_id).cloned().collect())
    }

    fn create(&mut self, todo: &Todo) -> RepositoryResult<()> {
        self.content.push(todo.to_owned());

        Ok(())
    }

    fn update(&mut self, todo: &Todo) -> RepositoryResult<Todo> {
        let replace_index = self.content
            .iter()
            .position(|t| t.id() == todo.id())
            .ok_or(RepositoryError::NotFound)?;

        self.content.remove(replace_index);
        self.content.push(todo.clone());

        Ok(todo.clone())
    }

    fn remove(&mut self, todo: Todo) -> RepositoryResult<()> {
        let replace_index = self.content
            .iter()
            .position(|t| t.id() == todo.id())
            .ok_or(RepositoryError::NotFound)?;

        self.content.remove(replace_index);
        Ok(())
    }
}
