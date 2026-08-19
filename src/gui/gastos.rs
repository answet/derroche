use iced::{Element, Task, Length, Alignment, Background, Color, Border, Shadow, Theme};
use iced::widget::{
    column,
    text,
    text_input,
    pick_list,
    button,
    container,
    row,
    scrollable,
    Space,
};
use iced::overlay::menu;

use chrono::{Datelike, NaiveDate, Local};
use crate::models::{Categoria, Persona, GastoDetalle};
use crate::estilos;

const DIAS: [u32; 31] = [
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
    11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
    31,
];

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

const ANIOS: [u32; 3] = [
    2026,
    2027,
    2028,
];

#[derive(Debug, Clone, Copy, Default)]
pub enum ModoFormulario {
    #[default]
    Cerrado,
    Nuevo,
    Editar(i32),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColumnaOrden {
    Descripcion,
    Monto,
    Persona,
    Categoria,
    Fecha,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DireccionOrden {
    Ascendente,
    Descendente,
}

#[derive(Debug, Clone)]
pub enum Message {
    DiaSeleccionado(u32),
    MesFechaSeleccionado(String),
    AnioSeleccionado(u32),

    MontoCambiado(String),
    DescripcionCambiada(String),
    CategoriaSeleccionada(Categoria),
    PersonaSeleccionada(Persona),

    Agregar,
    MostrarFormulario,
    CerrarFormulario,
    EditarGasto,
    EliminarGasto,

    GastoAgregado(Result<(), String>),
    GastoEliminado(Result<(), String>),

    GastosCargados(Result<Vec<GastoDetalle>, String>),

    MesSeleccionado(String),
    GastoSeleccionado(i32),

    GuardarEdicion,
    GastoEditado(Result<(), String>),

    OrdenarPor(ColumnaOrden),
}

pub struct Estado {
    pub fecha: String,
    pub monto: String,
    pub descripcion: String,

    pub categoria: Option<Categoria>,
    pub persona: Option<Persona>,

    pub categorias: Vec<Categoria>,
    pub personas: Vec<Persona>,

    pub gastos: Vec<GastoDetalle>,
    pub gasto_seleccionado: Option<i32>,

    pub mes: u32,
    pub anio: u32,

    pub error: Option<String>,

    pub modo_formulario: ModoFormulario,

    pub columna_orden: ColumnaOrden,
    pub direccion_orden: DireccionOrden,

    pub dia: u32,
    pub mes_fecha: u32,
    pub anio_fecha: u32,
}

impl Default for Estado {
    fn default() -> Self {
        let hoy = Local::now().date_naive();

        Self {
            fecha: hoy.format("%d-%m-%Y").to_string(),

            dia: hoy.day(),
            mes_fecha: hoy.month(),
            anio_fecha: hoy.year() as u32,

            monto: String::new(),
            descripcion: String::new(),

            categoria: None,
            persona: None,

            categorias: Vec::new(),
            personas: Vec::new(),

            gastos: Vec::new(),
            gasto_seleccionado: None,

            mes: hoy.month(),
            anio: hoy.year() as u32,

            error: None,

            modo_formulario: ModoFormulario::Cerrado,

            columna_orden: ColumnaOrden::Fecha,
            direccion_orden: DireccionOrden::Descendente,
        }
    }
}

pub fn update(estado: &mut Estado, mensaje: Message) -> Task<Message> {
    match mensaje {
        Message::MontoCambiado(monto) => {
            estado.monto = monto;
        }

        Message::DescripcionCambiada(descripcion) => {
            estado.descripcion = descripcion;
        }

        Message::CategoriaSeleccionada(categoria) => {
            estado.categoria = Some(categoria);
        }

        Message::PersonaSeleccionada(persona) => {
            estado.persona = Some(persona);
        }

        Message::Agregar => {
            match validar(estado) {
                Ok(()) => {
                    estado.error = None;

                    let categoria = estado.categoria.clone().unwrap();
                    let persona = estado.persona.clone().unwrap();

                    let descripcion = estado.descripcion.clone();
                    let monto: f64 = estado.monto.trim().parse().unwrap();
                    let fecha = estado.fecha.clone();

                    return Task::perform(
                        async move {
                            guardar_gasto(
                                descripcion,
                                monto,
                                fecha,
                                categoria.id,
                                persona.id,
                            )
                        },
                        Message::GastoAgregado,
                    );
                }

                Err(error) => {
                    estado.error = Some(error);
                }
            }
        }

        Message::GastoAgregado(resultado) => {
            match resultado {
                Ok(()) => {
                    estado.error = None;

                    reiniciar_formulario(estado);

                    estado.modo_formulario = ModoFormulario::Cerrado;

                    return Task::perform(
                        async {
                            cargar_gastos()
                        },
                        Message::GastosCargados,
                    );
                }

                Err(error) => {
                    estado.error = Some(error);
                }
            }
        }

        Message::MesSeleccionado(mes) => {
            if let Some(numero) = numero_mes(&mes) {
                estado.mes = numero;
                estado.gasto_seleccionado = None;
            }
        }

        Message::GastoSeleccionado(id) => {
            if estado.gasto_seleccionado == Some(id) {
                estado.gasto_seleccionado = None;
            } else {
                estado.gasto_seleccionado = Some(id);
            }
        }

        Message::MostrarFormulario => {
            estado.modo_formulario = ModoFormulario::Nuevo;

            estado.categoria = estado
                .categorias
                .iter()
                .find(|categoria| categoria.nombre == "Sin Categoria")
                .cloned();

            estado.persona = estado
                .personas
                .iter()
                .find(|persona| persona.nombre == "General")
                .cloned();

            estado.error = None;
        }

        Message::CerrarFormulario => {
            estado.modo_formulario = ModoFormulario::Cerrado;
            estado.error = None;

            estado.descripcion.clear();
            estado.monto.clear();
            estado.fecha.clear();
            estado.categoria = None;
            estado.persona = None;
        }

        Message::EliminarGasto => {
            if let Some(id) = estado.gasto_seleccionado {
                return Task::perform(
                    async move {
                        eliminar_gasto(id)
                    },
                    Message::GastoEliminado,
                );
            }
        }

        Message::GastoEliminado(resultado) => {
            match resultado {
                Ok(()) => {
                    estado.error = None;
                    estado.gasto_seleccionado = None;

                    return Task::perform(
                        async {
                            cargar_gastos()
                        },
                        Message::GastosCargados,
                    );
                }

                Err(error) => {
                    estado.error = Some(error);
                }
            }
        }

        Message::EditarGasto => {
            if let Some(id) = estado.gasto_seleccionado {
                if let Some(gasto) = estado.gastos.iter().find(|gasto| gasto.id == id) {
                    estado.descripcion = gasto.descripcion.clone();
                    estado.monto = gasto.monto.to_string();
                    estado.fecha = gasto.fecha.clone();

                    estado.categoria = estado
                        .categorias
                        .iter()
                        .find(|categoria| categoria.nombre == gasto.categoria)
                        .cloned();

                    estado.persona = estado
                        .personas
                        .iter()
                        .find(|persona| persona.nombre == gasto.persona)
                        .cloned();

                    estado.error = None;
                    estado.modo_formulario = ModoFormulario::Editar(id);
                }
            }
        }

        Message::GastosCargados(resultado) => {
            match resultado {
                Ok(gastos) => {
                    estado.gastos = gastos;
                }

                Err(error) => {
                    estado.error = Some(error);
                }
            }
        }

        Message::GuardarEdicion => {
            match validar(estado) {
                Ok(()) => {
                    estado.error = None;

                    let id = match estado.modo_formulario {
                        ModoFormulario::Editar(id) => id,
                        _ => return Task::none(),
                    };

                    let categoria = estado.categoria.clone().unwrap();
                    let persona = estado.persona.clone().unwrap();

                    let descripcion = estado.descripcion.clone();
                    let monto: f64 = estado.monto.trim().parse().unwrap();
                    let fecha = estado.fecha.clone();

                    return Task::perform(
                        async move {
                            actualizar_gasto(
                                id,
                                descripcion,
                                monto,
                                fecha,
                                categoria.id,
                                persona.id,
                            )
                        },
                        Message::GastoEditado,
                    );
                }

                Err(error) => {
                    estado.error = Some(error);
                }
            }
        }

        Message::GastoEditado(resultado) => {
            match resultado {
                Ok(()) => {
                    estado.error = None;

                    estado.descripcion.clear();
                    estado.monto.clear();
                    estado.fecha.clear();
                    estado.categoria = None;
                    estado.persona = None;

                    estado.gasto_seleccionado = None;
                    estado.modo_formulario = ModoFormulario::Cerrado;

                    return Task::perform(
                        async {
                            cargar_gastos()
                        },
                        Message::GastosCargados,
                    );
                }

                Err(error) => {
                    estado.error = Some(error);
                }
            }
        }

        Message::OrdenarPor(columna) => {
            if estado.columna_orden == columna {
                estado.direccion_orden = match estado.direccion_orden {
                    DireccionOrden::Ascendente => DireccionOrden::Descendente,
                    DireccionOrden::Descendente => DireccionOrden::Ascendente,
                };
            } else {
                estado.columna_orden = columna;
                estado.direccion_orden = DireccionOrden::Ascendente;
            }
        }

        Message::DiaSeleccionado(dia) => {
            estado.dia = dia;

            estado.fecha = format!(
                "{:02}-{:02}-{}",
                estado.dia,
                estado.mes_fecha,
                estado.anio_fecha
            );
        }

        Message::MesFechaSeleccionado(mes) => {
            if let Some(numero) = numero_mes(&mes) {
                estado.mes_fecha = numero;

                estado.fecha = format!(
                    "{:02}-{:02}-{}",
                    estado.dia,
                    estado.mes_fecha,
                    estado.anio_fecha
                );
            }
        }

        Message::AnioSeleccionado(anio) => {
            estado.anio_fecha = anio;

            estado.fecha = format!(
                "{:02}-{:02}-{}",
                estado.dia,
                estado.mes_fecha,
                estado.anio_fecha
            );
        }
    }

    Task::none()
}

fn formulario(estado: &Estado) -> Element<'_, Message> {
    let mensaje_error = match &estado.error {
        Some(error) => text(error),
        None => text(""),
    };

    let boton_guardar = match estado.modo_formulario {
        ModoFormulario::Nuevo => {
            button("Agregar gasto")
                .on_press(Message::Agregar)
                .style(|_theme, _status| button::Style {
                    background: Some(Background::Color(estilos::FONDO_BOTONES_GASTOS)),
                    text_color: estilos::GASTOS_TEXTO_BOTONES,
                    border: Border::default(),
                    ..Default::default()
                })
        }

        ModoFormulario::Editar(_) => {
            button("Guardar cambios")
                .on_press(Message::GuardarEdicion)
                .style(|_theme, _status| button::Style {
                    background: Some(Background::Color(estilos::FONDO_BOTONES_GASTOS)),
                    text_color: estilos::GASTOS_TEXTO_BOTONES,
                    border: Border::default(),
                    ..Default::default()
                })
        }

        ModoFormulario::Cerrado => {
            button("Cerrar")
        }
    };

    let contenido_formulario = column![
        row![
            pick_list(
                DIAS,
                Some(estado.dia),
                Message::DiaSeleccionado,
            )
            .menu_style(|_theme| menu::Style {
                text_color: estilos::GASTOS_TEXTO_SELECTOR,
                background: Background::Color(estilos::GASTOS_TEXTO_SELECTOR_FONDO),
                border: Border::default(),
                selected_text_color: estilos::GASTOS_TEXTO_SELECTOR_SELECCIONADO,
                selected_background: Background::Color(estilos::GASTOS_TEXTO_SELECTOR_FONDO_SELECCIONADO),
                shadow: Shadow::default(),
            }),

            pick_list(
                MESES,
                Some(nombre_mes(estado.mes_fecha)),
                |mes| Message::MesFechaSeleccionado(mes.to_string()),
            )
            .menu_style(|_theme| menu::Style {
                text_color: estilos::GASTOS_TEXTO_SELECTOR,
                background: Background::Color(estilos::GASTOS_TEXTO_SELECTOR_FONDO),
                border: Border::default(),
                selected_text_color: estilos::GASTOS_TEXTO_SELECTOR_SELECCIONADO,
                selected_background: Background::Color(estilos::GASTOS_TEXTO_SELECTOR_FONDO_SELECCIONADO),
                shadow: Shadow::default(),
            }),

            pick_list(
                ANIOS,
                Some(estado.anio_fecha),
                Message::AnioSeleccionado,
            )
            .menu_style(|_theme| menu::Style {
                text_color: estilos::GASTOS_TEXTO_SELECTOR,
                background: Background::Color(estilos::GASTOS_TEXTO_SELECTOR_FONDO),
                border: Border::default(),
                selected_text_color: estilos::GASTOS_TEXTO_SELECTOR_SELECCIONADO,
                selected_background: Background::Color(estilos::GASTOS_TEXTO_SELECTOR_FONDO_SELECCIONADO),
                shadow: Shadow::default(),
            }),

        ]
        .spacing(10),

        text_input("Monto", &estado.monto)
            .on_input(Message::MontoCambiado)
            .width(Length::Fixed(200.0)),

        text_input("Descripcion", &estado.descripcion)
            .on_input(Message::DescripcionCambiada)
            .width(Length::Fixed(200.0)),

        pick_list(
            estado.categorias.clone(),
            estado.categoria.clone(),
            Message::CategoriaSeleccionada,
        )
        .menu_style(|_theme| menu::Style {
            text_color: estilos::GASTOS_TEXTO_SELECTOR,
            background: Background::Color(estilos::GASTOS_TEXTO_SELECTOR_FONDO),
            border: Border::default(),
            selected_text_color: estilos::GASTOS_TEXTO_SELECTOR_SELECCIONADO,
            selected_background: Background::Color(estilos::GASTOS_TEXTO_SELECTOR_FONDO_SELECCIONADO),
            shadow: Shadow::default(),
        }),

        pick_list(
            estado.personas.clone(),
            estado.persona.clone(),
            Message::PersonaSeleccionada,
        )
        .menu_style(|_theme| menu::Style {
            text_color: estilos::GASTOS_TEXTO_SELECTOR,
            background: Background::Color(estilos::GASTOS_TEXTO_SELECTOR_FONDO),
            border: Border::default(),
            selected_text_color: estilos::GASTOS_TEXTO_SELECTOR_SELECCIONADO,
            selected_background: Background::Color(estilos::GASTOS_TEXTO_SELECTOR_FONDO_SELECCIONADO),
            shadow: Shadow::default(),
        }),

        mensaje_error,

        container(
            row![
                Space::new().width(Length::Fill),

                button("Cancelar")
                    .on_press(Message::CerrarFormulario)
                    .style(|_theme, _status| button::Style {
                        background: Some(Background::Color(estilos::FONDO_BOTONES_GASTOS)),
                        text_color: estilos::GASTOS_TEXTO_BOTONES,
                        border: Border::default(),
                        ..Default::default()
                    }),

                boton_guardar,

                Space::new().width(Length::Fill),
            ]
            .spacing(10),
        )
        .width(Length::Fixed(300.0))
        .padding(10)
        .style(|_theme| container::Style {
            background: Some(Background::Color(estilos::FONDO_BOTONES_GASTOS)),
            border: Border {
                color: estilos::BORDE_BOTONES_GASTOS,
                width: 1.0,
                radius: 14.0.into(),
            },
            ..Default::default()
        }),
    ]
    .width(Length::Fill)
    .spacing(10)
    .align_x(Alignment::Center);

    container(
        container(contenido_formulario)
            .width(Length::Fixed(500.0))
            .padding(30)
            .style(|_theme| container::Style {
                background: Some(Background::Color(estilos::FONDO_TABLA_GASTOS)),
                border: Border {
                    color: estilos::BORDE_TABLA_GASTOS,
                    width: 1.0,
                    radius: 14.0.into(),
                },
                ..Default::default()
            })
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(Background::Color(estilos::FONDO_GASTOS)),
            ..Default::default()
        })
        .into()
}

