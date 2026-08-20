use std::time::Duration;

const REPOSITORIO: &str = "answet/derroche";
const NOMBRE_INSTALADOR: &str = "Derroche-Setup.exe";

pub async fn buscar_actualizacion() -> Result<Option<(String, String)>, String> {
    let resultado = std::thread::spawn(|| {
        let cliente = reqwest::blocking::Client::builder()
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(10))
            .build()
            .map_err(|error| format!("No se pudo crear el cliente HTTP: {error}"))?;

        let url = format!(
            "https://api.github.com/repos/{}/releases/latest",
            REPOSITORIO
        );

        let respuesta = cliente
            .get(&url)
            .header("User-Agent", "Derroche")
            .header("Accept", "application/vnd.github+json")
            .send()
            .map_err(|error| format!("Error al conectarse con GitHub: {error}"))?;

        if !respuesta.status().is_success() {
            return Err(format!(
                "GitHub respondió con el código {}",
                respuesta.status()
            ));
        }

        let datos: serde_json::Value = respuesta
            .json()
            .map_err(|error| format!("Error procesando la respuesta de GitHub: {error}"))?;

        let tag = datos["tag_name"]
            .as_str()
            .ok_or_else(|| "GitHub no devolvió la versión de la release".to_string())?;

        let version_remota = tag.strip_prefix('v').unwrap_or(tag);

        let version_actual = env!("CARGO_PKG_VERSION");

        if version_remota == version_actual {
            return Ok(None);
        }

        let assets = datos["assets"]
            .as_array()
            .ok_or_else(|| "La release no contiene archivos".to_string())?;

        let instalador = assets
            .iter()
            .find(|asset| asset["name"].as_str() == Some(NOMBRE_INSTALADOR))
            .ok_or_else(|| "La release no contiene Derroche-Setup.exe".to_string())?;

        let url_descarga = instalador["browser_download_url"]
            .as_str()
            .ok_or_else(|| "GitHub no devolvió la URL del instalador".to_string())?;

        Ok(Some((version_remota.to_string(), url_descarga.to_string())))
    })
    .join()
    .map_err(|_| "El proceso de búsqueda de actualizaciones falló".to_string())?;

    resultado
}

pub async fn descargar_actualizacion(url: String) -> Result<std::path::PathBuf, String> {
    let resultado = std::thread::spawn(move || {
        let cliente = reqwest::blocking::Client::builder()
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(60))
            .build()
            .map_err(|error| format!("No se pudo crear el cliente HTTP: {error}"))?;

        let respuesta = cliente
            .get(&url)
            .header("User-Agent", "Derroche")
            .send()
            .map_err(|error| format!("Error al descargar la actualización: {error}"))?;

        if !respuesta.status().is_success() {
            return Err(format!(
                "GitHub respondió con el código {}",
                respuesta.status()
            ));
        }

        let datos = respuesta
            .bytes()
            .map_err(|error| format!("Error descargando el instalador: {error}"))?;

        let ruta = std::env::temp_dir().join(NOMBRE_INSTALADOR);

        std::fs::write(&ruta, datos)
            .map_err(|error| format!("No se pudo guardar el instalador: {error}"))?;

        Ok(ruta)
    })
    .join()
    .map_err(|_| "El proceso de descarga falló".to_string())?;

    resultado
}

pub fn ejecutar_instalador(ruta: std::path::PathBuf) -> Result<(), String> {
    std::process::Command::new(&ruta)
        .spawn()
        .map_err(|error| format!("No se pudo ejecutar el instalador: {error}"))?;

    Ok(())
}
