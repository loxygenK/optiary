use crate::{repository::{TaskRepository, RepositoryError, RepositoryResult}, entity::Task, types::Id};

pub struct MockTaskRepository {
    content: Vec<Task>
}
impl MockTaskRepository {
    pub fn new() -> Self {
        MockTaskRepository {
            content: (0..9)
                .map(|i| { Task::new(Id::new(format!("task-{}", i)), format!("Task #{}", i)).unwrap() })
                .collect()
        }
    }
}
impl TaskRepository for MockTaskRepository {
    fn fetch_by_id(&self, id: &Id) -> Option<Task> {
        self.content.iter().find(|t| t.id() == id).cloned()
    }

    fn create(&mut self, todo: &Task) -> RepositoryResult<()> {
        self.content.push(todo.to_owned());

        Ok(())
    }

    fn update(&mut self, task: &Task) -> RepositoryResult<Task> {
        let replace_index = self.content
            .iter()
            .position(|t| t.id() == task.id())
            .ok_or(RepositoryError::NotFound)?;

        self.content.remove(replace_index);
        self.content.push(task.to_owned());

        Ok(task.to_owned())
    }

    fn remove(&mut self, task: Task) -> RepositoryResult<()> {
        let replace_index = self.content
            .iter()
            .position(|t| t.id() == task.id())
            .ok_or(RepositoryError::NotFound)?;

        self.content.remove(replace_index);

        Ok(())
    }
}
