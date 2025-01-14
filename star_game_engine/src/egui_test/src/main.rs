use eframe::egui;
use egui::{FontFamily, FontId};
use std;
use std::io::Write;
use std::time::SystemTime;
use std::process;
use  std::fs;
use  crate::lua;

pub fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Star Engine",
        options,
        Box::new(|_cc| Ok(Box::new(TemplateApp::new()))),
    )
}

struct TemplateApp {
    start_time: SystemTime,
    show_heading: bool,
    code: String,
    name: String,
    output: String,
}

impl TemplateApp {
    fn new() -> Self {
        Self {
            start_time: SystemTime::now(),
            show_heading: true,
            code: String::new(),
            name: String::new(),
            output: String::new(),
        }
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Ok(elapsed) = self.start_time.elapsed() {
            if elapsed.as_secs() >= 2 {
                self.show_heading = false;
            }
        }

        if ctx.input(|i| i.viewport().close_requested()) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            process::exit(0);
        }
        
        let dark_gray = egui::Color32::from_rgb(52, 52, 52);

        egui::CentralPanel::default().frame(egui::Frame::default().fill(dark_gray)).show(ctx, |ui| {
            if self.show_heading {
                ui.centered_and_justified(|ui| {
                    ui.heading(egui::RichText::new("Star Engine\nLoading...").color(egui::Color32::from_rgb(255, 255, 0)).size( 30.0).font(FontId::new(32.0, FontFamily::Proportional)))
                });
            }
            
            if self.show_heading == false {
                ui.vertical_centered(|ui| {
                    ui.label("name");
                    ui.text_edit_singleline(&mut self.name);
                    ui.label("Enter Lua code:");
                    ui.text_edit_multiline(&mut self.code);
                    
                    if ui.button("Create lua file").clicked() {
                        let mut file = std::fs::File::create(format!("C:\\Users\\richa\\star_game_engine\\Scripts\\{}.lua", self.name)).expect("Failed to create file");
                        file.write_all(self.code.as_bytes()).expect("Failed to write");
                    }
                    if ui.button("Run").clicked() {
                        match lua::run_lua_file(format!("C:\\Users\\richa\\star_game_engine\\Scripts\\{}.lua", self.name)) {
                            Ok(output) => self.output = output,
                            Err(e) => self.output = format!("Error: {}", e),
                        }
                    }
                    // After your text editor
                    ui.separator();
                    ui.label("Output:");
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.label(egui::RichText::new(&self.output).color(egui::Color32::WHITE));
                    });
                });
                ui.horizontal(|ui| {
                    ui.selectable_value(current_value, selected_value, text)
                });
            }
        });
    }
}
