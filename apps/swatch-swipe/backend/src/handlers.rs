use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::models::{GenerateRequest, GenerateResponse, SavePaletteRequest, SavedPalette};
use crate::starflask;
use crate::AppState;

pub async fn generate_palette(
    State(state): State<AppState>,
    Json(req): Json<GenerateRequest>,
) -> Result<Json<GenerateResponse>, (StatusCode, String)> {
    let premise = req.premise.trim().to_string();
    if premise.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Premise cannot be empty".to_string(),
        ));
    }

    let palettes = if let Some(ref client) = state.starflask_client {
        client
            .generate_palettes(&premise)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?
    } else {
        tracing::info!("Using mock palettes (no STARFLASK_API_URL configured)");
        starflask::mock_palettes(&premise)
    };

    Ok(Json(GenerateResponse { palettes }))
}

pub async fn list_palettes(
    State(state): State<AppState>,
) -> Json<Vec<SavedPalette>> {
    let palettes = state.saved_palettes.lock().unwrap();
    Json(palettes.clone())
}

pub async fn save_palette(
    State(state): State<AppState>,
    Json(req): Json<SavePaletteRequest>,
) -> (StatusCode, Json<SavedPalette>) {
    let saved = SavedPalette {
        id: Uuid::new_v4(),
        palette: req.palette,
        premise: req.premise,
    };

    let mut palettes = state.saved_palettes.lock().unwrap();
    palettes.push(saved.clone());

    (StatusCode::CREATED, Json(saved))
}

pub async fn delete_palette(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> StatusCode {
    let mut palettes = state.saved_palettes.lock().unwrap();
    let len_before = palettes.len();
    palettes.retain(|p| p.id != id);

    if palettes.len() < len_before {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
