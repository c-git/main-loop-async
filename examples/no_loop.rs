// Native and WASM require different main functions but after that it should be
// the same. This example shows how to do a simple fetch.

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

#[expect(dead_code)]
async fn some_task(value: i32) -> i32 {
    value * 2
}

async fn common_code() -> Result<(), Box<dyn std::error::Error>> {
    // spawn_with_callback(move || some_task(5).await
    //     request,
    //     move |result: Result<reqwest::Response, reqwest::Error>| async {
    //         tx.send(result.expect("Expecting Response not Error").status())
    //             .expect("Receiver should still be available");
    //     },
    // );

    // // Note the next call block this execution path (task / thread) see loop
    // // examples for alternatives
    // let status = rx.await?;
    // assert_eq!(status, 200);
    // Ok(())
    todo!()
}
