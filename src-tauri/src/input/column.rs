#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Column {
    pub header: String,
    pub accessor: String,
}
