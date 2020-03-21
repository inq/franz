pub mod request;

#[derive(Debug, Serialize, Deserialize)]
pub struct Size(pub i32);

impl Size {
    pub fn new(value: i32) -> Self {
        Self(value)
    }
}
