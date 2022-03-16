pub struct Task {
    name: String
}
#[derive(PartialEq, Debug)]
pub enum TaskValidationError {
    EmptyName
}
impl Task {
    pub fn new(name: &str) -> Result<Task, TaskValidationError> {
        if name.is_empty() {
            return Err(TaskValidationError::EmptyName)
        }
        Ok(Task { name: name.to_owned() })
    }

    pub fn name(&self) -> &str {
        &self.name
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
    fn empty_name_not_allowed(name: &str, expected: Option<TaskValidationError>) {
        let maybe_task = Task::new(name);

        assert_eq!(maybe_task.err(), expected);
    }
}
