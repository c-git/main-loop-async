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
