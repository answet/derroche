use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Categoria {
    pub id: i32,
    pub nombre: String,
}

impl fmt::Display for Categoria {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.nombre)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Persona {
    pub id: i32,
    pub nombre: String,
}

impl fmt::Display for Persona {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.nombre)
    }
}

#[derive(Debug, Clone)]
pub struct GastoDetalle {
    pub id: i32,
    pub descripcion: String,
    pub monto: f64,
    pub fecha: String,
    pub categoria: String,
    pub persona: String,
}

#[derive(Debug, Clone)]
pub struct TotalMensual {
    pub mes: u32,
    pub anio: i32,
    pub total: f64,
}

#[derive(Debug, Clone)]
pub struct GastoPorCategoria {
    pub categoria: String,
    pub total: f64,
}

#[derive(Debug, Clone)]
pub struct GastoPorPersona {
    pub persona: String,
    pub total: f64,
}

#[derive(Debug, Clone)]
pub struct Configuracion {
    pub mes_default: u32,
    pub anio_default: i32,
}
