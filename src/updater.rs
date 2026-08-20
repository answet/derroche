use reqwest::Client;

const REPOSITORIO: &str = "answet/derroche";

pub async fn buscar_actualizacion() -> Result<Option<String>, String> {
    let cliente = Client::new();

    let respuesta = cliente
        .get(format!(
            "https://api.github.com/repos/{}/releases/latest",
            REPOSITORIO
        ))
        .header("User-Agent", "Derroche")
        .header("Accept", "application/vnd.github+json")
        .send()
        .await
        .map_err(|error| error.to_string())?;

    if !respuesta.status().is_success() {
        return Err(format!(
            "GitHub respondió con {}",
            respuesta.status()
        ));
    }

    let datos: serde_json::Value = respuesta
        .json()
        .await
        .map_err(|error| error.to_string())?;

    let version_actual = env!("CARGO_PKG_VERSION");

    let tag = datos["tag_name"]
        .as_str()
        .ok_or_else(|| "La release no tiene tag".to_string())?;

    let version_remota = tag.strip_prefix('v').unwrap_or(tag);

    if version_remota != version_actual {
        Ok(Some(version_remota.to_string()))
    } else {
        Ok(None)
    }
}
