use serde::Serialize;

use crate::Ollama;

impl Ollama {
    /// Delete a model and its data.
    pub async fn delete_model(&self, model_name: String) -> crate::error::Result<()> {
        let request = DeleteModelRequest {
            model_name,
        };

        let uri = format!("{}/api/delete", self.uri());
        let serialized = serde_json::to_string(&request).map_err(|e| e.to_string())?;
        let res = self.reqwest_client.delete(uri)
            .body(serialized)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(res.text().await.unwrap_or_else(|e| e.to_string()).into())
        }
    }
}

#[derive(Serialize)]
struct DeleteModelRequest {
    #[serde(rename = "name")]
    model_name: String,
}