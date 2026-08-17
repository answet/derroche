use iced::{
    Alignment,
    Element,
    Length,
    Background,
    Color,
    Border,
    Shadow,
    widget::{column, container, row, text, pick_list},
    overlay::menu,
};

use crate::models::{GastoDetalle, GastoPorCategoria, TotalMensual, GastoPorPersona};
use crate::estilos;

const MESES: [&str; 12] = [
    "Enero",
    "Febrero",
    "Marzo",
    "Abril",
    "Mayo",
    "Junio",
    "Julio",
    "Agosto",
    "Septiembre",
    "Octubre",
    "Noviembre",
    "Diciembre",
];

#[derive(Debug, Clone)]
pub enum Message {
    MesSeleccionado(String),
}

pub struct Estado {
    pub mes: u32,
    pub anio: i32,

    pub total_mes: f64,
    pub total_mes_anterior: f64,
    pub diferencia_mes: f64,

    pub mayor_gasto: Option<GastoDetalle>,
    pub totales_mensuales: Vec<TotalMensual>,
    pub gastos_por_categoria: Vec<GastoPorCategoria>,
    pub gastos_por_persona: Vec<GastoPorPersona>,
}

impl Default for Estado {
    fn default() -> Self {
        Self {
            mes: 8,
            anio: 2026,
            total_mes: 0.0,
            total_mes_anterior: 0.0,
            diferencia_mes: 0.0,
            mayor_gasto: None,
            totales_mensuales: Vec::new(),
            gastos_por_categoria: Vec::new(),
            gastos_por_persona: Vec::new(),
        }
    }
}

pub fn update(estado: &mut Estado, mensaje: Message) {
    match mensaje {
        Message::MesSeleccionado(mes) => {
            if let Some(numero) = numero_mes(&mes) {
                estado.mes = numero;
            }
        }
    }
}

pub fn view(estado: &Estado) -> Element<'_, Message> {
    let selector_mes = pick_list(
        MESES,
        Some(nombre_mes(estado.mes)),
        |mes| Message::MesSeleccionado(mes.to_string()),
    )
    .width(Length::Fixed(165.0))
    .text_size(24)
    .style(|_theme, _status| pick_list::Style {
        text_color: estilos::GASTOS_TEXTO_MES,
        background: Background::Color(Color::TRANSPARENT),
        border: Border::default(),
        placeholder_color: estilos::GASTOS_TEXTO_MES,
        handle_color: estilos::GASTOS_TEXTO_MES,
    })
    .menu_style(|_theme| menu::Style {
        text_color: estilos::GASTOS_TEXTO_SELECTOR,
        background: Background::Color(estilos::GASTOS_TEXTO_SELECTOR_FONDO),
        border: Border::default(),
        selected_text_color: estilos::GASTOS_TEXTO_SELECTOR_SELECCIONADO,
        selected_background: Background::Color(estilos::GASTOS_TEXTO_SELECTOR_FONDO_SELECCIONADO),
        shadow: Shadow::default(),
    });

    let mayor_gasto = match &estado.mayor_gasto {
        Some(gasto) => {
            column![
                text(formatear_monto(gasto.monto)).size(24),
                text(&gasto.descripcion).size(14),
            ]
            .spacing(4)
        }

        None => {
            column![
                text("$0.00"),
                text("Sin gastos").size(14),
            ]
            .spacing(4)
        }
    };

    let resumen = container(
        row![
            tarjeta(
                "Total gastado",
                formatear_monto(estado.total_mes),
            ),

            tarjeta_elemento(
                "Mayor gasto".to_string(),
                mayor_gasto.into(),
            ),

            tarjeta(
                "Diferencia",
                format!("{:+.1}%", estado.diferencia_mes),
            ),
        ]
        .spacing(20)
    )
    .width(Length::Fill)
    .center_x(Length::Fill);

    let evolucion = grafico_evolucion(estado);

    let categorias = grafico_categorias(estado);

    let personas = grafico_personas(estado);

    container(
        column![
            row![
                container(
                    column![
                        container(selector_mes)
                            .height(Length::Fill)
                            .center_x(Length::Fill)
                            .center_y(Length::Fill),

                        container(resumen)
                            .height(Length::Fill)
                            .center_x(Length::Fill)
                            .center_y(Length::Fill),
                    ]
                    .spacing(20)
                )
                .width(Length::Fill)
                .height(Length::Fill)
            ]
            .width(Length::Fill)
            .height(Length::Fill),

            row![
                container(evolucion)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill),

                container(
                    column![
                        categorias,
                        personas,
                    ]
                    .spacing(50)
                    .align_x(Alignment::Center)
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill),
            ]
            .width(Length::Fill)
            .height(Length::Fill),
        ]
        .width(Length::Fill)
        .spacing(30)
        .align_x(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(10)
    .style(|_theme| container::Style {
        background: Some(Background::Color(estilos::FONDO_ANALISIS)),
        ..Default::default()
    })
    .into()
}

