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

struct MyApp {
    input: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            input: "0".to_string(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Basic Calculator");

            ui.add(egui::TextEdit::singleline(&mut self.input)
                .font(egui::TextStyle::Monospace)
                .desired_width(f32::INFINITY));

            egui::Grid::new("calculator_grid")
                .spacing([10.0, 10.0])
                .show(ui, |ui| {
                    if ui.button("7").clicked() { self.input.push('7'); }
                    if ui.button("8").clicked() { self.input.push('8'); }
                    if ui.button("9").clicked() { self.input.push('9'); }
                    ui.end_row();
                    if ui.button("4").clicked() { self.input.push('4'); }
                    if ui.button("5").clicked() { self.input.push('5'); }
                    if ui.button("6").clicked() { self.input.push('6'); }
                    ui.end_row();
                    if ui.button("1").clicked() { self.input.push('1'); }
                    if ui.button("2").clicked() { self.input.push('2'); }
                    if ui.button("3").clicked() { self.input.push('3'); }
                    ui.end_row();
                    if ui.button("0").clicked() { self.input.push('0'); }
                    if ui.button(" ").clicked() { }
                    if ui.button(".").clicked() { self.input.push('.'); }
            });
        });
    }
}
