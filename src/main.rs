use eframe::egui;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Start the eframe application
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "REX", // Application title
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

#[derive(Default)]
struct MyApp {
    text: String, // A String to store the notepad content
    file_path: Option<PathBuf>, // Current file path
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::default().inner_margin(0.0))
            .show(ctx, |ui| {
                // Menu bar
                ui.horizontal(|ui| {
                    if ui.button("📄 New").clicked() {
                        self.text.clear();
                        self.file_path = None;
                    }
                    if ui.button("📁 Open").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("text", &["txt"])
                            .pick_file()
                        {
                            if let Ok(content) = fs::read_to_string(&path) {
                                self.text = content;
                                self.file_path = Some(path);
                            }
                        }
                    }
                    if ui.button("💾 Save").clicked() {
                        if let Some(path) = &self.file_path {
                            let _ = fs::write(path, &self.text);
                        } else {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("text", &["txt"])
                                .save_file()
                            {
                                let _ = fs::write(&path, &self.text);
                                self.file_path = Some(path);
                            }
                        }
                    }
                });
                
                ui.separator();

                // Text editor
                ui.add(
                    egui::TextEdit::multiline(&mut self.text)
                        .hint_text("Start typing...")
                        .desired_rows(20)
                        .desired_width(f32::INFINITY)
                );
            });
    }
}
