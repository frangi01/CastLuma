use castluma_ffi::library_version;

#[test]
fn library_version_matches_package_version() {
    assert_eq!(library_version(), env!("CARGO_PKG_VERSION"));
}
