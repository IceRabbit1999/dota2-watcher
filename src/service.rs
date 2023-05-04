use std::collections::HashMap;
use std::sync::Arc;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use crate::{AppError, AppState};
use crate::client::{Courier, PlayerPerformance};

/// Get the player performance of the latest match by a given account_id.
/// Request example: http://ipaddr:port/match/latest?account_id=xxx
pub async fn latest_match(State(state): State<AppState>, param: Query<HashMap<String, String>>) -> Result<Json<PlayerPerformance>, AppError> {

    let account_id = param.0.get("account_id").unwrap().to_owned();
    // get the latest match id
    let res = state.client.match_history_with_account_id(&account_id, 1).await?;

    let match_id = res.into_keys()
        .collect::<String>();

    let detail = state.client.match_detail(&match_id).await?;
    let performance = PlayerPerformance::from_value(detail, &account_id)?;

    // todo: check cache status
    // if state.cache.lock().unwrap().contains_key(&(account_id, match_id)) {
    //
    // }

    Ok(Json(performance))
}