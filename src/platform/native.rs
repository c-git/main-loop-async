//! Stores the code specific to native compilations
// but the comments are for both because these are the ones that show on docs.rs

#[cfg(not(feature = "native-tokio"))]
compile_error!(
    "Must chose a native runtime by enabling a feature flag. Right now only tokio is supported. If you have a different runtime that you want please create an issue on github."
);

/// Spawns a future on the underlying runtime in a cross platform way (NB: the
/// Send bound is removed in WASM)
#[cfg(feature = "native-tokio")]
pub fn spawn<F: crate::SpawnableNoReturn>(future: F) {
    tokio::spawn(future);
}

/// Spawns a thead to run a sync job returns a
/// [`futures::channel::oneshot::Receiver`] which can be used to get the return
/// value from `f`.
///
/// See the examples [folder](https://github.com/c-git/reqwest-cross/tree/main/examples)
/// for more complete examples.
///
/// # Example
/// ```rust
/// # use main_loop_async::spawn_thread_with_return;
///
/// # #[cfg(all(not(target_arch = "wasm32"),feature = "native-tokio"))]
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
///  let task = async || { "hi".to_owned() };
///  let rx = spawn_with_return(task);
///
///  let result = rx.await?; //Only an example, in a real use case use try_recv instead
///  assert_eq!(result, "hi");
/// # Ok(())
/// # }
///
/// # #[cfg(target_arch = "wasm32")]
/// # fn main(){}
/// ```
pub fn spawn_thread_with_return<F, T>(f: F) -> futures::channel::oneshot::Receiver<T>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static + std::fmt::Debug,
{
    let (tx, rx) = futures::channel::oneshot::channel();
    std::thread::spawn(move || {
        let task_result = f();
        let result = tx.send(task_result);
        if let Err(err_msg) = result {
            tracing::error!("failed to send result from `spawn_thread_with_return`: {err_msg:?}");
        }
    });
    rx
}
