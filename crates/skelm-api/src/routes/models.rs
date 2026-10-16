//! `GET /v1/models` and `GET /v1/models/{id}`.

use std::str::FromStr;

use axum::Json;
use axum::extract::{Path, State};

use skelm_hf::{HfCache, HfModelRef};
use skelm_ollama::{ModelDescr as OllamaDescr, OllamaStore};

use crate::error::ApiError;
use crate::protocol::{ModelList, ModelObject};
use crate::state::AppState;

pub async fn list_models(State(_state): State<AppState>) -> Result<Json<ModelList>, ApiError> {
    let (descrs, hf_models) = tokio::task::spawn_blocking(|| {
        let hf_models = HfCache::from_env().list_models();
        (OllamaStore::default().list_model_descrs(), hf_models)
    })
    .await?;
    let descrs = descrs.map_err(|e| ApiError::Internal(format!("listing models: {e}")))?;

    let data = descrs
        .into_iter()
        .map(|d| ModelObject::new(d.to_string()))
        .chain(
            hf_models
                .into_iter()
                .map(|m| ModelObject::new(m.model.to_string())),
        )
        .collect();

    Ok(Json(ModelList {
        object: "list",
        data,
    }))
}

pub async fn get_model(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ModelObject>, ApiError> {
    let found = if HfModelRef::is_hf_name(&id) {
        let model: HfModelRef = id
            .parse()
            .map_err(|_| ApiError::BadRequest(format!("invalid model id '{id}'")))?;
        tokio::task::spawn_blocking(move || HfCache::from_env().resolve(&model).is_ok()).await?
    } else {
        let descr = OllamaDescr::from_str(&id)
            .map_err(|_| ApiError::BadRequest(format!("invalid model id '{id}'")))?;
        tokio::task::spawn_blocking(move || OllamaStore::default().get_manifest(&descr).is_ok())
            .await?
    };

    if !found {
        return Err(ApiError::ModelNotFound(id));
    }

    Ok(Json(ModelObject::new(id)))
}
