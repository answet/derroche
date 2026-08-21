use std::time::Duration;

const REPOSITORIO: &str = "answet/derroche";

#[cfg(target_os = "windows")]
const NOMBRE_INSTALADOR: &str = "Derroche-Setup.exe";


pub async fn buscar_actualizacion(
) -> Result<Option<(String, Option<String>)>, String> {
    let resultado = std::thread::spawn(|| {
        let cliente = reqwest::blocking::Client::builder()
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(10))
            .build()
            .map_err(|error| {
                format!(
                    "No se pudo crear el cliente HTTP: {error}"
                )
            })?;

        let url = format!(
            "https://api.github.com/repos/{}/releases/latest",
            REPOSITORIO
        );

        let respuesta = cliente
            .get(&url)
            .header("User-Agent", "Derroche")
            .header(
                "Accept",
                "application/vnd.github+json",
            )
            .send()
            .map_err(|error| {
                format!(
                    "Error al conectarse con GitHub: {error}"
                )
            })?;

        if !respuesta.status().is_success() {
            return Err(format!(
                "GitHub respondió con el código {}",
                respuesta.status()
            ));
        }

        let datos: serde_json::Value = respuesta
            .json()
            .map_err(|error| {
                format!(
                    "Error procesando la respuesta de GitHub: {error}"
                )
            })?;

        let tag = datos["tag_name"]
            .as_str()
            .ok_or_else(|| {
                "GitHub no devolvió la versión de la release"
                    .to_string()
            })?;

        let version_remota = tag.strip_prefix('v').unwrap_or(tag);
        let version_remota = parsear_version(version_remota)
            .map_err(|error| {
                format!(
                    "GitHub devolvió una versión inválida ({tag}): {error}"
                )
            })?;

        let version_actual = parsear_version(env!("CARGO_PKG_VERSION"))
            .map_err(|error| {
                format!(
                    "La versión actual de la aplicación no es válida: {error}"
                )
            })?;

        if comparar_versiones(&version_remota, &version_actual)
            != std::cmp::Ordering::Greater
        {
            return Ok(None);
        }

        #[cfg(target_os = "windows")]
        {
            let assets = datos["assets"]
                .as_array()
                .ok_or_else(|| {
                    "La release no contiene archivos".to_string()
                })?;

            let instalador = assets
                .iter()
                .find(|asset| {
                    asset["name"].as_str()
                        == Some(NOMBRE_INSTALADOR)
                })
                .ok_or_else(|| {
                    "La release no contiene Derroche-Setup.exe"
                        .to_string()
                })?;

            let url_descarga =
                instalador["browser_download_url"]
                    .as_str()
                    .ok_or_else(|| {
                        "GitHub no devolvió la URL del instalador"
                            .to_string()
                    })?;

            Ok(Some((
                formatear_version(&version_remota),
                Some(url_descarga.to_string()),
            )))
        }

        #[cfg(not(target_os = "windows"))]
        {
            Ok(Some((
                formatear_version(&version_remota),
                None,
            )))
        }
    })
    .join()
    .map_err(|_| {
        "El proceso de búsqueda de actualizaciones falló"
            .to_string()
    })?;

    resultado
}

type Version = (u64, u64, u64, Vec<IdentificadorPrerelease>);

#[derive(Debug, PartialEq, Eq)]
enum IdentificadorPrerelease {
    Numerico(u64),
    Texto(String),
}

