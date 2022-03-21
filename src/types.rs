pub struct WithId<T> {
    id: String,
    content: T
}
impl<T> WithId<T> {
    pub fn new(id: String, content: T) -> WithId<T> {
        WithId { id, content }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn content(&self) -> &T {
        &self.content
    }

    pub fn content_mut(&mut self) -> &mut T {
        &mut self.content
    }
}
