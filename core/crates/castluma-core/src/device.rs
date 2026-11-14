#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportKind {
    GoogleCast, // Using the official Android SDK, which handles Cast discovery
    Dlna,       // Via SSDP, followed by reading the device and service descriptions
    CastLuma, // Mechanism to be designed; initially, we could also use a manually configured address
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Device {
    pub id: String,   // Device identifier; not necessarily its IP address
    pub name: String, // Name displayed to the user
    pub transports: Vec<TransportKind>, // Protocols available for that device
}

impl Device {
    pub fn supports_transport(&self, transport: TransportKind) -> bool {
        self.transports.contains(&transport)
    }
}
