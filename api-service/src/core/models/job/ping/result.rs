use frunk::LabelledGeneric;
use ipnet::IpNet;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, PartialEq, Eq, Clone, Deserialize, Serialize, ToSchema, Default, LabelledGeneric,
)]
// Returned JobResultPing
pub struct DisplayJobResultPing {
    pub id: String,
    #[schema(value_type = String, format = "cidr", example = "192.0.2.5/32")]
    pub ip_address: IpNet,
    pub latency: i32,
}
