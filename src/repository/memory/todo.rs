use std::ops::Add;

use chrono::{Utc, Duration, DateTime};

use crate::entity::{Todo, Task, DateTimeRange, DoneStatus, DoneStatusList};
use crate::repository::{RepositoryResult, TodoRepository, RepositoryError};
use crate::types::Id;

struct MockTodoRepository {
    content: Vec<Todo>
}
impl MockTodoRepository {
    fn new() -> MockTodoRepository {
        let task: [Task; 5] = (0..4)
            .map(|i| { Task::new(Id::generate(), format!("task-{}", i)).unwrap() })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        MockTodoRepository {
            content: (0..100).map(|i| {
                Todo::new(
                    Id::generate(),
                    task[i % 5].to_owned(),
                    MockTodoRepository::generate_range(i / 3, i % 3),
                    DoneStatusList::new(
                        (0..3).map(|j| {
                            DoneStatus::new(
                                Id::generate(),
                                MockTodoRepository::generate_datetime(i, j),
                                false
                            )
                        }).collect()
                    )
                )
            }).collect()
        }
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
