pub struct Task {
    name: String
}
#[derive(Debug)]
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
