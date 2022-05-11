use std::sync::Arc;

use sje_generator_macro::Immutable;

use crate::{document::DocumentId, Document};

#[derive(Immutable)]
pub struct Project<T> {
    pub documents: im::HashMap<DocumentId, Arc<Document<T>>>,
}

impl<T> Project<T> {
    pub fn new() -> Self {
        Self {
            documents: im::HashMap::new(),
        }
    }

    pub fn try_get_document(&self, id: &DocumentId) -> Option<&Arc<Document<T>>> {
        self.documents.get(id)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn new() {
        let hash_map = im::HashMap::<i32, i32>::new();
        let new_hash_map = hash_map.update(1, 2);
        assert!(hash_map.is_empty());
        assert_eq!(new_hash_map.len(), 1);
    }
}