pub fn view(estado: &Estado) -> Element<'_, Message> {
    match estado.modo_formulario {
        ModoFormulario::Nuevo | ModoFormulario::Editar(_) => {
            formulario(estado)
        }

        ModoFormulario::Cerrado => {
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

            let botones = if estado.gasto_seleccionado.is_some() {
                row![
                    Space::new().width(Length::Fill),
                    button("Editar")
                        .on_press(Message::EditarGasto)
                        .style(|_theme, _status| button::Style {
                            background: Some(Background::Color(estilos::FONDO_BOTONES_GASTOS)),
                            text_color: estilos::GASTOS_TEXTO_BOTONES,
                            border: Border::default(),
                            ..Default::default()
                        }),
                    button("Eliminar")
                        .on_press(Message::EliminarGasto)
                        .style(|_theme, _status| button::Style {
                            background: Some(Background::Color(estilos::FONDO_BOTONES_GASTOS)),
                            text_color: estilos::GASTOS_TEXTO_BOTONES,
                            border: Border::default(),
                            ..Default::default()
                        }),
                   Space::new().width(Length::Fill),
                ]
                .spacing(10)
            } else {
                row![
                    Space::new().width(Length::Fill),
                    button("+ Agregar gasto")
                        .on_press(Message::MostrarFormulario)
                        .style(|_theme, _status| button::Style {
                            background: Some(Background::Color(estilos::FONDO_BOTONES_GASTOS)),
                            text_color: estilos::GASTOS_TEXTO_BOTONES,
                            border: Border::default(),
                            ..Default::default()
                        }),
                   Space::new().width(Length::Fill),
                ]
            };

            let centro = column![
                Space::new().height(Length::Fill),
                container(tabla_gastos(estado))
                    .width(Length::Fill)
                    .height(Length::Fixed(500.0))
                    .padding(10)
                    .style(|_theme| container::Style {
                        background: Some(Background::Color(estilos::FONDO_TABLA_GASTOS)),
                        border: Border {
                            color: estilos::BORDE_TABLA_GASTOS,
                            width: 1.0,
                            radius: 14.0.into(),
                        },
                        ..Default::default()
                    }),

                container(botones)
                    .width(Length::Fixed(200.0))
                    .padding(10)
                    .style(|_theme| container::Style {
                        background: Some(Background::Color(estilos::FONDO_BOTONES_GASTOS)),
                        border: Border {
                            color: estilos::BORDE_BOTONES_GASTOS,
                            width: 1.0,
                            radius: 14.0.into(),
                        },
                        ..Default::default()
                    }),

                    Space::new().height(Length::Fill),
            ]
            .spacing(20)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center);

            let contenido = row![
                column![
                    selector_mes,
                    container(Space::new())
                        .width(Length::Fixed(165.0))
                        .height(Length::Fixed(1.0))
                        .style(|_theme| container::Style {
                            background: Some(Background::Color(estilos::GASTOS_TEXTO_MES)),
                            ..Default::default()
                        }),
                ]
                .spacing(3),

                container(centro)
                    .width(Length::Fill)
                    .height(Length::Fill),
            ];

            container(contenido)
                .padding(30)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(|_theme| container::Style {
                    background: Some(Background::Color(estilos::FONDO_GASTOS)),
                    ..Default::default()
                })
                .into()
        }
    }
}

