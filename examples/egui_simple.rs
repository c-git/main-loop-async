//! Example of usage with egui. Base example copied from
//! <https://github.com/emilk/egui/blob/main/examples/hello_world_simple/src/main.rs>
//!
//! NB: This one is not run as part of the test since it's interactive. Run by
//! using the following command:
//!
//! ```rust
//! cargo run --example egui_simple --features="egui"
//! ```
//!
//! This example showcases multiple use ways to use this library. See the impl
//! for the [`Examples`] struct for docs on each.

#[cfg(all(
    not(target_arch = "wasm32"),
    feature = "native-tokio",
    feature = "egui"
))]
pub use eg_mod::main;

#[cfg(all(
    not(target_arch = "wasm32"),
    feature = "native-tokio",
    feature = "egui"
))]
mod eg_mod {
    use main_loop_async::{DataState, DataStateRetry, spawn_with_return};
    use std::sync::{
        Arc,
        atomic::{AtomicU8, Ordering},
    };

    pub fn main() -> eframe::Result {
        use eframe::egui;

        // Setup Tokio Runtime on a separate thread
        let rt = Helper::create_tokio_runtime();
        let _enter = rt.enter(); // This Guard must be held to call `tokio::spawn` anywhere in the program
        Helper::start_background_worker(rt); // This is also needed to prevent the runtime from stopping

        // Our application state:
        let mut name = "Arthur".to_owned();
        let mut age = 42;
        let mut data_state = DataState::None;
        // This is more intended for use with things that load automatically, if you do
        // not intend to use it this way you will need a separate variable to track if
        // it should be attempting to load
        let mut data_state_retry = DataStateRetry::new(3, 5000..10_000);
        let mut seconds_required_to_load = 5;
        let atomic_load_count = Arc::new(AtomicU8::new(0));
        let mut name_loader = DataState::None;

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
                    Helper::controls_separation(ui);
                    Examples::example_load_keep(
                        ui,
                        &mut data_state,
                        &mut seconds_required_to_load,
                        &atomic_load_count,
                    );

                    // Alternate version to show data with automatic retry and automatically starts
                    // trying to load
                    Helper::controls_separation(ui);
                    Examples::example_retry(
                        ui,
                        &mut data_state_retry,
                        seconds_required_to_load,
                        &atomic_load_count,
                    );

                    // Note this version has the poll where we can set the other variable
                    Helper::controls_separation(ui);
                    Examples::example_load_and_take(
                        ui,
                        &mut name_loader,
                        &mut seconds_required_to_load,
                        &atomic_load_count,
                    );
                    if let Some(new_name) = name_loader.egui_poll_take(ui, None) {
                        name = new_name;
                    }
                });
            },
        )
    }

    struct Examples;
    impl Examples {
        /// Example that shows loading some data that stays in the [`DataState`]
        ///
        /// # Example use cases
        ///
        /// - The user initiates the load and the data to use at that location
        fn example_load_keep(
            ui: &mut egui::Ui,
            data_state: &mut DataState<String>,
            seconds_required_to_load: &mut u64,
            atomic_load_count: &Arc<AtomicU8>,
        ) {
            ui.heading(format!(
                "Load Data Without Retry (NB: Load randomly fails, {} attempts so far)",
                atomic_load_count.load(Ordering::Relaxed)
            ));
            if let Some(data) = data_state.egui_poll_mut(ui, None) {
                ui.horizontal(|ui| {
                    ui.label("Editable Data from spawned task");
                    ui.text_edit_singleline(data);
                });
                if ui.button("Clear Data").clicked() {
                    *data_state = DataState::None;
                }
            } else if data_state.is_none() {
                ui.add(
                    egui::Slider::new(seconds_required_to_load, 0..=30)
                        .text("Seconds needed to load data"),
                );
                if ui.button("Spawn task to load data").clicked() {
                    let secs = *seconds_required_to_load;
                    let atomic_load_count = Arc::clone(atomic_load_count);
                    let can_make_progress = data_state.egui_start_task(ui, || {
                        spawn_with_return(move || Helper::load_data(secs, atomic_load_count))
                    });
                    assert!(
                        can_make_progress.is_able_to_make_progress(),
                        "checks that we don't have a logic error, this should always be able to make progress from this point"
                    );
                }
            } else if data_state.is_awaiting_response() {
                // Currently loading allowing the user to abort, might not make sense for your
                // application
                if ui.button("Cancel Loading").clicked() {
                    *data_state = DataState::None;
                }
            }
        }

        /// Example that shows loading some data that stays in the
        /// [`DataState`] with automatic retry added.
        ///
        /// # Example use cases
        ///
        /// - The data auto loaded and not initiated by the user
        fn example_retry(
            ui: &mut egui::Ui,
            data_state: &mut DataStateRetry<String>,
            secs: u64,
            atomic_load_count: &Arc<AtomicU8>,
        ) {
            ui.heading(format!(
                "Load Data With Retry (NB: Load randomly fails, {} attempts so far)",
                atomic_load_count.load(Ordering::Relaxed)
            ));
            if let Some(data) = data_state.present_mut() {
                ui.horizontal(|ui| {
                    ui.label("Editable Data from spawned task");
                    ui.text_edit_singleline(data);
                });
                if ui.button("Clear Data").clicked() {
                    data_state.clear();
                }
            } else {
                let atomic_load_count = Arc::clone(atomic_load_count);
                let can_make_progress = data_state.egui_start_or_poll(ui, None, || {
                    spawn_with_return(move || Helper::load_data(secs, atomic_load_count))
                });
                assert!(
                    can_make_progress.is_able_to_make_progress(),
                    "checks that we don't have a logic error, this should always be able to make progress from this point"
                );
            }
            if data_state.is_awaiting_response() {
                // Currently loading allowing the user to abort, might not make sense for your
                // application
                if ui.button("Cancel Loading").clicked() {
                    data_state.clear();
                }
            }
        }

        /// Example that shows loading some data that is meant to be passed on
        /// and used somewhere else
        ///
        /// # Example use cases
        ///
        /// - You already have an application and want to just move the loading
        ///   part off the main thread
        /// - You need to pass an owned value off to another API and you don't
        ///   need it after it's been loaded
        fn example_load_and_take(
            ui: &mut egui::Ui,
            data_state: &mut DataState<String>,
            seconds_required_to_load: &mut u64,
            atomic_load_count: &Arc<AtomicU8>,
        ) {
            ui.heading(format!(
                "Load value for Name (NB: Load randomly fails, {} attempts so far)",
                atomic_load_count.load(Ordering::Relaxed)
            ));
            if data_state.is_none() {
                ui.add(
                    egui::Slider::new(seconds_required_to_load, 0..=30)
                        .text("Seconds needed to load data"),
                );
                ui.horizontal(|ui|{
                    if ui.button("Spawn task to load data").clicked() {
                        let secs = *seconds_required_to_load;
                        let atomic_load_count = Arc::clone(atomic_load_count);
                        let can_make_progress = data_state.egui_start_task(ui, || {
                            spawn_with_return(move || Helper::load_data(secs, atomic_load_count))
                        });
                        assert!(
                            can_make_progress.is_able_to_make_progress(),
                            "checks that we don't have a logic error, this should always be able to make progress from this point"
                        );
                    }
                    ui.label("NB: See name for output of load");
                });
            } else if data_state.is_awaiting_response() {
                // Currently loading allowing the user to abort, might not make sense for your
                // application
                if ui.button("Cancel Loading").clicked() {
                    *data_state = DataState::None;
                }
            }
        }
    }

    struct Helper;
    impl Helper {
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

        async fn load_data(secs: u64, atomic_load_count: Arc<AtomicU8>) -> anyhow::Result<String> {
            atomic_load_count.fetch_add(1, Ordering::Relaxed);
            if Self::should_fail() {
                anyhow::bail!("there was a random problem loading the data, try again");
            }

            tokio::time::sleep_until(
                tokio::time::Instant::now() + std::time::Duration::from_secs(secs),
            )
            .await;

            Ok(format!("loaded the data after {secs} seconds"))
        }

        fn should_fail() -> bool {
            rand::random()
        }

        fn controls_separation(ui: &mut egui::Ui) {
            ui.add_space(40.0);
            ui.separator();
        }
    }
}

#[cfg(not(all(
    not(target_arch = "wasm32"),
    feature = "native-tokio",
    feature = "egui"
)))]
fn main() {}
