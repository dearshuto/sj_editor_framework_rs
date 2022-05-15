use std::sync::Arc;

use sje_generator_macro::Immutable;

#[derive(Immutable)]
pub struct DocumentInfo<T> {
    pub content: Arc<T>,
}