fn validar(estado: &Estado) -> Result<(), String> {
    if estado.fecha.trim().is_empty() {
        return Err("La fecha es obligatoria".to_string());
    }

    if NaiveDate::parse_from_str(&estado.fecha, "%d-%m-%Y").is_err() {
        return Err("La fecha debe tener el formato DD-MM-AAAA".to_string());
    }

    if estado.monto.trim().is_empty() {
        return Err("El monto es obligatorio".to_string());
    }

    let monto: f64 = match estado.monto.trim().parse() {
        Ok(monto) => monto,
        Err(_) => { return Err("El monto debe ser un numero valido".to_string()); }
    };

    if monto <= 0.0 {
        return Err("El monto debe ser mayor a 0".to_string());
    }

    if estado.descripcion.trim().is_empty() {
        return Err("La descripcion es obligatoria".to_string());
    }

    if estado.categoria.is_none() {
        return Err("Debes seleccionar una categoria".to_string());
    }

    if estado.persona.is_none() {
        return Err("Debes seleccionar una persona".to_string());
    }

    Ok(())
}

fn guardar_gasto(
    descripcion: String,
    monto: f64,
    fecha: String,
    categoria_id: i32,
    persona_id: i32,
) -> Result<(), String> {
    let conexion = crate::database::conectar()
        .map_err(|error| error.to_string())?;

    crate::repository::agregar_gasto(
        &conexion,
        &descripcion,
        monto,
        &fecha,
        categoria_id,
        persona_id,
    )
    .map_err(|error| error.to_string())?;

    Ok(())
}

