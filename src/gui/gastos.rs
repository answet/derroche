use iced::{Element, Task, Length, Alignment, Background, Color, Border, Theme};
use iced::widget::{
    column,
    text,
    text_input,
    pick_list,
    button,
    container,
    row,
    responsive,
    scrollable,
    Space,
};

use chrono::{Datelike, NaiveDate, Local};
use crate::formato::{anios_alrededor, formatear_monto, nombre_mes, numero_mes, MESES};
use crate::models::{Categoria, Persona, GastoDetalle};
use crate::estilos;

const DIAS: [u32; 31] = [
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
    11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
    31,
];

#[derive(Debug, Clone, Copy, Default)]
pub enum ModoFormulario {
    #[default]
    Cerrado,
    Nuevo,
    Editar(i32),
    ConfirmarEliminacion,
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
    SolicitarEliminarGasto,
    ConfirmarEliminarGasto,
    CancelarEliminarGasto,
    DeseleccionarGasto,

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
                Ok(monto) => {
                    estado.error = None;

                    let categoria = estado.categoria.clone().unwrap();
                    let persona = estado.persona.clone().unwrap();

                    let descripcion = estado.descripcion.clone();
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
            estado.gasto_seleccionado = Some(id);
        }

        Message::DeseleccionarGasto => {
            estado.gasto_seleccionado = None;
        }

