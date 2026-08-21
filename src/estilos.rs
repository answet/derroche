use iced::{Background, Border, Color, Shadow, Theme};
use iced::overlay::menu;
use iced::widget::button;


// ==================================================
// BARRA SUPERIOR
// ==================================================

pub const FONDO_BARRA_SUPERIOR: Color =
    Color::from_rgb8(46, 52, 64);

pub const TEXTO_BARRA_SUPERIOR: Color =
    Color::from_rgb8(236, 239, 244);

pub const BOTONES_BARRA_SUPERIOR_INACTIVOS: Color =
    Color::from_rgb8(59, 66, 82);


// ==================================================
// GASTOS
// ==================================================

pub const FONDO_GASTOS: Color =
    Color::from_rgb8(67, 76, 94);

pub const FONDO_TABLA_GASTOS: Color =
    Color::from_rgb8(216, 222, 233);

pub const BORDE_TABLA_GASTOS: Color =
    Color::from_rgb8(129, 161, 193);

pub const GASTOS_TEXTO_TABLA: Color =
    Color::from_rgb8(46, 52, 64);

pub const GASTOS_TEXTO_CABEZA_TABLA: Color =
    Color::from_rgb8(46, 52, 64);

pub const FONDO_FILA_SELECCIONADA: Color =
    Color::from_rgb8(76, 86, 106);

pub const TEXTO_FILA_SELECCIONADA: Color =
    Color::from_rgb8(216, 222, 233);

pub const TEXTO_ERROR: Color =
    Color::from_rgb8(191, 97, 106);

// --------------------------------------------------
// Gastos - Mes
// --------------------------------------------------

pub const GASTOS_TEXTO_MES: Color =
    Color::from_rgb8(236, 239, 244);


// --------------------------------------------------
// Gastos - Selectores
// --------------------------------------------------

pub const GASTOS_TEXTO_SELECTOR: Color =
    Color::from_rgb8(46, 52, 64);

pub const GASTOS_TEXTO_SELECTOR_SELECCIONADO: Color =
    Color::from_rgb8(216, 222, 233);

pub const GASTOS_TEXTO_SELECTOR_FONDO: Color =
    Color::from_rgb8(129, 161, 193);

pub const GASTOS_TEXTO_SELECTOR_FONDO_SELECCIONADO: Color =
    Color::from_rgb8(76, 86, 106);


// --------------------------------------------------
// Gastos - Botones
// --------------------------------------------------

pub const FONDO_BOTONES_GASTOS: Color =
    Color::from_rgb8(216, 222, 233);

pub const BORDE_BOTONES_GASTOS: Color =
    Color::from_rgb8(129, 161, 193);

pub const GASTOS_TEXTO_BOTONES: Color =
    Color::from_rgb8(46, 52, 64);

pub fn estilo_boton_gastos(
    _theme: &Theme,
    _status: button::Status,
) -> button::Style {
    button::Style {
        background: Some(Background::Color(FONDO_BOTONES_GASTOS)),
        text_color: GASTOS_TEXTO_BOTONES,
        border: Border::default(),
        ..Default::default()
    }
}

pub fn estilo_menu_selector(_theme: &Theme) -> menu::Style {
    menu::Style {
        text_color: GASTOS_TEXTO_SELECTOR,
        background: Background::Color(GASTOS_TEXTO_SELECTOR_FONDO),
        border: Border::default(),
        selected_text_color: GASTOS_TEXTO_SELECTOR_SELECCIONADO,
        selected_background: Background::Color(GASTOS_TEXTO_SELECTOR_FONDO_SELECCIONADO),
        shadow: Shadow::default(),
    }
}


// ==================================================
// ANÁLISIS
// ==================================================

pub const FONDO_ANALISIS: Color =
    Color::from_rgb8(67, 76, 94);

pub const ANALISIS_TARJETA: Color = Color::from_rgb8(59, 66, 82);
pub const ANALISIS_BORDE: Color = Color::from_rgb8(76, 86, 106);
pub const ANALISIS_TEXTO_PRINCIPAL: Color = Color::from_rgb8(236, 239, 244);
pub const ANALISIS_TEXTO_SECUNDARIO: Color = Color::from_rgb8(180, 190, 205);
pub const ANALISIS_ACENTO: Color = Color::from_rgb8(136, 192, 208);
pub const ANALISIS_ACENTO_SUAVE: Color = Color::from_rgb8(163, 190, 140);
pub const ANALISIS_BARRA_FONDO: Color = Color::from_rgb8(76, 86, 106);


// ==================================================
// CONFIGURACIÓN
// ==================================================

pub const FONDO_CONFIGURACION: Color =
    Color::from_rgb8(67, 76, 94);


// --------------------------------------------------
// Configuración - Botones
// --------------------------------------------------

pub const BOTONES_CONFIGURACION_AGREGAR: Color =
    Color::from_rgb8(94, 129, 172);

pub const TEXTO_CONFIGURACION_AGREGAR: Color =
    Color::from_rgb8(229, 233, 240);

pub const BOTONES_CONFIGURACION_ELIMINAR: Color =
    Color::from_rgb8(191, 97, 106);

pub const TEXTO_CONFIGURACION_ELIMINAR: Color =
    Color::from_rgb8(229, 233, 240);

pub fn estilo_boton_configuracion_agregar(
    _theme: &Theme,
    _status: button::Status,
) -> button::Style {
    button::Style {
        background: Some(Background::Color(BOTONES_CONFIGURACION_AGREGAR)),
        text_color: TEXTO_CONFIGURACION_AGREGAR,
        ..Default::default()
    }
}

pub fn estilo_boton_configuracion_eliminar(
    _theme: &Theme,
    _status: button::Status,
) -> button::Style {
    button::Style {
        background: Some(Background::Color(BOTONES_CONFIGURACION_ELIMINAR)),
        text_color: TEXTO_CONFIGURACION_ELIMINAR,
        ..Default::default()
    }
}


// --------------------------------------------------
// Configuración - Listas
// --------------------------------------------------

pub const BOTON_CONFIGURACION_LISTA: Color =
    Color::from_rgb8(59, 66, 82);

pub const BOTON_CONFIGURACION_LISTA_TEXTO: Color =
    Color::from_rgb8(229, 233, 240);
