use frunk::LabelledGeneric;
use ipnet::IpNet;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, PartialEq, Eq, Clone, Deserialize, Serialize, ToSchema, Default, LabelledGeneric,
)]
/// returned from the database when displaying a job result
pub struct DisplayJobResultHttp {
    pub id: String,
    #[schema(value_type = String, format = "cidr", example = "192.0.2.5/32")]
    pub ip_address: IpNet,
    pub status_code: i32,
    pub latency: i32,
    pub payload: Vec<u8>,
}
