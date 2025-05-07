use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let viewport = egui::ViewportBuilder::default()
        .with_inner_size([160.0, 300.0])
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
    decimal_flag: bool,
    reset_flag: bool,
    stored_number: f64,
    calculation_flag: char,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            input: "0".to_string(),
            stored_number: 0.0,
            decimal_flag: false,
            reset_flag: true,
            calculation_flag: ' ',
        }
    }
}

impl MyApp {
    fn calculator_button(&mut self, ui: &mut egui::Ui, label: &str) -> bool {
        let button = ui.add(egui::Button::new(
            egui::RichText::new(label)
                .size(24.0)
                .text_style(egui::TextStyle::Button)
        )
            .min_size(egui::vec2(35.0, 35.0))
            .rounding(egui::Rounding::same(2.5))
        );

        if button.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        if button.clicked() && self.reset_flag {
            if self.input == "0".to_string() && label == "0" {

            } else {
                self.input = "".to_string();
                self.input.push_str(label);
                self.reset_flag = false;
                return true;
            }
        } else if button.clicked() {
            self.input.push_str(label);
        }
        button.clicked()
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Basic Calculator");

            ui.add(egui::TextEdit::singleline(&mut self.input)
                .min_size(egui::vec2(50.0, 35.0))
                .font(egui::FontId::new(36.0, egui::FontFamily::Monospace))
            );

            egui::Grid::new("calculator_grid")
                .spacing([5.0, 5.0])
                .show(ui, |ui| {
                    self.calculator_button(ui, "7");
                    self.calculator_button(ui, "8");
                    self.calculator_button(ui, "9");

                    if ui.add(egui::Button::new(
                        egui::RichText::new("C")
                            .size(24.0)
                    )
                        .min_size(egui::vec2(35.0, 35.0))
                        .rounding(egui::Rounding::same(2.5))
                        .fill(egui::Color32::from_rgb(80, 30, 30))
                    ).clicked() {
                        self.input = String::from("0"); 
                        self.decimal_flag = false;
                        self.stored_number = 0.0;
                        self.reset_flag = true;
                    }
                    ui.end_row();
                    self.calculator_button(ui, "4");
                    self.calculator_button(ui, "5");
                    self.calculator_button(ui, "6");
                    if ui.add(egui::Button::new(
                        egui::RichText::new("+")
                            .size(24.0)
                    )
                        .min_size(egui::vec2(35.0, 35.0))
                        .rounding(egui::Rounding::same(2.5))
                    ).clicked() {
                        self.calculation_flag = '+';
                        match self.input.parse() {
                            Ok(num) => {
                                self.stored_number = num;
                            },
                            Err(e) => println!("Error parsing: {}", e),
                        }

                        self.input = 0.to_string();
                        self.reset_flag = true;
                        self.decimal_flag = false;
                    }
                    ui.end_row();
                    self.calculator_button(ui, "1");
                    self.calculator_button(ui, "2");
                    self.calculator_button(ui, "3");
                    if ui.add(egui::Button::new(
                        egui::RichText::new("-")
                            .size(24.0)
                    )
                        .min_size(egui::vec2(35.0, 35.0))
                        .rounding(egui::Rounding::same(2.5))
                    ).clicked() {
                        self.calculation_flag = '-';
                        self.stored_number = self.input.parse::<f64>().unwrap();
                        self.input = 0.to_string();
                        self.reset_flag = true;
                        self.decimal_flag = false;
                    }
                    ui.end_row();
                    self.calculator_button(ui, "0");
                    if ui.add(egui::Button::new(
                        egui::RichText::new(" ")
                            .size(24.0)
                    )
                        .min_size(egui::vec2(35.0, 35.0))
                        .rounding(egui::Rounding::same(2.5))
                    ).clicked() { }
                    if ui.add(egui::Button::new(
                        egui::RichText::new(".")
                            .size(24.0)
                    )
                        .min_size(egui::vec2(35.0, 35.0))
                        .rounding(egui::Rounding::same(2.5))
                    ).clicked() {
                        if self.decimal_flag {
                            return;
                        }
                        self.input.push('.');
                        self.decimal_flag = true;
                        self.reset_flag = false;
                    }
                    if ui.add(egui::Button::new(
                        egui::RichText::new("=")
                            .size(24.0)
                    )
                        .min_size(egui::vec2(35.0, 35.0))
                        .rounding(egui::Rounding::same(2.5))
                    ).clicked() {
                        let input_num = self.input.parse::<f64>().unwrap();
                        let mut result: f64 = 0.0;
                        match self.calculation_flag {
                            '+' => result += input_num + self.stored_number,
                            '-' => result += self.stored_number - input_num,
                            _ => result = input_num,
                        }

                        self.input = result.to_string();
                        self.calculation_flag = ' ';
                        self.reset_flag = true;
                        self.decimal_flag = false;
                    }
            });
        });
    }
}
