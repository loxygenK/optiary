pub struct Task {
    name: String
}
impl Task {
    pub fn new(name: &str) -> Task {
        Task { name: name.to_owned() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
