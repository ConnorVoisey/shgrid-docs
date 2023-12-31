use crate::query_params::{SortingDirection, StdQueryParams};
use std::cmp::Ordering;

pub mod contact;
pub mod organisation;

pub const DEFAULT_INDEX_LIMIT: usize = 50;
pub const DEFAULT_INDEX_OFFSET: usize = 0;

pub trait IndexQueryable {
    type Fields;
    type Output: IndexQueryable<Fields = Self::Fields>;
    fn get_col_str_val(&self, key: &Self::Fields) -> String;
    fn index(input: Vec<Self::Output>, query: StdQueryParams<Self::Fields>) -> Vec<Self::Output> {
        let mut data: Vec<Self::Output> = input
            .into_iter()
            .filter(|el| {
                for (key, filter_val) in &query.filters {
                    let val = el.get_col_str_val(key);
                    if !val.contains(filter_val) {
                        return false;
                    }
                }
                true
            })
            .collect();
        data.sort_by(|a, b| {
            let mut cmp = Ordering::Equal;
            for sorter in &query.sort {
                if cmp != Ordering::Equal {
                    break;
                }
                let (key, dir) = &sorter;
                cmp = a.get_col_str_val(key).cmp(&b.get_col_str_val(key));
                if let SortingDirection::Desc = dir {
                    cmp = cmp.reverse();
                }
            }
            cmp
        });

        data.into_iter()
            .skip(query.offset.unwrap_or(DEFAULT_INDEX_OFFSET))
            .take(query.limit.unwrap_or(DEFAULT_INDEX_LIMIT))
            .collect()
    }
}