fn parsear_version(version: &str) -> Result<Version, String> {
    let version = version.split('+').next().unwrap_or(version);
    let (base, prerelease) = version.split_once('-').unwrap_or((version, ""));
    let partes: Vec<_> = base.split('.').collect();

    if partes.len() != 3 {
        return Err("debe tener formato MAJOR.MINOR.PATCH".to_string());
    }

    let parsear_numero = |valor: &str| {
        if valor.is_empty() || (valor.len() > 1 && valor.starts_with('0')) {
            return Err("los números no pueden tener ceros iniciales".to_string());
        }

        valor
            .parse::<u64>()
            .map_err(|_| format!("número inválido: {valor}"))
    };

    let major = parsear_numero(partes[0])?;
    let minor = parsear_numero(partes[1])?;
    let patch = parsear_numero(partes[2])?;
    let mut identificadores = Vec::new();

    if !prerelease.is_empty() {
        for identificador in prerelease.split('.') {
            if identificador.is_empty()
                || !identificador.chars().all(|caracter| {
                    caracter.is_ascii_alphanumeric() || caracter == '-'
                })
            {
                return Err("identificador prerelease inválido".to_string());
            }

            if identificador.chars().all(|caracter| caracter.is_ascii_digit()) {
                identificadores.push(IdentificadorPrerelease::Numerico(
                    parsear_numero(identificador)?,
                ));
            } else {
                identificadores.push(IdentificadorPrerelease::Texto(
                    identificador.to_string(),
                ));
            }
        }
    }

    Ok((major, minor, patch, identificadores))
}

fn comparar_versiones(a: &Version, b: &Version) -> std::cmp::Ordering {
    use std::cmp::Ordering;

    for (izquierda, derecha) in [
        (a.0, b.0),
        (a.1, b.1),
        (a.2, b.2),
    ] {
        match izquierda.cmp(&derecha) {
            Ordering::Equal => {}
            orden => return orden,
        }
    }

    match (a.3.is_empty(), b.3.is_empty()) {
        (true, true) => Ordering::Equal,
        (true, false) => Ordering::Greater,
        (false, true) => Ordering::Less,
        (false, false) => {
            for (izquierda, derecha) in a.3.iter().zip(&b.3) {
                let orden = match (izquierda, derecha) {
                    (IdentificadorPrerelease::Numerico(a), IdentificadorPrerelease::Numerico(b)) => a.cmp(b),
                    (IdentificadorPrerelease::Numerico(_), IdentificadorPrerelease::Texto(_)) => Ordering::Less,
                    (IdentificadorPrerelease::Texto(_), IdentificadorPrerelease::Numerico(_)) => Ordering::Greater,
                    (IdentificadorPrerelease::Texto(a), IdentificadorPrerelease::Texto(b)) => a.cmp(b),
                };

                if orden != Ordering::Equal {
                    return orden;
                }
            }

            a.3.len().cmp(&b.3.len())
        }
    }
}

fn formatear_version(version: &Version) -> String {
    let mut resultado = format!("{}.{}.{}", version.0, version.1, version.2);

    if !version.3.is_empty() {
        resultado.push('-');
        resultado.push_str(
            &version
                .3
                .iter()
                .map(|identificador| match identificador {
                    IdentificadorPrerelease::Numerico(numero) => numero.to_string(),
                    IdentificadorPrerelease::Texto(texto) => texto.clone(),
                })
                .collect::<Vec<_>>()
                .join("."),
        );
    }

    resultado
}


#[cfg(target_os = "windows")]
pub async fn descargar_actualizacion(
    url: String,
) -> Result<std::path::PathBuf, String> {
    let resultado = std::thread::spawn(move || {
        let cliente = reqwest::blocking::Client::builder()
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(60))
            .build()
            .map_err(|error| {
                format!(
                    "No se pudo crear el cliente HTTP: {error}"
                )
            })?;

        let respuesta = cliente
            .get(&url)
            .header("User-Agent", "Derroche")
            .send()
            .map_err(|error| {
                format!(
                    "Error al descargar la actualización: {error}"
                )
            })?;

        if !respuesta.status().is_success() {
            return Err(format!(
                "GitHub respondió con el código {}",
                respuesta.status()
            ));
        }

        let datos = respuesta
            .bytes()
            .map_err(|error| {
                format!(
                    "Error descargando el instalador: {error}"
                )
            })?;

        let ruta = std::env::temp_dir()
            .join("Derroche-Setup.exe");

        std::fs::write(&ruta, datos)
            .map_err(|error| {
                format!(
                    "No se pudo guardar el instalador: {error}"
                )
            })?;

        Ok(ruta)
    })
    .join()
    .map_err(|_| {
        "El proceso de descarga falló".to_string()
    })?;

    resultado
}
