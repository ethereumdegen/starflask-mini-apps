use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Palette {
    pub name: String,
    pub mood: String,
    pub colors: Vec<PaletteColor>,
    pub use_case: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaletteColor {
    pub hex: String,
    pub name: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SavedPalette {
    pub id: Uuid,
    #[serde(flatten)]
    pub palette: Palette,
    pub premise: String,
}

#[derive(Debug, Deserialize)]
pub struct GenerateRequest {
    pub premise: String,
}

#[derive(Debug, Serialize)]
pub struct GenerateResponse {
    pub palettes: Vec<Palette>,
}

#[derive(Debug, Deserialize)]
pub struct SavePaletteRequest {
    #[serde(flatten)]
    pub palette: Palette,
    pub premise: String,
}
