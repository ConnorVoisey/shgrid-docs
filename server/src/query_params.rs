use crate::error::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct StdQueryParamsPreSerialize {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub sorting: Option<String>,
    pub filters: Option<String>,
}
#[derive(Deserialize, Debug)]
pub struct StdQueryParams {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub sorting: Sorting,
    pub filters: Filters,
}
pub type Filters = Vec<[String; 2]>;
pub type Sorting = Vec<[String; 2]>;

impl StdQueryParams {
    pub fn from(pre_serialize: StdQueryParamsPreSerialize) -> Result<Self> {
        let filters = match pre_serialize.filters {
            Some(filters) => serde_json::from_str::<Filters>(&filters)?,
            None => vec![],
        };
        let sorting = match pre_serialize.sorting {
            Some(sorting) => serde_json::from_str::<Sorting>(&sorting)?,
            None => vec![],
        };
        Ok(Self {
            limit: pre_serialize.limit,
            offset: pre_serialize.offset,
            filters,
            sorting,
        })
    }
}
