#![expect(clippy::print_stdout, clippy::print_stderr)]

// Native and WASM require different main functions but after that it should be
// the same. Uses yield but yield isn't available yet for wasm_bindgen_futures
// so uses a workaround found. If you're building a bigger application or have
// multiple places you need to make requests look at the
// loop_yield_data_state.rs example instead.

use main_loop_async::spawn_with_return;

#[cfg(all(not(target_arch = "wasm32"), feature = "native-tokio"))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    common_code().await
}

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
#[cfg(target_arch = "wasm32")]
fn main() {
    #[wasm_bindgen_test::wasm_bindgen_test]
    async fn do_fetch() -> Result<(), Box<dyn std::error::Error>> {
        common_code().await
    }
}

#[expect(
    clippy::unused_async,
    reason = "for demonstration purposes of the example"
)]
async fn doubled(input: i32) -> String {
    println!("Task run");
    (input * 2).to_string()
}

enum State {
    Startup,
    AwaitingResponse(futures::channel::oneshot::Receiver<String>),
    Done,
}

async fn common_code() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = State::Startup;

    println!("Starting loop");

    // This loop would normally be a game loop, or the executor of an immediate mode
    // GUI.
    loop {
        match state {
            State::Startup => {
                // Send request
                let rx = spawn_with_return(|| doubled(5));
                println!("Task spawned");
                state = State::AwaitingResponse(rx);
            }
            State::AwaitingResponse(mut rx) => {
                // Check if response is ready
                match rx.try_recv() {
                    Ok(option) => {
                        if let Some(task_result) = option {
                            println!("Response received");
                            assert_eq!(task_result, "10", "response should be 5 * 2 as a String");
                            state = State::Done;
                        } else {
                            // Still waiting
                            state = State::AwaitingResponse(rx);
                            main_loop_async::yield_now().await;
                        }
                    }
                    Err(e) => {
                        eprintln!("Canceled");
                        return Err(Box::new(e));
                    }
                }
            }
            State::Done => {
                // All done exit now
                println!("Completed exiting");
                return Ok(());
            }
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]

mod tests {

    #[tokio::test]
    async fn test_name() {
        super::common_code().await.unwrap();
    }
}
