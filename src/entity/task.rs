use crate::types::Id;

pub struct Task {
    id: Id,
    name: String
}
#[derive(PartialEq, Debug)]
pub enum TaskValidationError {
    EmptyName
}
impl Task {
    pub fn new(id: Id, name: &str) -> Result<Self, TaskValidationError> {
        Task::validate_name(name)?;

        Ok(Self { id, name: name.to_owned() })
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: &str) -> Result<(), TaskValidationError> {
        Task::validate_name(name)?;
        self.name = name.to_owned();
        Ok(())
    }

    fn validate_name(name: &str) -> Result<(), TaskValidationError> {
        if name.is_empty() {
            return Err(TaskValidationError::EmptyName)
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Task, TaskValidationError};
    use rstest::rstest;

    #[rstest(name, expected,
        case("task", None),
        case("", Some(TaskValidationError::EmptyName))
    )]
    fn initializing_empty_name_not_allowed(name: &str, expected: Option<TaskValidationError>) {
        let maybe_task = Task::new("task-id-1".to_owned(), name);

        assert_eq!(maybe_task.err(), expected);
    }

    #[rstest(name, expected,
        case("task", None),
        case("", Some(TaskValidationError::EmptyName))
    )]
    fn setting_empty_name_not_allowed(name: &str, expected: Option<TaskValidationError>) {
        let mut task = Task::new("task-id-1".to_owned(), "Ok").unwrap();

        assert_eq!(task.set_name(name).err(), expected);
    }
}
