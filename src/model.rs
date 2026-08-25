#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatingSystem {
    Windows,
    Android,
    Linux,
    Ios,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionStatus {
    Handshaking,
    Connecting,
    Connected,
    Disconnected,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Capabilities(pub u32);

impl Capabilities {
    pub const FILE_TRANSFER: u32 = 1 << 0;
    pub const CALL_TRANSFER: u32 = 1 << 1;
    pub const REMOTE_ACCESS: u32 = 1 << 2;
    pub const STREAMING:     u32 = 1 << 3;

    pub fn contains(&self, flag: u32) -> bool {
        (self.0 & flag) != 0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceInfo {
    pub os: OperatingSystem,
    pub device_name: String,
    pub device_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Connection {
    pub status: ConnectionStatus,
    pub device_info: Option<DeviceInfo>,
    pub connection_id: Option<String>,
    pub capabilities: Option<Capabilities>,
    pub connected_at: Option<u64>,
}

impl Connection {
    pub fn new() -> Self {
        Self {
            status: ConnectionStatus::Handshaking,
            device_info: None,
            connection_id: None,
            capabilities: None,
            connected_at: None,
        }
    }

    pub fn identify(&mut self, info: DeviceInfo) {
        self.device_info = Some(info);
        self.status = ConnectionStatus::Connecting;
    }

    pub fn complete_handshake(&mut self, conn_id: String, caps: Capabilities, timestamp: u64) {
        self.connection_id = Some(conn_id);
        self.capabilities = Some(caps);
        self.connected_at = Some(timestamp);
        self.status = ConnectionStatus::Connected;
    }

    pub fn fail(&mut self) {
        self.status = ConnectionStatus::Failed;
    }

    pub fn disconnect(&mut self) {
        self.status = ConnectionStatus::Disconnected;
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Connections {
    pub list: Vec<Connection>,
}

impl Connections {
    pub fn active_connections(&self) -> Vec<&Connection> {
        self.list
            .iter()
            .filter(|c| c.status == ConnectionStatus::Connected)
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VeyruxDevice {
    pub host: DeviceInfo,
    pub capabilities: Capabilities,
    pub connections: Connections,
}