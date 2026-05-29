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

    // Setup Tokio Runtime on a separate thread
    let rt = create_tokio_runtime();
    let _enter = rt.enter(); // This Guard must be held to call `tokio::spawn` anywhere in the program
    start_background_worker(rt); // This is also needed to prevent the runtime from stopping

    // Our application state:
    let mut name = "Arthur".to_owned();
    let mut age = 42;
    let mut data_state = main_loop_async::DataState::None;
    let mut seconds_required_to_load = 10;

    eframe::run_ui_native(
        "egui Example",
        eframe::NativeOptions::default(),
        move |ui, _frame| {
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

                // Data from the spawned task will show here after the user clicks
                ui.separator();
                ui.heading("Load Data (NB: Set to randomly fail)");
                if let Some(data) = data_state.egui_poll_mut(ui, None) {
                    ui.horizontal(|ui| {
                        ui.label("Editable Data from spawned task");
                        ui.text_edit_singleline(data);
                    });
                    if ui.button("Clear Data").clicked() {
                        data_state = main_loop_async::DataState::None;
                    }
                } else if data_state.is_none() {
                    ui.add(
                        egui::Slider::new(&mut seconds_required_to_load, 0..=30)
                            .text("Seconds needed to load data"),
                    );
                    if ui.button("Spawn task to load data").clicked() {
                        let can_make_progress =
                            data_state.egui_start_request(ui, || main_loop_async::spawn_with_return(move || load_data(seconds_required_to_load)));
                            assert!(
                                can_make_progress.is_able_to_make_progress(),
                                "checks that we don't have a logic error, this should always be able to make progress from this point"
                            );
                    }
                } else if data_state.is_awaiting_response() {
                    // Currently loading allowing the user to abort, might not make sense for your application
                    if ui.button("Cancel Loading").clicked(){
                        data_state = main_loop_async::DataState::None;
                    }
                }
            });
        },
    )
}

fn create_tokio_runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Unable to create Runtime")
}

fn start_background_worker(rt: tokio::runtime::Runtime) {
    // Execute the runtime in its own thread.
    std::thread::spawn(move || {
        rt.block_on(async {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
            }
        })
    });
}

async fn load_data(secs: u64) -> anyhow::Result<String> {
    if should_fail() {
        anyhow::bail!("there was a random problem loading the data, try again");
    }

    tokio::time::sleep_until(tokio::time::Instant::now() + std::time::Duration::from_secs(secs))
        .await;

    Ok(format!("loaded the data after {secs} seconds"))
}

fn should_fail() -> bool {
    rand::random()
}
