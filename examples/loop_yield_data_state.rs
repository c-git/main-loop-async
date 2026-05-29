// Native and WASM require different main functions but after that it should be
// the same. This example demonstrates how this crate can be used with the
// DataState type.

use main_loop_async::{DataState, spawn_with_return};

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

async fn doubled(input: i32) -> Result<String, &'static str> {
    Ok((input * 2).to_string())
}

async fn common_code() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = DataState::None;

    println!("Starting loop");

    // This loop would normally be a game loop, or the executor of an immediate mode
    // GUI.
    loop {
        if state.is_none() {
            let can_make_progress = state.start_task(|| spawn_with_return(|| doubled(10)));
            assert!(can_make_progress.is_able_to_make_progress());
        }
        if let Some(task_result) = state.poll().present() {
            println!("Response received");
            assert_eq!(task_result, "20");
            break;
        }
        main_loop_async::yield_now().await;
    }
    println!("Exited loop");
    Ok(())
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {

    #[tokio::test]
    async fn test_name() {
        super::common_code().await.unwrap();
    }
}
