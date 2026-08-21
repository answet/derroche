use directories::ProjectDirs;
use rusqlite::{Connection, Result};

pub fn conectar() -> Result<Connection> {
    let project_dirs = ProjectDirs::from("", "", "Derroche")
        .ok_or_else(|| {
            rusqlite::Error::InvalidPath(
                "No se pudo determinar el directorio de datos".into(),
            )
        })?;

    let directorio = project_dirs.data_local_dir();

    std::fs::create_dir_all(directorio)
        .map_err(|_| {
            rusqlite::Error::InvalidPath(
                directorio.to_path_buf(),
            )
        })?;

    let ruta = directorio.join("derroche.db");

    let conexion = Connection::open(ruta)?;

    conexion.execute("PRAGMA foreign_keys = ON", [])?;

    Ok(conexion)
}

pub fn conectar_inicializada() -> Result<Connection, String> {
    let conexion = conectar().map_err(|error| error.to_string())?;

    // Las operaciones pueden ejecutarse antes de que la app haya cargado datos iniciales.
    inicializar_db(&conexion).map_err(|error| error.to_string())?;

    Ok(conexion)
}

pub fn inicializar_db(conexion: &Connection) -> rusqlite::Result<()> {
    conexion.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS categorias (
            id INTEGER PRIMARY KEY,
            nombre TEXT NOT NULL UNIQUE
        );

        CREATE TABLE IF NOT EXISTS personas (
            id INTEGER PRIMARY KEY,
            nombre TEXT NOT NULL UNIQUE
        );

        CREATE TABLE IF NOT EXISTS gastos (
            id INTEGER PRIMARY KEY,
            descripcion TEXT NOT NULL,
            monto REAL NOT NULL CHECK(monto > 0),
            fecha TEXT NOT NULL,
            categoria_id INTEGER NOT NULL,
            persona_id INTEGER NOT NULL,

            FOREIGN KEY (categoria_id)
                REFERENCES categorias(id),

            FOREIGN KEY (persona_id)
                REFERENCES personas(id)
        );

        CREATE TABLE IF NOT EXISTS configuracion (
            id INTEGER PRIMARY KEY,
            mes_default INTEGER NOT NULL,
            anio_default INTEGER NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_gastos_mes_anio
        ON gastos (
            CAST(substr(fecha, 4, 2) AS INTEGER),
            CAST(substr(fecha, 7, 4) AS INTEGER)
        );
        ",
    )?;

    conexion.execute(
        "INSERT OR IGNORE INTO categorias (id, nombre) VALUES (1, 'Sin Categoria')",
        [],
    )?;

    conexion.execute(
        "INSERT OR IGNORE INTO personas (id, nombre) VALUES (1, 'General')",
        [],
    )?;

    conexion.execute(
        "INSERT OR IGNORE INTO configuracion (id, mes_default, anio_default) VALUES (1, 8, 2026)",
        [],
    )?;

    Ok(())
}
