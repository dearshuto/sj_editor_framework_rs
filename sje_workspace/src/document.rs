use std::sync::Arc;

use sje_generator_macro::Immutable;
use uuid::Uuid;

use crate::DocumentInfo;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct DocumentId {
    id: Uuid,
}

impl DocumentId {
    pub fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }
}

#[derive(Immutable)]
pub struct Document<T> {
    pub content: Arc<T>,
}

impl<T> Document<T> {
    pub fn from(info: &DocumentInfo<T>) -> Self {
        Self {
            content: info.content.clone(),
        }
    }
}
