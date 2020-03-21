#[derive(Serialize, Deserialize)]
pub struct RequestHeaderV0 {
    api_key: i16,
    api_version: i16,
    correlation_id: i32,
    dummy: u16,
}

impl RequestHeaderV0 {
    pub fn api_version_request_v0() -> Self {
        Self {
            api_key: 18,
            api_version: 0,
            correlation_id: 1,
            dummy: 0xffff,
        }
    }
}
