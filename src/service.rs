use std::collections::HashMap;
use std::sync::Arc;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use crate::AppError;
use crate::client::{Courier, PlayerPerformance};

/// Request example: http://ipaddr:port/match/latest?account_id=xxx
pub async fn latest_match(State(courier): State<Arc<Courier>>, param: Query<HashMap<String, String>>) -> Result<String, AppError> {
    let account_id = param.0.get("account_id").unwrap();
    // get the latest match id
    let res = courier.match_history_with_account_id(account_id, 1).await?;

    let match_id = res.into_keys()
        .collect::<String>();

    let detail = courier.match_detail(&match_id).await?;
    let performance = PlayerPerformance::from_value(detail, account_id)?;

    Ok(serde_json::to_string(&performance).unwrap())
}