fn eliminar_gasto(id: i32) -> Result<(), String> {
    let conexion = crate::database::conectar()
        .map_err(|error| error.to_string())?;

    crate::repository::eliminar_gasto(&conexion, id)
        .map_err(|error| error.to_string())?;

    Ok(())
}

fn actualizar_gasto(
    id: i32,
    descripcion: String,
    monto: f64,
    fecha: String,
    categoria_id: i32,
    persona_id: i32,
) -> Result<(), String> {
    let conexion =
        crate::database::conectar()
            .map_err(|error| error.to_string())?;

    crate::repository::actualizar_gasto(
        &conexion,
        id,
        &descripcion,
        monto,
        &fecha,
        categoria_id,
        persona_id,
    )
    .map_err(|error| error.to_string())
}

fn cargar_gastos() -> Result<Vec<GastoDetalle>, String> {
    let conexion =
        crate::database::conectar()
            .map_err(|error| error.to_string())?;

    crate::repository::obtener_gastos_detalle(&conexion)
        .map_err(|error| error.to_string())
}

fn tabla_gastos<'a>(estado: &'a Estado) -> Element<'a, Message> {
    let mut indices: Vec<usize> = estado
        .gastos
        .iter()
        .enumerate()
        .filter_map(|(indice, gasto)| {
            let fecha = NaiveDate::parse_from_str(
                &gasto.fecha,
                "%d-%m-%Y",
            )
            .ok()?;

            if fecha.month() == estado.mes
                && fecha.year() as u32 == estado.anio
            {
                Some(indice)
            } else {
                None
            }
        })
        .collect();

    indices.sort_by(|&a, &b| {
        let gasto_a = &estado.gastos[a];
        let gasto_b = &estado.gastos[b];

        let orden = match estado.columna_orden {
            ColumnaOrden::Descripcion => {
                gasto_a.descripcion.cmp(&gasto_b.descripcion)
            }

            ColumnaOrden::Monto => {
                gasto_a
                    .monto
                    .partial_cmp(&gasto_b.monto)
                    .unwrap_or(std::cmp::Ordering::Equal)
            }

            ColumnaOrden::Persona => {
                gasto_a.persona.cmp(&gasto_b.persona)
            }

            ColumnaOrden::Categoria => {
                gasto_a.categoria.cmp(&gasto_b.categoria)
            }

            ColumnaOrden::Fecha => {
                let fecha_a = NaiveDate::parse_from_str(
                    &gasto_a.fecha,
                    "%d-%m-%Y",
                );

                let fecha_b = NaiveDate::parse_from_str(
                    &gasto_b.fecha,
                    "%d-%m-%Y",
                );

                fecha_a.into_iter().cmp(fecha_b)
            }
        };

        match estado.direccion_orden {
            DireccionOrden::Ascendente => orden,
            DireccionOrden::Descendente => orden.reverse(),
        }
    });

    let encabezado = row![
        button(
            text(titulo_columna(
                "Descripcion",
                ColumnaOrden::Descripcion,
                estado,
            ))
            .align_x(Alignment::Center)
        )
        .on_press(Message::OrdenarPor(ColumnaOrden::Descripcion))
        .width(Length::FillPortion(3))
        .style(estilo_encabezado),

        button(
            text(titulo_columna(
                "Monto",
                ColumnaOrden::Monto,
                estado,
            ))
            .align_x(Alignment::Center)
        )
        .on_press(Message::OrdenarPor(ColumnaOrden::Monto))
        .width(Length::FillPortion(1))
        .style(estilo_encabezado),

        button(
            text(titulo_columna(
                "Persona",
                ColumnaOrden::Persona,
                estado,
            ))
            .align_x(Alignment::Center)
        )
        .on_press(Message::OrdenarPor(ColumnaOrden::Persona))
        .width(Length::FillPortion(1))
        .style(estilo_encabezado),

        button(
            text(titulo_columna(
                "Categoria",
                ColumnaOrden::Categoria,
                estado,
            ))
            .align_x(Alignment::Center)
        )
        .on_press(Message::OrdenarPor(ColumnaOrden::Categoria))
        .width(Length::FillPortion(1))
        .style(estilo_encabezado),

        button(
            text(titulo_columna(
                "Fecha",
                ColumnaOrden::Fecha,
                estado,
            ))
            .align_x(Alignment::Center)
        )
        .on_press(Message::OrdenarPor(ColumnaOrden::Fecha))
        .width(Length::FillPortion(1))
        .style(estilo_encabezado),
    ]
    .spacing(10);

    let mut filas = column![]
        .spacing(5);

    for indice in indices {
        let gasto = &estado.gastos[indice];

        let seleccionado = estado.gasto_seleccionado == Some(gasto.id);

        let fila = row![
            text(&gasto.descripcion)
                .width(Length::FillPortion(3))
                .align_x(Alignment::Center),

            text(formatear_monto(gasto.monto))
                .width(Length::FillPortion(1))
                .align_x(Alignment::Center),

            text(&gasto.persona)
                .width(Length::FillPortion(1))
                .align_x(Alignment::Center),

            text(&gasto.categoria)
                .width(Length::FillPortion(1))
                .align_x(Alignment::Center),

            text(&gasto.fecha)
                .width(Length::FillPortion(1))
                .align_x(Alignment::Center),
        ]
        .spacing(5)
        .height(Length::Fixed(36.0));

        let fila = button(fila)
            .on_press(Message::GastoSeleccionado(gasto.id))
            .style(move |_theme, _status| {
                if seleccionado {
                    button::Style {
                        background: Some(Background::Color(estilos::FONDO_FILA_SELECCIONADA)),
                        text_color: estilos::TEXTO_FILA_SELECCIONADA,
                        ..Default::default()
                    }
                } else {
                    button::Style {
                        background: Some(Background::Color(estilos::FONDO_TABLA_GASTOS)),
                        text_color: estilos::GASTOS_TEXTO_TABLA,
                        ..Default::default()
                    }
                }
            });

        filas = filas.push(fila);
    }

    container(
        column![
            encabezado,

            scrollable(filas)
                  .height(Length::Fill),
        ]
        .spacing(10),
    )
    .width(Length::Fill)
    .into()
}

