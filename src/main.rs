use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let viewport = egui::ViewportBuilder::default()
        .with_inner_size([400.0, 300.0])
        .with_title("Calculator");
    
    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    
    eframe::run_native(
        "RustEguiCalc",
        options,
        Box::new(|_cc| Box::new(MyApp::default()))
    )
}

struct MyApp {}

impl Default for MyApp {
    fn default() -> Self {
        Self {}
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Basic Calculator");
            
            // Draw a box using a frame
            egui::Frame::canvas(ui.style())
                .stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 200, 100)))
                .fill(egui::Color32::from_rgb(50, 100, 50))
                .inner_margin(egui::Margin::same(20.0))
                .show(ui, |ui| {
                    ui.label("This is a box!");
                });
        });
    }
}
