#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Id {
    content: String
}
impl Id {
    pub fn new(content: impl Into<String>) -> Id {
        Id { content: content.into() }
    }
    pub fn generate() -> Id {
        Id {
            content: uuid::Uuid::new_v4().to_hyphenated().to_string()
        }
    }

    pub fn get(&self) -> &str {
        &self.content
    }
}
