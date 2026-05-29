//! This example shows how to use `main_loop_async::spawn_thread_with_return`
//! which only works on native

fn double(value: i32) -> i32 {
    value * 2
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let mut rx = main_loop_async::spawn_thread_with_return(|| double(5));

    // Note the next call blocks this execution path (task / thread) see loop
    // examples for alternatives
    let task_result = loop {
        match rx.try_recv() {
            Ok(x) => {
                if let Some(x) = x {
                    break x;
                } else {
                    // No value yet, let's go around the loop again
                }
            }
            Err(err_msg) => panic!("sender was dropped, no value received: {err_msg}"),
        }
    };
    assert_eq!(
        task_result, 10,
        "5 sent in and it should be doubled to be 10"
    );
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // Do nothing, this doesn't work on wasm
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {

    #[test]
    fn test_name() {
        super::main();
    }
}
