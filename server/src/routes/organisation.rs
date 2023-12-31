use super::AppState;
use crate::{
    models::{organisation::Organisation, IndexQueryable},
    query_params::{StdQueryParams, StdQueryParamsPreSerialize},
};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use std::sync::Arc;

pub async fn index_organisations(
    State(state): State<Arc<AppState>>,
    Query(query): Query<StdQueryParamsPreSerialize>,
) -> impl IntoResponse {
    let query_res = StdQueryParams::from(query);
    let query = match query_res {
        Ok(val) => val,
        Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    };
    let orgs = Organisation::index(state.organisations.clone(), query);
    let val = json!({"count": orgs.len(), "data": orgs});
    Json(val).into_response()
}
