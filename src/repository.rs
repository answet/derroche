use rusqlite::Connection;
use crate::models::{Categoria, GastoDetalle, Persona, TotalMensual, GastoPorCategoria, GastoPorPersona, Configuracion};

const CATEGORIA_SIN_CATEGORIA_ID: i32 = 1;
const PERSONA_GENERAL_ID: i32 = 1;

pub fn agregar_categoria(
    conn: &Connection,
    nombre: &str,
    ) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO categorias (nombre) VALUES (?1)",
        [nombre],
    )?;

    Ok(())
}

pub fn obtener_categorias(
    conn: &Connection,
    ) -> rusqlite::Result<Vec<Categoria>> {
    let mut stmt = conn.prepare(
        "SELECT id, nombre FROM categorias"
    )?;

    let categorias = stmt
        .query_map([], |row| {
            Ok(Categoria {
                id: row.get(0)?,
                nombre: row.get(1)?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    Ok(categorias)
}

pub fn eliminar_categoria(conn: &Connection, id: i32) -> rusqlite::Result<()> {
    if id == CATEGORIA_SIN_CATEGORIA_ID {
        return  Err(rusqlite::Error::InvalidParameterName(
            "No se puede eliminar 'Sin Categoria'".to_string(),
        ));
    }

    let tx = conn.unchecked_transaction()?;

    tx.execute(
        "
        UPDATE gastos
        SET categoria_id = ?1
        WHERE categoria_id = ?2
        ",
        (CATEGORIA_SIN_CATEGORIA_ID, id),
    )?;

    tx.execute(
        "
        DELETE FROM categorias
        WHERE id = ?1
        ",
        [id],
    )?;

    tx.commit()?;

    Ok(())
}

pub fn agregar_persona(conn: &Connection, nombre: &str) -> rusqlite::Result<()> {
    conn.execute(
        "
        INSERT INTO personas (nombre) VALUES (?1)
        ",
        [nombre],
    )?;

    Ok(())
}

pub fn obtener_personas(conn: &Connection) -> rusqlite::Result<Vec<Persona>> {
    let mut stmt = conn.prepare("SELECT id, nombre FROM personas")?;

    let personas = stmt
        .query_map([], |row| {
            Ok(Persona {
                id: row.get(0)?,
                nombre: row.get(1)?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    Ok(personas)
}

pub fn eliminar_persona(conn: &Connection, id: i32) -> rusqlite::Result<()> {

    if id == PERSONA_GENERAL_ID {
        return Err(rusqlite::Error::InvalidParameterName(
            "No se puede eliminar 'General'".to_string(),
        ));
    }

    let tx = conn.unchecked_transaction()?;

    tx.execute(
        "
        UPDATE gastos
        SET persona_id = ?1
        WHERE persona_id = ?2
        ",
        (PERSONA_GENERAL_ID, id),
    )?;

    tx.execute(
        "
        DELETE FROM personas
        WHERE id = ?1
        ",
        [id],
    )?;

    tx.commit()?;

    Ok(())
}

pub fn agregar_gasto(
    conn: &Connection,
    descripcion: &str,
    monto: f64,
    fecha: &str,
    categoria_id: i32,
    persona_id: i32
    ) -> rusqlite::Result<()> {

    conn.execute(
        "
        INSERT INTO gastos
        (descripcion, monto, fecha, categoria_id, persona_id)
        VALUES (?1, ?2, ?3, ?4, ?5)
        ",
        (
            descripcion,
            monto,
            fecha,
            categoria_id,
            persona_id,
        ),
    )?;

    Ok(())
}

pub fn eliminar_gasto(conn: &Connection, id: i32) -> rusqlite::Result<()> {
    conn.execute(
        "DELETE FROM gastos WHERE id = ?1",
        [id],
        )?;

    Ok(())
}

pub fn actualizar_gasto(
    conn: &Connection,
    id: i32,
    descripcion: &str,
    monto: f64,
    fecha: &str,
    categoria_id: i32,
    persona_id: i32,
) -> rusqlite::Result<()> {
    conn.execute(
        "
        UPDATE gastos
        SET
            descripcion = ?1,
            monto = ?2,
            fecha = ?3,
            categoria_id = ?4,
            persona_id = ?5
        WHERE id = ?6
        ",
        (
            descripcion,
            monto,
            fecha,
            categoria_id,
            persona_id,
            id,
        ),
    )?;

    Ok(())
}

pub fn obtener_gastos_detalle(conn: &Connection) -> rusqlite::Result<Vec<GastoDetalle>> {
    let mut stmt = conn.prepare(
        "
        SELECT
            gastos.id,
            gastos.descripcion,
            gastos.monto,
            gastos.fecha,
            categorias.nombre,
            personas.nombre
        FROM gastos
        JOIN categorias
            ON gastos.categoria_id = categorias.id
        JOIN personas
            ON gastos.persona_id = personas.id
        ORDER BY gastos.fecha DESC, gastos.id DESC
        "
        )?;

    let gastos = stmt
        .query_map([], |row| {
            Ok(GastoDetalle {
                id: row.get(0)?,
                descripcion: row.get(1)?,
                monto: row.get(2)?,
                fecha: row.get(3)?,
                categoria: row.get(4)?,
                persona: row.get(5)?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    Ok(gastos)
}

pub fn obtener_total_mes(
    conn: &Connection,
    mes: u32,
    anio: i32,
) -> rusqlite::Result<f64> {
    let mut stmt = conn.prepare(
        "
        SELECT COALESCE(SUM(monto), 0)
        FROM gastos
        WHERE CAST(substr(fecha, 4, 2) AS INTEGER) = ?1
        AND CAST(substr(fecha, 7, 4) AS INTEGER) = ?2
        "
    )?;

    let total = stmt.query_row((mes, anio), |row| row.get(0))?;

    Ok(total)
}

pub fn obtener_mayor_gasto_mes(
    conn: &Connection,
    mes: u32,
    anio: i32,
) -> rusqlite::Result<Option<GastoDetalle>> {
    let mut stmt = conn.prepare(
        "
        SELECT
            gastos.id,
            gastos.descripcion,
            gastos.monto,
            gastos.fecha,
            categorias.nombre,
            personas.nombre
        FROM gastos
        JOIN categorias
            ON gastos.categoria_id = categorias.id
        JOIN personas
            ON gastos.persona_id = personas.id
        WHERE CAST(substr(gastos.fecha, 4, 2) AS INTEGER) = ?1
        AND CAST(substr(gastos.fecha, 7, 4) AS INTEGER) = ?2
        ORDER BY gastos.monto DESC
        LIMIT 1
        "
    )?;

    let mut filas = stmt.query((mes, anio))?;

    if let Some(row) = filas.next()? {
        Ok(Some(GastoDetalle {
            id: row.get(0)?,
            descripcion: row.get(1)?,
            monto: row.get(2)?,
            fecha: row.get(3)?,
            categoria: row.get(4)?,
            persona: row.get(5)?,
        }))
    } else {
        Ok(None)
    }
}

pub fn obtener_totales_mensuales(
    conn: &Connection,
) -> rusqlite::Result<Vec<TotalMensual>> {
    let mut stmt = conn.prepare(
        "
        SELECT
            CAST(substr(fecha, 4, 2) AS INTEGER) AS mes,
            CAST(substr(fecha, 7, 4) AS INTEGER) AS anio,
            SUM(monto) AS total
        FROM gastos
        GROUP BY anio, mes
        ORDER BY anio, mes
        "
    )?;

    let totales = stmt
        .query_map([], |row| {
            Ok(TotalMensual {
                mes: row.get(0)?,
                anio: row.get(1)?,
                total: row.get(2)?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    Ok(totales)
}

pub fn obtener_gastos_por_categoria(
    conn: &Connection,
    mes: u32,
    anio: i32,
) -> rusqlite::Result<Vec<GastoPorCategoria>> {
    let mut stmt = conn.prepare(
        "
        SELECT
            categorias.nombre,
            SUM(gastos.monto) AS total
        FROM gastos
        JOIN categorias
            ON gastos.categoria_id = categorias.id
        WHERE
            CAST(substr(gastos.fecha, 4, 2) AS INTEGER) = ?1
            AND CAST(substr(gastos.fecha, 7, 4) AS INTEGER) = ?2
        GROUP BY categorias.id, categorias.nombre
        ORDER BY total DESC
        "
    )?;

    let gastos = stmt
        .query_map((mes, anio), |row| {
            Ok(GastoPorCategoria {
                categoria: row.get(0)?,
                total: row.get(1)?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    Ok(gastos)
}

pub fn obtener_gastos_por_persona(
    conn: &Connection,
    mes: u32,
    anio: i32,
) -> rusqlite::Result<Vec<GastoPorPersona>> {
    let mut stmt = conn.prepare(
        "
        SELECT
            personas.nombre,
            SUM(gastos.monto) AS total
        FROM gastos
        JOIN personas
            ON gastos.persona_id = personas.id
        WHERE
            CAST(substr(gastos.fecha, 4, 2) AS INTEGER) = ?1
            AND CAST(substr(gastos.fecha, 7, 4) AS INTEGER) = ?2
        GROUP BY personas.id, personas.nombre
        ORDER BY total DESC
        "
    )?;

    let gastos = stmt
        .query_map((mes, anio), |row| {
            Ok(GastoPorPersona {
                persona: row.get(0)?,
                total: row.get(1)?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    Ok(gastos)
}

pub fn obtener_configuracion(
    conn: &Connection,
) -> rusqlite::Result<Configuracion> {
    let mut stmt = conn.prepare(
        "
        SELECT mes_default, anio_default
        FROM configuracion
        WHERE id = 1
        "
    )?;

    stmt.query_row([], |row| {
        Ok(Configuracion {
            mes_default: row.get(0)?,
            anio_default: row.get(1)?,
        })
    })
}

pub fn actualizar_configuracion(
    conn: &Connection,
    mes_default: u32,
    anio_default: i32,
) -> rusqlite::Result<()> {
    conn.execute(
        "
        UPDATE configuracion
        SET
            mes_default = ?1,
            anio_default = ?2
        WHERE id = 1
        ",
        (mes_default, anio_default),
    )?;

    Ok(())
}