fn tarjeta(titulo: &str, valor: String) -> Element<'_, Message> {
    container(
        column![
            text(titulo).size(14),
            text(valor).size(24),
        ]
        .spacing(8),
    )
    .into()
}

fn tarjeta_elemento(titulo: String, contenido: Element<'_, Message>) -> Element<'_, Message> {
    container(
        column![
            text(titulo).size(14),
            contenido,
        ]
        .spacing(8),
    )
    .into()
}

fn numero_mes(nombre: &str) -> Option<u32> {
    match nombre {
        "Enero" => Some(1),
        "Febrero" => Some(2),
        "Marzo" => Some(3),
        "Abril" => Some(4),
        "Mayo" => Some(5),
        "Junio" => Some(6),
        "Julio" => Some(7),
        "Agosto" => Some(8),
        "Septiembre" => Some(9),
        "Octubre" => Some(10),
        "Noviembre" => Some(11),
        "Diciembre" => Some(12),
        _ => None,
    }
}

fn nombre_mes(numero: u32) -> &'static str {
    match numero {
        1 => "Enero",
        2 => "Febrero",
        3 => "Marzo",
        4 => "Abril",
        5 => "Mayo",
        6 => "Junio",
        7 => "Julio",
        8 => "Agosto",
        9 => "Septiembre",
        10 => "Octubre",
        11 => "Noviembre",
        12 => "Diciembre",
        _ => "Agosto",
    }
}

fn totales_del_anio(
    totales: &[TotalMensual],
    anio: i32,
) -> Vec<f64> {
    (1..=12)
        .map(|mes| {
            totales
                .iter()
                .find(|item| {
                    item.anio == anio
                        && item.mes == mes
                })
                .map(|item| item.total)
                .unwrap_or(0.0)
        })
        .collect()
}

fn grafico_evolucion(
    estado: &Estado,
) -> Element<'_, Message> {
    let valores = totales_del_anio(
        &estado.totales_mensuales,
        estado.anio,
    );

    let maximo = valores
        .iter()
        .copied()
        .fold(0.0_f64, f64::max);

    let mut barras = row![
    ]
    .spacing(12)
    .align_y(Alignment::End);

    for (indice, valor) in valores.iter().enumerate() {
        let mes = (indice + 1) as u32;

        let altura: f32 = if maximo > 0.0 {
            ((valor / maximo) * 220.0).max(4.0) as f32
        } else {
            4.0
        };

        let barra = container(
            text("")
        )
        .width(Length::Fixed(28.0))
        .height(Length::Fixed(altura))
        .style(if mes == estado.mes {
            |_: &iced::Theme| container::Style {
                background: Some(Background::Color(
                    estilos::EVO_MES_SELECCIONADO,
                )),
                ..Default::default()
            }
        } else {
            |_: &iced::Theme| container::Style {
                background: Some(Background::Color(
                    estilos::EVO_MES,
                )),
                ..Default::default()
            }
        });

        let monto = if *valor > 0.0 {
            text(format!("${:.0}", valor))
                .size(11)
        } else {
            text("")
                .size(11)
        };

        barras = barras.push(
            column![
                monto,
                barra,
                text(nombre_mes(mes)).size(12),
            ]
            .align_x(Alignment::Center)
            .spacing(8)
        );
    }

    container(
        column![
            text("Evolución del gasto").size(20),
            container(barras)
                .height(Length::Fixed(280.0))
                .width(Length::Fill)
                .align_y(Alignment::End),
        ]
        .spacing(20)
    )
    .width(Length::Fixed(500.0))
    .into()
}

