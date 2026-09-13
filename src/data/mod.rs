#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod data_access;
#[cfg(target_arch = "wasm32")]
pub(crate) mod local_storage;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) use data_access::*;
#[cfg(target_arch = "wasm32")]
pub(crate) use local_storage::*;
