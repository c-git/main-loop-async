#![expect(clippy::print_stdout)]

// This example demonstrates how this crate can be used with the
// DataState type without async (Native only)

fn doubled(input: i32) -> String {
    (input * 2).to_string()
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let mut state: main_loop_async::DataState<String, &'static str> =
        main_loop_async::DataState::None;

    println!("Starting loop");

    // This loop would normally be a game loop, or the executor of an immediate mode
    // GUI.
    loop {
        if state.is_none() {
            let can_make_progress =
                state.start_task(|| main_loop_async::spawn_thread_with_return(|| Ok(doubled(10))));
            assert!(
                can_make_progress.is_able_to_make_progress(),
                "checks that we don't have a logic error, this should always be able to make progress from this point"
            );
        }
        if let Some(task_result) = state.poll().present() {
            println!("Response received");
            assert_eq!(task_result, "20", "response should be 10 * 2 as a String");
            break;
        }
    }
    println!("Exited loop");
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // Do nothing this doesn't work on wasm
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {

    #[test]
    fn test_name() {
        super::main();
    }
}
