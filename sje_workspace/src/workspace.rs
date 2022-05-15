use std::sync::{Arc, Mutex};

use crate::{document::DocumentId, project::Project, Document, DocumentInfo};

pub struct Observable<T: Send + Sync> {
    funcs: Vec<Arc<dyn Fn(Arc<Project<T>>) + Send + Sync + 'static>>,
}
impl<T: Send + Sync + 'static> Observable<T> {
    pub fn subscribe<TFunc: Fn(Arc<Project<T>>) + 'static + Send + Sync>(&mut self, func: TFunc) {
        self.funcs.push(Arc::new(func));
    }

    pub(crate) fn on_next(&self, content: Arc<Project<T>>) {
        for func in &self.funcs {
            func(content.clone());
        }
    }
}

pub struct Workspace<T: Send + Sync> {
    pub current_project: Arc<Project<T>>,
    observable: Arc<Mutex<Observable<T>>>,
}

impl<T: Sized + Send + Sync + 'static> Workspace<T> {
    pub fn new() -> Self {
        Self {
            current_project: Arc::new(Project::new()),
            observable: Arc::new(Mutex::new(Observable { funcs: Vec::new() })),
        }
    }

    pub fn observe_project(&mut self) -> Arc<Mutex<Observable<T>>> {
        self.observable.clone()
    }

    pub fn add_document(&mut self, document_info: &DocumentInfo<T>) -> DocumentId {
        let new_document = Document::from(document_info);
        let id = DocumentId::new();
        let new_documents = self
            .current_project
            .documents
            .update(id, Arc::new(new_document));
        let new_project = self.current_project.with_documents(new_documents);
        self.current_project = new_project;

        self.observable
            .lock()
            .unwrap()
            .on_next(self.current_project.clone());
        id
    }

    pub fn update_current_project<TFunc: Fn(Arc<T>) -> Arc<T>>(
        &mut self,
        id: &DocumentId,
        updater: TFunc,
    ) {
        if let Some(document) = self.current_project.try_get_document(id) {
            // カレントプロジェクトの更新
            let new_content = updater(document.content.clone());
            let new_document = document.with_content(new_content);
            let new_documts = self
                .current_project
                .documents
                .update(id.clone(), new_document);
            self.current_project = self.current_project.with_documents(new_documts);

            // 非同期に通知
            let project = self.current_project.clone();
            let observable = self.observable.clone();
            let _ = tokio::task::spawn(async move {
                for func in &observable.lock().unwrap().funcs {
                    func(project.clone());
                }
            });
        }
    }
}
