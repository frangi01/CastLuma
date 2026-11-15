//! Mobile-facing bindings for CastLuma.

uniffi::setup_scaffolding!();

/// Return the version of the FFI package
#[uniffi::export]
pub fn library_version() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}
