use calculator::*;
use eframe::egui;
mod calculator;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Calculator",
        options,
        Box::new(|cc| Box::new(Calculator::new(cc))),
    )
}

#[derive(Default)]
struct Calculator {
    operation: i8,
    first_input: String,
    second_input: String,
    third_input: String,
    fourth_input: String,
    result: f64,
    mode: i8,
    is_calculating: bool,
}

impl Calculator {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for Calculator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.mode == 0 {
            // Four Operations
            egui::SidePanel::left("Left panel").show(ctx, |ui| {
                ui.heading("Input n° 1");
                ui.text_edit_singleline(&mut self.first_input);
                ui.spacing();
                ui.heading("Input n° 2");
                ui.text_edit_singleline(&mut self.second_input);
            });

            egui::TopBottomPanel::top("Top panel").show(ctx, |ui| {
                if ui.button("Plus").clicked() {
                    self.operation = 0;
                    self.result = calculate_operations(
                        parse_text(&self.first_input),
                        parse_text(&self.second_input),
                        self.operation,
                    );
                }
                if ui.button("Minus").clicked() {
                    self.operation = 1;
                    self.result = calculate_operations(
                        parse_text(&self.first_input),
                        parse_text(&self.second_input),
                        self.operation,
                    );
                }
                if ui.button("Multiply").clicked() {
                    self.operation = 2;
                    self.result = calculate_operations(
                        parse_text(&self.first_input),
                        parse_text(&self.second_input),
                        self.operation,
                    );
                }
                if ui.button("Divide").clicked() {
                    self.operation = 3;
                    self.result = calculate_operations(
                        parse_text(&self.first_input),
                        parse_text(&self.second_input),
                        self.operation,
                    );
                }
            });
        }
        if self.mode == 1 {
            egui::SidePanel::left("Left panel").show(ctx, |ui| {
                ui.heading("Entry");
                ui.text_edit_singleline(&mut self.first_input);
            });
            egui::TopBottomPanel::top("Top panel").show(ctx, |ui| {
                if !self.is_calculating {
                    if ui.button("Start Calculation").clicked() {
                        self.result = sqrt(parse_text(&self.first_input));
                    }
                }
            });
        }
        if self.mode == 2 {
            egui::SidePanel::left("Left panel").show(ctx, |ui| {
                ui.heading("Target value");
                ui.text_edit_singleline(&mut self.first_input);
                ui.spacing();
                ui.heading("Power");
                ui.text_edit_singleline(&mut self.second_input);
                ui.spacing();
                ui.heading("Increment");
                ui.text_edit_singleline(&mut self.third_input);
                ui.spacing();
                ui.heading("Start value");
                ui.text_edit_singleline(&mut self.fourth_input);
            });

            egui::TopBottomPanel::top("Top panel").show(ctx, |ui| {
                if !self.is_calculating {
                    if ui.button("Start Calculation").clicked() {
                        self.result = get_root(
                            parse_text(&self.first_input),
                            parse_text(&self.second_input),
                            parse_text(&self.third_input),
                            parse_text(&self.fourth_input),
                        );
                    }
                }
            });
        }

        egui::SidePanel::right("Right panel").show(ctx, |ui| {
            ui.heading("Result: ");
            ui.heading(self.result.to_string());
        });

        egui::TopBottomPanel::bottom("Bottom panel").show(ctx, |ui| {
            ui.heading("IMPORTANT: use . and not ,");
            ui.heading("Example: Use 3.2, and not 3,2");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Four Operations").clicked() {
                self.mode = 0;
            }

            if ui.button("Get Square Root").clicked() {
                self.mode = 1;
            }

            if ui.button("Get All The Roots").clicked() {
                self.mode = 2;
            }
        });
    }
}
