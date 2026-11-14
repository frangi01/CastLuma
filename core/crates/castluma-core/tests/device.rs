use castluma_core::{Device, TransportKind};

#[test]
fn different_transport_kind_are_not_equal() {
    assert_ne!(TransportKind::GoogleCast, TransportKind::Dlna)
}

#[test]
fn device_can_have_multiple_transports() {
    let id = String::from("test-device-1");
    let name = String::from("Living room TV");
    let transports = vec![TransportKind::GoogleCast, TransportKind::Dlna];
    let device = Device {
        id,
        name,
        transports,
    };

    assert!(device.supports_transport(TransportKind::GoogleCast));
    assert!(device.supports_transport(TransportKind::Dlna));
    assert!(!device.supports_transport(TransportKind::CastLuma));
}

#[test]
fn device_without_transports_supports_none() {
    let device = Device {
        id: String::from("unknown-device"),
        name: String::from("Unknown TV"),
        transports: Vec::new(),
    };
    assert!(!device.supports_transport(TransportKind::GoogleCast));
    assert!(!device.supports_transport(TransportKind::Dlna));
    assert!(!device.supports_transport(TransportKind::CastLuma));
}
