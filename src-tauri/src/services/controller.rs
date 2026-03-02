use crate::services::processors::traits::Advancement;

pub struct Controller<T: Advancement> {
    pub processor: T,
}

impl<T> Controller<T>
where
    T:Advancement
{
    pub fn new(processor: T) -> Self {
        Self {
            processor,
        }
    }
}