fn titulo_columna(
    nombre: &str,
    columna: ColumnaOrden,
    estado: &Estado,
) -> String {
    if estado.columna_orden == columna {
        match estado.direccion_orden {
            DireccionOrden::Ascendente => {
                format!("{} ↑", nombre)
            }

            DireccionOrden::Descendente => {
                format!("{} ↓", nombre)
            }
        }
    } else {
        nombre.to_string()
    }
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
        _ => Some(0),
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

fn reiniciar_formulario(estado: &mut Estado) {
    let hoy = Local::now().date_naive();

    estado.fecha =
        hoy.format("%d-%m-%Y").to_string();

    estado.dia = hoy.day();
    estado.mes_fecha = hoy.month();
    estado.anio_fecha = hoy.year() as u32;

    estado.monto.clear();
    estado.descripcion.clear();

    estado.categoria =
        estado.categorias
            .iter()
            .find(|categoria| categoria.nombre == "Sin Categoria")
            .cloned();

    estado.persona =
        estado.personas
            .iter()
            .find(|persona| persona.nombre == "General")
            .cloned();
}

fn estilo_encabezado(
    _theme: &Theme,
    _status: button::Status,
) -> button::Style {
    button::Style {
        background: Some(Background::Color(estilos::BORDE_TABLA_GASTOS)),
        text_color: estilos::GASTOS_TEXTO_CABEZA_TABLA,
        border: Border::default(),
        ..Default::default()
    }
}
