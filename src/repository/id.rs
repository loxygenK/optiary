use crate::types::Id;

pub struct IdSuppier;
impl IdSuppier{
    fn next(&self) -> Id {
        uuid::Uuid::new_v4().to_hyphenated().to_string()
    }
}
