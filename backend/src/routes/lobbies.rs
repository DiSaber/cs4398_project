use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
};

use crate::{app_state::AppState, models::lobby::JoinCode};

pub fn router() -> Router<AppState> {
    Router::new().route("/", post(create_lobby))
}

// TODO: Should this also return the initial owner/admin session token?
/// Creates a new lobby and returns.
async fn create_lobby(State(_app_state): State<AppState>) -> Result<Json<String>, LobbyError> {
    // TODO: Store in db
    let join_code = JoinCode::new(rand::random());
    Ok(Json(join_code.to_string()))
}

// TODO: Probably a shared `AppError`
#[derive(Debug)]
pub enum LobbyError {}

impl IntoResponse for LobbyError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong.".to_owned(),
        )
            .into_response()
    }
}
