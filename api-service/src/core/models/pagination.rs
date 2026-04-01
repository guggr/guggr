use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema, Debug, Default)]
/// Returned paginated response
pub struct PaginatedResponse<T> {
    data: Vec<T>,
    meta: PaginatedResponseMetadata,
}

#[derive(Serialize, ToSchema, Debug, Default)]
/// Returned paginated metadata
pub struct PaginatedResponseMetadata {
    #[schema(example = 0)]
    page: i64,
    #[schema(example = 40)]
    per_page: i64,
    #[schema(example = 200)]
    total: i64,
    #[schema(example = 4)]
    total_pages: i64,
}

#[derive(utoipa::IntoParams, Deserialize)]
#[into_params(parameter_in = Query)]
#[allow(unused)]
pub struct PaginationQuery {
    /// Current Page
    #[param(default = default_page, minimum = 0, maximum = 4294967295u32)]
    #[serde(default = "default_page")]
    pub page: u32,
    /// Items per page
    #[param(default = default_per_page, minimum = 0, maximum = 4294967295u32)]
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

const fn default_page() -> u32 {
    0
}
const fn default_per_page() -> u32 {
    40
}

impl<T> PaginatedResponse<T> {
    #[must_use]
    pub const fn new(data: Vec<T>, meta: PaginatedResponseMetadata) -> Self {
        Self { data, meta }
    }
}

impl PaginatedResponseMetadata {
    /// build the Pagination Metadata from a query and the total amount of
    /// records
    #[must_use]
    pub fn build(value: &PaginationQuery, total: i64) -> Self {
        // i64 div_ceil is still only available on nightly
        let total_pages = if value.per_page == 0 {
            0
        } else {
            (total + i64::from(value.per_page) - 1) / i64::from(value.per_page)
        };
        Self {
            page: value.page.into(),
            per_page: value.per_page.into(),
            total,
            total_pages,
        }
    }
}