fn grafico_categorias(
    estado: &Estado,
) -> Element<'_, Message> {
    if estado.gastos_por_categoria.is_empty() {
        return container(
            column![
                text("Gasto por categoría").size(20),
                text("Sin gastos")
            ]
            .spacing(15)
        )
        .width(Length::Fill)
        .into();
    }

    let maximo = estado
        .gastos_por_categoria
        .iter()
        .map(|item| item.total)
        .fold(0.0_f64, f64::max);

    let mut filas = column![]
        .spacing(12);

    for item in &estado.gastos_por_categoria {
        let ancho: f32 = if maximo > 0.0 {
            ((item.total / maximo) * 220.0) as f32
        } else {
            0.0
        };

        let barra = container(text(""))
            .width(Length::Fixed(ancho))
            .height(Length::Fixed(18.0))
            .style(|_: &iced::Theme| container::Style {
                background: Some(
                    Background::Color(
                        Color::from_rgb(
                            0.75,
                            0.75,
                            0.75,
                        ),
                    )
                ),
                ..Default::default()
            });

        filas = filas.push(
            row![
                text(&item.categoria)
                    .width(Length::Fixed(100.0)),

                barra,

                text(format!("${:.0}", item.total))
                    .size(12),
            ]
            .spacing(10)
            .align_y(Alignment::Center)
        );
    }

    container(
        column![
            text("Gasto por categoría").size(20),
            filas,
        ]
        .spacing(20)
    )
    .width(Length::Fixed(400.0))
    .into()
}

fn grafico_personas(
    estado: &Estado,
) -> Element<'_, Message> {
    if estado.gastos_por_persona.is_empty() {
        return container(
            column![
                text("Gasto por persona").size(20),
                text("Sin gastos")
            ]
            .spacing(15)
        )
        .width(Length::Fill)
        .into();
    }

    let maximo = estado
        .gastos_por_persona
        .iter()
        .map(|item| item.total)
        .fold(0.0_f64, f64::max);

    let mut filas = column![]
        .spacing(12);

    for item in &estado.gastos_por_persona {
        let ancho: f32 = if maximo > 0.0 {
            ((item.total / maximo) * 220.0) as f32
        } else {
            0.0
        };

        let barra = container(text(""))
            .width(Length::Fixed(ancho))
            .height(Length::Fixed(18.0))
            .style(|_: &iced::Theme| container::Style {
                background: Some(
                    Background::Color(
                        Color::from_rgb(
                            0.75,
                            0.75,
                            0.75,
                        ),
                    )
                ),
                ..Default::default()
            });

        filas = filas.push(
            row![
                text(&item.persona)
                    .width(Length::Fixed(100.0)),

                barra,

                text(format!("${:.0}", item.total))
                    .size(12),
            ]
            .spacing(10)
            .align_y(Alignment::Center)
        );
    }

    container(
        column![
            text("Gasto por persona").size(20),
            filas,
        ]
        .spacing(20)
    )
    .width(Length::Fixed(400.0))
    .into()
}

fn formatear_monto(monto: f64) -> String {
    let negativo = monto < 0.0;
    let monto = monto.abs();

    let parte_entera = monto.trunc() as u64;
    let parte_decimal = ((monto.fract() * 100.0).round()) as u64;

    let digitos = parte_entera.to_string();
    let mut entero_formateado = String::new();

    for (i, caracter) in digitos.chars().enumerate() {
        if i > 0 && (digitos.len() - i) % 3 == 0 {
            entero_formateado.push('.');
        }

        entero_formateado.push(caracter);
    }

    let signo = if negativo { "-" } else { "" };

    if parte_decimal == 0 {
        format!("{}${}", signo, entero_formateado)
    } else {
        format!(
            "{}${},{:02}",
            signo,
            entero_formateado,
            parte_decimal
        )
    }
}