        Message::MostrarFormulario => {
            estado.modo_formulario = ModoFormulario::Nuevo;
            reiniciar_formulario(estado);
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

        Message::SolicitarEliminarGasto => {
            if estado.gasto_seleccionado.is_some() {
                estado.modo_formulario = ModoFormulario::ConfirmarEliminacion;
            }
        }

        Message::CancelarEliminarGasto => {
            estado.modo_formulario = ModoFormulario::Cerrado;
        }

        Message::ConfirmarEliminarGasto => {
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
            if let Some(id) = estado.gasto_seleccionado
                && let Some(gasto) = estado.gastos.iter().find(|gasto| gasto.id == id).cloned()
            {
                    estado.descripcion = gasto.descripcion.clone();
                    estado.monto = gasto.monto.to_string();
                    estado.fecha = gasto.fecha.clone();

                    if let Err(error) = sincronizar_selectores_fecha(estado, &gasto.fecha) {
                        estado.error = Some(error);
                        return Task::none();
                    }

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
                Ok(monto) => {
                    estado.error = None;

                    let id = match estado.modo_formulario {
                        ModoFormulario::Editar(id) => id,
                        _ => return Task::none(),
                    };

                    let categoria = estado.categoria.clone().unwrap();
                    let persona = estado.persona.clone().unwrap();

                    let descripcion = estado.descripcion.clone();
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
            actualizar_fecha(estado);
        }

        Message::MesFechaSeleccionado(mes) => {
            if let Some(numero) = numero_mes(&mes) {
                estado.mes_fecha = numero;
                actualizar_fecha(estado);
            }
        }

        Message::AnioSeleccionado(anio) => {
            estado.anio_fecha = anio;
            actualizar_fecha(estado);
        }
    }

    Task::none()
}

fn formulario(estado: &Estado) -> Element<'_, Message> {
    let anios = anios_alrededor(estado.anio_fecha as i32)
        .into_iter()
        .filter_map(|anio| u32::try_from(anio).ok())
        .collect::<Vec<_>>();

    let mensaje_error = match &estado.error {
        Some(error) => text(error).color(estilos::TEXTO_ERROR),
        None => text(""),
    };

    let boton_guardar = match estado.modo_formulario {
        ModoFormulario::Nuevo => {
            button("Agregar")
                .on_press(Message::Agregar)
                .style(estilos::estilo_boton_gastos)
        }

        ModoFormulario::Editar(_) => {
            button("Guardar")
                .on_press(Message::GuardarEdicion)
                .style(estilos::estilo_boton_gastos)
        }

        ModoFormulario::Cerrado => {
            button("Cerrar")
        }

        ModoFormulario::ConfirmarEliminacion => {
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
            .style(|_theme, _status| pick_list::Style {
                text_color: estilos::GASTOS_TEXTO_SELECTOR_SELECCIONADO,
                background: Background::Color(estilos::FONDO_GASTOS),
                placeholder_color: estilos::GASTOS_TEXTO_SELECTOR,
                handle_color: estilos::GASTOS_TEXTO_SELECTOR,
                border: Border {
                    color: estilos::BORDE_TABLA_GASTOS,
                    width: 1.0,
                    radius: 8.0.into(),
                }
            })
            .menu_style(estilos::estilo_menu_selector),

            pick_list(
                MESES,
                nombre_mes(estado.mes_fecha),
                |mes| Message::MesFechaSeleccionado(mes.to_string()),
            )
            .style(|_theme, _status| pick_list::Style {
                text_color: estilos::GASTOS_TEXTO_SELECTOR_SELECCIONADO,
                background: Background::Color(estilos::FONDO_GASTOS),
                placeholder_color: estilos::GASTOS_TEXTO_SELECTOR,
                handle_color: estilos::GASTOS_TEXTO_SELECTOR,
                border: Border {
                    color: estilos::BORDE_TABLA_GASTOS,
                    width: 1.0,
                    radius: 8.0.into(),
                }
            })
            .menu_style(estilos::estilo_menu_selector),

            pick_list(
                anios,
                Some(estado.anio_fecha),
                Message::AnioSeleccionado,
            )
            .style(|_theme, _status| pick_list::Style {
                text_color: estilos::GASTOS_TEXTO_SELECTOR_SELECCIONADO,
                background: Background::Color(estilos::FONDO_GASTOS),
                placeholder_color: estilos::GASTOS_TEXTO_SELECTOR,
                handle_color: estilos::GASTOS_TEXTO_SELECTOR,
                border: Border {
                    color: estilos::BORDE_TABLA_GASTOS,
                    width: 1.0,
                    radius: 8.0.into(),
                }
            })
            .menu_style(estilos::estilo_menu_selector),

        ]
        .spacing(10),

        text_input("Monto", &estado.monto)
            .on_input(Message::MontoCambiado)
            .width(Length::Fixed(200.0))
            .style(|_theme, _status| text_input::Style {
                background: Background::Color(estilos::FONDO_GASTOS),
                border: Border {
                    color: estilos::BORDE_TABLA_GASTOS,
                    width: 1.0,
                    radius: 8.0.into(),
                },
                icon: estilos::GASTOS_TEXTO_SELECTOR_SELECCIONADO,
                placeholder: estilos::GASTOS_TEXTO_SELECTOR_SELECCIONADO,
                value: estilos::GASTOS_TEXTO_SELECTOR_SELECCIONADO,
                selection: estilos::GASTOS_TEXTO_SELECTOR,
            }),

        text_input("Descripcion", &estado.descripcion)
            .on_input(Message::DescripcionCambiada)
            .width(Length::Fixed(200.0))
            .style(|_theme, _status| text_input::Style {
                background: Background::Color(estilos::FONDO_GASTOS),
                border: Border {
                    color: estilos::BORDE_TABLA_GASTOS,
                    width: 1.0,
                    radius: 8.0.into(),
                },
                icon: estilos::GASTOS_TEXTO_SELECTOR_SELECCIONADO,
                placeholder: estilos::GASTOS_TEXTO_SELECTOR_SELECCIONADO,
                value: estilos::GASTOS_TEXTO_SELECTOR_SELECCIONADO,
                selection: estilos::GASTOS_TEXTO_SELECTOR,
            }),

        pick_list(
            estado.categorias.as_slice(),
            estado.categoria.as_ref(),
            Message::CategoriaSeleccionada,
        )
        .style(|_theme, _status| pick_list::Style {
            text_color: estilos::GASTOS_TEXTO_SELECTOR_SELECCIONADO,
            background: Background::Color(estilos::FONDO_GASTOS),
            placeholder_color: estilos::GASTOS_TEXTO_SELECTOR,
            handle_color: estilos::GASTOS_TEXTO_SELECTOR,
            border: Border {
                    color: estilos::BORDE_TABLA_GASTOS,
                    width: 1.0,
                    radius: 8.0.into(),
                }
        })
            .menu_style(estilos::estilo_menu_selector),

        pick_list(
            estado.personas.as_slice(),
            estado.persona.as_ref(),
            Message::PersonaSeleccionada,
        )
        .style(|_theme, _status| pick_list::Style {
            text_color: estilos::GASTOS_TEXTO_SELECTOR_SELECCIONADO,
            background: Background::Color(estilos::FONDO_GASTOS),
            placeholder_color: estilos::GASTOS_TEXTO_SELECTOR,
            handle_color: estilos::GASTOS_TEXTO_SELECTOR,
            border: Border {
                    color: estilos::BORDE_TABLA_GASTOS,
                    width: 1.0,
                    radius: 8.0.into(),
                }
        })
            .menu_style(estilos::estilo_menu_selector),

        mensaje_error,

        container(
            row![
                Space::new().width(Length::Fill),

                button("Cancelar")
                    .on_press(Message::CerrarFormulario)
                    .style(estilos::estilo_boton_gastos),

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

        ModoFormulario::ConfirmarEliminacion => confirmar_eliminacion(estado),

        ModoFormulario::Cerrado => {
            let selector_mes = pick_list(
                MESES,
                nombre_mes(estado.mes),
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
            .menu_style(estilos::estilo_menu_selector);

            let botones = if estado.gasto_seleccionado.is_some() {
                row![
                    button("Editar")
                        .on_press(Message::EditarGasto)
                        .style(estilos::estilo_boton_gastos),

                    button("Eliminar")
                        .on_press(Message::SolicitarEliminarGasto)
                        .style(estilos::estilo_boton_gastos),

                    button("Cancelar selección")
                        .on_press(Message::DeseleccionarGasto)
                        .style(estilos::estilo_boton_gastos),
                ]
                .spacing(10)
            } else {
                row![
                    button("+ Agregar Gasto")
                        .on_press(Message::MostrarFormulario)
                        .style(estilos::estilo_boton_gastos),
                ]
            };

            let contenido = column![
                selector_mes,

                container(
                    responsive(move |tamano| tabla_gastos(estado, tamano.width))
                        .width(Length::Fill)
                        .height(Length::Fill),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                    .padding(10)
                    .style(|_theme| container::Style {
                        background: Some(
                            Background::Color(
                                estilos::FONDO_TABLA_GASTOS
                            )
                        ),
                        border: Border {
                            color: estilos::BORDE_TABLA_GASTOS,
                            width: 1.0,
                            radius: 14.0.into(),
                        },
                        ..Default::default()
                    }),

                container(botones)
                    .width(Length::Shrink)
                    .padding(10)
                    .style(|_theme| container::Style {
                        background: Some(
                            Background::Color(
                                estilos::FONDO_BOTONES_GASTOS
                            )
                        ),
                        border: Border {
                            color: estilos::BORDE_BOTONES_GASTOS,
                            width: 1.0,
                            radius: 14.0.into(),
                        },
                        ..Default::default()
                    }),
            ]
            .spacing(20)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center);

            container(contenido)
                .padding(30)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(|_theme| container::Style {
                    background: Some(
                        Background::Color(estilos::FONDO_GASTOS)
                    ),
                    ..Default::default()
                })
                .into()
        }
    }
}

fn validar(estado: &Estado) -> Result<f64, String> {
    if estado.fecha.trim().is_empty() {
        return Err("La fecha es obligatoria".to_string());
    }

    if NaiveDate::parse_from_str(&estado.fecha, "%d-%m-%Y").is_err() {
        return Err("La fecha debe tener el formato DD-MM-AAAA".to_string());
    }

    if estado.monto.trim().is_empty() {
        return Err("El monto es obligatorio".to_string());
    }

    let monto = parsear_monto(&estado.monto)?;

    if !monto.is_finite() || monto <= 0.0 {
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

    Ok(monto)
}

fn parsear_monto(entrada: &str) -> Result<f64, String> {
    let entrada = entrada.trim();
    let normalizado = if entrada.contains(',') {
        entrada.replace('.', "").replace(',', ".")
    } else {
        entrada.to_string()
    };

    normalizado
        .parse()
        .map_err(|_| "El monto debe ser un numero valido".to_string())
}

fn sincronizar_selectores_fecha(estado: &mut Estado, fecha: &str) -> Result<(), String> {
    let fecha = NaiveDate::parse_from_str(fecha, "%d-%m-%Y")
        .map_err(|_| "La fecha del gasto no es válida".to_string())?;

    estado.dia = fecha.day();
    estado.mes_fecha = fecha.month();
    estado.anio_fecha = fecha.year() as u32;

    Ok(())
}

fn actualizar_fecha(estado: &mut Estado) {
    estado.dia = estado.dia.min(ultimo_dia_del_mes(estado.mes_fecha, estado.anio_fecha));
    estado.fecha = format!(
        "{:02}-{:02}-{}",
        estado.dia,
        estado.mes_fecha,
        estado.anio_fecha
    );
}

fn ultimo_dia_del_mes(mes: u32, anio: u32) -> u32 {
    match mes {
        2 if anio.is_multiple_of(400) || (anio.is_multiple_of(4) && !anio.is_multiple_of(100)) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

fn guardar_gasto(
    descripcion: String,
    monto: f64,
    fecha: String,
    categoria_id: i32,
    persona_id: i32,
) -> Result<(), String> {
    let conexion = crate::database::conectar_inicializada()?;

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
    let conexion = crate::database::conectar_inicializada()?;

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
    let conexion = crate::database::conectar_inicializada()?;

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
    let conexion = crate::database::conectar_inicializada()?;

    crate::repository::obtener_gastos_detalle(&conexion)
        .map_err(|error| error.to_string())
}

// Bajo este ancho, los metadatos pasan a una segunda línea para preservar legibilidad.
const ANCHO_TABLA_COMPACTA: f32 = 680.0;

fn confirmar_eliminacion(estado: &Estado) -> Element<'_, Message> {
    let detalle = estado
        .gasto_seleccionado
        .and_then(|id| estado.gastos.iter().find(|gasto| gasto.id == id))
        .map(|gasto| {
            format!(
                "{} · {} · {}",
                gasto.descripcion,
                formatear_monto(gasto.monto),
                gasto.fecha,
            )
        })
        .unwrap_or_else(|| "el gasto seleccionado".to_string());

    container(
        column![
            text("¿Eliminar este gasto?").size(28),
            text(detalle).size(18),
            text("Esta acción no se puede deshacer."),
            row![
                button("Cancelar")
                    .on_press(Message::CancelarEliminarGasto)
                    .style(estilos::estilo_boton_gastos),
                button("Sí, eliminar")
                    .on_press(Message::ConfirmarEliminarGasto)
                    .style(estilos::estilo_boton_gastos),
            ]
            .spacing(12),
        ]
        .spacing(18),
    )
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

fn tabla_gastos<'a>(estado: &'a Estado, ancho_disponible: f32) -> Element<'a, Message> {
    let mut indices: Vec<(usize, NaiveDate)> = estado
        .gastos
        .iter()
        .enumerate()
        .filter_map(|(indice, gasto)| {
            let fecha = NaiveDate::parse_from_str(&gasto.fecha, "%d-%m-%Y").ok()?;

            if fecha.month() == estado.mes && fecha.year() as u32 == estado.anio {
                Some((indice, fecha))
            } else {
                None
            }
        })
        .collect();

    indices.sort_by(|(indice_a, fecha_a), (indice_b, fecha_b)| {
        let gasto_a = &estado.gastos[*indice_a];
        let gasto_b = &estado.gastos[*indice_b];

        let orden = match estado.columna_orden {
            ColumnaOrden::Descripcion => gasto_a.descripcion.cmp(&gasto_b.descripcion),

            ColumnaOrden::Monto => gasto_a
                .monto
                .partial_cmp(&gasto_b.monto)
                .unwrap_or(std::cmp::Ordering::Equal),

            ColumnaOrden::Persona => gasto_a.persona.cmp(&gasto_b.persona),

            ColumnaOrden::Categoria => gasto_a.categoria.cmp(&gasto_b.categoria),

            ColumnaOrden::Fecha => fecha_a.cmp(fecha_b),
        };

        match estado.direccion_orden {
            DireccionOrden::Ascendente => orden,
            DireccionOrden::Descendente => orden.reverse(),
        }
    });

    let vista_compacta = ancho_disponible < ANCHO_TABLA_COMPACTA;

    let encabezado: Element<'a, Message> = if vista_compacta {
        column![
            row![
                button(
                    text(titulo_columna(
                        "Descripcion",
                        ColumnaOrden::Descripcion,
                        estado,
                    ))
                    .align_x(Alignment::Center)
                )
                .on_press(Message::OrdenarPor(ColumnaOrden::Descripcion))
                .width(Length::Fill)
                .style(estilo_encabezado),
                button(
                    text(titulo_columna("Monto", ColumnaOrden::Monto, estado))
                        .align_x(Alignment::Center)
                )
                .on_press(Message::OrdenarPor(ColumnaOrden::Monto))
                .width(Length::Fixed(120.0))
                .style(estilo_encabezado),
            ]
            .spacing(5),
            text("Fecha · Persona · Categoria")
                .size(14)
                .align_x(Alignment::Center),
        ]
        .spacing(4)
        .into()
    } else {
        row![
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
                text(titulo_columna("Monto", ColumnaOrden::Monto, estado,))
                    .align_x(Alignment::Center)
            )
            .on_press(Message::OrdenarPor(ColumnaOrden::Monto))
            .width(Length::FillPortion(1))
            .style(estilo_encabezado),
            button(
                text(titulo_columna("Persona", ColumnaOrden::Persona, estado,))
                    .align_x(Alignment::Center)
            )
            .on_press(Message::OrdenarPor(ColumnaOrden::Persona))
            .width(Length::FillPortion(1))
            .style(estilo_encabezado),
            button(
                text(titulo_columna("Categoria", ColumnaOrden::Categoria, estado,))
                    .align_x(Alignment::Center)
            )
            .on_press(Message::OrdenarPor(ColumnaOrden::Categoria))
            .width(Length::FillPortion(1))
            .style(estilo_encabezado),
            button(
                text(titulo_columna("Fecha", ColumnaOrden::Fecha, estado,))
                    .align_x(Alignment::Center)
            )
            .on_press(Message::OrdenarPor(ColumnaOrden::Fecha))
            .width(Length::FillPortion(1))
            .style(estilo_encabezado),
        ]
        .spacing(5)
        .into()
    };

    let mut filas = column![].spacing(5);

    for (indice, _) in indices {
        let gasto = &estado.gastos[indice];

        let seleccionado = estado.gasto_seleccionado == Some(gasto.id);

        let fila: Element<'a, Message> = if vista_compacta {
            let detalles = format!("{} · {} · {}", gasto.fecha, gasto.persona, gasto.categoria);

            button(
                column![
                    row![
                        text(&gasto.descripcion)
                            .width(Length::Fill)
                            .align_x(Alignment::Start),
                        text(formatear_monto(gasto.monto))
                            .width(Length::Fixed(120.0))
                            .align_x(Alignment::Center),
                    ]
                    .spacing(5),
                    text(detalles)
                        .size(14)
                        .width(Length::Fill)
                        .align_x(Alignment::Center),
                ]
                .spacing(4)
                .padding([7, 0]),
            )
            .on_press(Message::GastoSeleccionado(gasto.id))
            .style(move |_theme, _status| estilo_fila_gasto(seleccionado))
            .into()
        } else {
            button(
                row![
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
                .height(Length::Fixed(36.0)),
            )
            .on_press(Message::GastoSeleccionado(gasto.id))
            .style(move |_theme, _status| estilo_fila_gasto(seleccionado))
            .into()
        };

        filas = filas.push(fila);
    }

    container(
        column![encabezado, scrollable(filas).height(Length::Fill),]
            .spacing(10)
            .width(Length::Fill)
            .height(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
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

fn estilo_fila_gasto(seleccionado: bool) -> button::Style {
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
}

#[cfg(test)]
mod tests {
    use super::{ultimo_dia_del_mes, parsear_monto};

    #[test]
    fn acepta_montos_en_formato_local() {
        assert_eq!(parsear_monto("1.234,50"), Ok(1234.5));
        assert_eq!(parsear_monto("1234.50"), Ok(1234.5));
    }

    #[test]
    fn calcula_el_ultimo_dia_incluso_en_anios_bisiestos() {
        assert_eq!(ultimo_dia_del_mes(2, 2024), 29);
        assert_eq!(ultimo_dia_del_mes(2, 2026), 28);
        assert_eq!(ultimo_dia_del_mes(4, 2026), 30);
    }
}
