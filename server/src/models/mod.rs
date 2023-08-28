pub mod contact;
pub mod organisation;
use crate::error::{Error, Result};
use crate::query_params::{Filters, Sorting};
use sea_query::{Expr, Iden};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait GetColFromStr {
    type Item;
    fn get_col_from_str(search: &str) -> Option<Self::Item>;
    fn add_filters<'a>(
        query: &'a mut sea_query::SelectStatement,
        filters: &Filters,
    ) -> Result<&'a mut sea_query::SelectStatement>
    where
        <Self as GetColFromStr>::Item: Iden + 'static,
    {
        for filter in filters {
            let col = match Self::get_col_from_str(&filter[0]) {
                Some(val) => val,
                None => {
                    return Err(Error::UnprocessableEntity {
                        errors: HashMap::from([(
                            "filter".to_owned(),
                            vec![format!("invalid column name: {}", filter[0])],
                        )]),
                    });
                }
            };
            let value = &filter[1];
            query.and_where(Expr::col(col).like(&format!("%{value}%")));
        }
        Ok(query)
    }

    fn add_sorting<'a>(
        query: &'a mut sea_query::SelectStatement,
        sorting: &Sorting,
    ) -> Result<&'a mut sea_query::SelectStatement>
    where
        <Self as GetColFromStr>::Item: Iden + 'static,
    {
        for sorter in sorting {
            let col = match Self::get_col_from_str(&sorter[0]) {
                Some(val) => val,
                None => {
                    return Err(Error::UnprocessableEntity {
                        errors: HashMap::from([(
                            "sorting".to_owned(),
                            vec![format!("invalid column name: {}", sorter[0])],
                        )]),
                    });
                }
            };
            let direction = match &sorter[1] {
                _ if &sorter[1] == "asc" => sea_query::Order::Asc,
                _ if &sorter[1] == "desc" => sea_query::Order::Desc,
                _ => {
                    return Err(Error::UnprocessableEntity {
                        errors: HashMap::from([(
                            "sorting".to_owned(),
                            vec![format!(
                                r#"invalid direction: {}, expecting "asc" or "desc""#,
                                sorter[1]
                            )],
                        )]),
                    });
                }
            };
            query.order_by(col, direction);
        }
        Ok(query)
    }
}

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct RowCount {
    count: i64,
}
