#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PaxType {
    pub name: String,
}

impl From<&str> for PaxType {
    fn from(value: &str) -> Self {
        PaxType::from(value.to_string())
    }
}

impl From<String> for PaxType {
    fn from(value: String) -> Self {
        PaxType { name: value }
    }
}
