/// An async function to spawn
pub trait Spawnable:
    'static + Send + std::future::Future<Output: Send + 'static + std::fmt::Debug>
{
}

impl<T: 'static + Send + std::future::Future<Output: Send + 'static + std::fmt::Debug>> Spawnable
    for T
{
}

/// An async func that accepts a generic argument
/// and returns a generic value
pub trait SpawnableWithReturn<Out: Spawnable>: 'static + Send + FnOnce() -> Out {}

impl<Out: Spawnable, T: 'static + Send + FnOnce() -> Out> SpawnableWithReturn<Out> for T {}
