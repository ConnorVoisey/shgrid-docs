use crate::error::Result;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct StdQueryParamsPreSerialize {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub sort: Option<String>,
    pub filters: Option<String>,
}
#[derive(Deserialize, Debug)]
pub struct StdQueryParams<T> {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub sort: Sorting<T>,
    pub filters: Filtering<T>,
}
pub type Filtering<T> = Vec<(T, String)>;
pub type Sorting<T> = Vec<(T, SortingDirection)>;

impl<T> StdQueryParams<T>
where
    T: DeserializeOwned,
{
    pub fn from(pre_serialize: StdQueryParamsPreSerialize) -> Result<Self> {
        let filters = match pre_serialize.filters {
            Some(filters) => serde_json::from_str::<Filtering<T>>(&filters)?,
            None => vec![],
        };
        let sort = match pre_serialize.sort {
            Some(sort) => serde_json::from_str::<Sorting<T>>(&sort)?,
            None => vec![],
        };
        Ok(Self {
            limit: pre_serialize.limit,
            offset: pre_serialize.offset,
            filters,
            sort,
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SortingDirection {
    Asc,
    Desc,
}
