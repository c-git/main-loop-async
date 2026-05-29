//! Example of usage with egui. Base example copied from
//! <https://github.com/emilk/egui/blob/main/examples/hello_world_simple/src/main.rs>
//!
//! NB: This one is not run as part of the test since it's interactive. Run by
//! using the following command:
//!
//! ```rust
//! cargo run --example egui_simple --features="egui"
//! ```

fn main() -> eframe::Result {
    use eframe::egui;

    let options = eframe::NativeOptions::default();

    // Our application state:
    let mut name = "Arthur".to_owned();
    let mut age = 42;

    eframe::run_ui_native("egui Example", options, move |ui, _frame| {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("egui Example");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                age += 1;
            }
            ui.label(format!("Hello '{name}', age {age}"));
        });
    })
}
