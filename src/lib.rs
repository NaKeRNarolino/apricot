use eframe::egui;

fn main() -> eframe::Result {
        
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
                Ok(Box::<App>::default())
        }),
    )
}

struct App {
}

impl Default for App {
    fn default() -> Self {
        Self { }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
         egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Heading!");
         });
    }
}
