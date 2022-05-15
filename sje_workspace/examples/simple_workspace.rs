use sje_generator_macro::Immutable;
use sje_workspace::{DocumentInfo, Workspace};
use std::{sync::Arc, time::Duration};

use eframe::egui;

// モデルデータ
#[derive(Immutable)]
struct TestData {
    pub value: i32,
}

struct MyApp {
    workspace: Workspace<TestData>,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut workspace = Workspace::<TestData>::new();
        let edit_model = Arc::new(TestData { value: 5 });
        let document_info = DocumentInfo {
            content: edit_model,
        };
        let _id = workspace.add_document(&document_info);

        workspace.observe_project().lock().unwrap().subscribe(|_| {
            std::thread::sleep(Duration::from_secs(1));
            println!("Notify");
        });

        Self { workspace }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");

            // スライダーでモデルを編集
            let current_project = self.workspace.current_project.clone();
            for (id, document) in &current_project.documents {
                let mut value = document.content.value;
                ui.add(egui::Slider::new(&mut value, 0..=10).text("Value"));

                if document.content.value != value {
                    self.workspace
                        .update_current_project(&id, |x| x.with_value(value));
                }
            }

            // 編集結果をラベルに表示
            for (_id, document) in &current_project.documents {
                ui.label(format!("Hello World: {}", document.content.value));
            }
        });
    }
}

#[tokio::main]
async fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}
