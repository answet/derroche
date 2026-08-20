use iced::widget::{button, column, container, pick_list, row, text, text_input};

use iced::{Alignment, Background, Border, Element, Length, Shadow, Task};

use crate::models::{Categoria, Configuracion, Persona};

use iced::overlay::menu;

use crate::estilos;

const ANIOS: [i32; 5] = [2026, 2027, 2028, 2029, 2030];

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

pub struct Estado {
    pub mes_default: u32,
    pub anio_default: i32,

    pub categorias: Vec<Categoria>,
    pub personas: Vec<Persona>,

    pub nueva_categoria: String,
    pub nueva_persona: String,

    pub actualizacion_disponible: Option<String>,
    pub buscando_actualizacion: bool,
    pub error_actualizacion: Option<String>,
    pub busqueda_actualizacion_realizada: bool,
    pub url_actualizacion: Option<String>,
    pub descargando_actualizacion: bool,
}

impl Default for Estado {
    fn default() -> Self {
        Self {
            mes_default: 8,
            anio_default: 2026,

            categorias: Vec::new(),
            personas: Vec::new(),

            nueva_categoria: String::new(),
            nueva_persona: String::new(),

            actualizacion_disponible: None,
            buscando_actualizacion: false,
            error_actualizacion: None,
            busqueda_actualizacion_realizada: false,
            url_actualizacion: None,
            descargando_actualizacion: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    CargarDatos,
    DatosCargados(Configuracion, Vec<Categoria>, Vec<Persona>),

    MesDefaultSeleccionado(String),
    AnioDefaultSeleccionado(i32),

    GuardarConfiguracion,

    AgregarCategoria,
    EliminarCategoria(i32),

    AgregarPersona,
    EliminarPersona(i32),

    CategoriaNombreCambiado(String),
    PersonaNombreCambiado(String),

    BuscarActualizacion,
    ActualizacionEncontrada(Result<Option<(String, String)>, String>),
    DescargarActualizacion,
    ActualizacionDescargada(Result<std::path::PathBuf, String>),
}

pub fn update(estado: &mut Estado, mensaje: Message) -> Task<Message> {
    match mensaje {
        Message::CargarDatos => Task::none(),

        Message::DatosCargados(configuracion, categorias, personas) => {
            estado.mes_default = configuracion.mes_default;
            estado.anio_default = configuracion.anio_default;
            estado.categorias = categorias;
            estado.personas = personas;

            Task::none()
        }

        Message::MesDefaultSeleccionado(mes) => {
            if let Some(numero) = numero_mes(&mes) {
                estado.mes_default = numero;
            }

            Task::done(Message::GuardarConfiguracion)
        }

        Message::AnioDefaultSeleccionado(anio) => {
            estado.anio_default = anio;

            Task::done(Message::GuardarConfiguracion)
        }

        Message::GuardarConfiguracion => Task::none(),

        Message::AgregarCategoria => {
            estado.nueva_categoria.clear();
            Task::none()
        }

        Message::EliminarCategoria(_) => Task::none(),

        Message::AgregarPersona => {
            estado.nueva_persona.clear();
            Task::none()
        }

        Message::EliminarPersona(_) => Task::none(),

        Message::CategoriaNombreCambiado(nombre) => {
            estado.nueva_categoria = nombre;
            Task::none()
        }

        Message::PersonaNombreCambiado(nombre) => {
            estado.nueva_persona = nombre;
            Task::none()
        }

        Message::BuscarActualizacion => {
            estado.buscando_actualizacion = true;
            estado.busqueda_actualizacion_realizada = false;
            estado.actualizacion_disponible = None;
            estado.url_actualizacion = None;
            estado.error_actualizacion = None;

            Task::perform(crate::updater::buscar_actualizacion(), |resultado| {
                Message::ActualizacionEncontrada(resultado)
            })
        }

        Message::ActualizacionEncontrada(resultado) => {
            estado.buscando_actualizacion = false;
            estado.busqueda_actualizacion_realizada = true;

            match resultado {
                Ok(Some((version, url))) => {
                    estado.actualizacion_disponible = Some(version);
                    estado.url_actualizacion = Some(url);
                }

                Ok(None) => {
                    estado.actualizacion_disponible = None;
                    estado.url_actualizacion = None;
                }

                Err(error) => {
                    estado.actualizacion_disponible = None;
                    estado.url_actualizacion = None;
                    estado.error_actualizacion = Some(error);
                }
            }

            Task::none()
        }

        Message::DescargarActualizacion => {
            let Some(url) = estado.url_actualizacion.clone() else {
                return Task::none();
            };

            estado.descargando_actualizacion = true;
            estado.error_actualizacion = None;

            Task::perform(crate::updater::descargar_actualizacion(url), |resultado| {
                Message::ActualizacionDescargada(resultado)
            })
        }

        Message::ActualizacionDescargada(resultado) => {
            estado.descargando_actualizacion = false;

            match resultado {
                Ok(ruta) => match crate::updater::ejecutar_instalador(ruta) {
                    Ok(()) => {
                        std::process::exit(0);
                    }

                    Err(error) => {
                        estado.error_actualizacion = Some(error);
                    }
                },

                Err(error) => {
                    estado.error_actualizacion = Some(error);
                }
            }

            Task::none()
        }
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

pub fn view(estado: &Estado) -> Element<'_, Message> {
    let selector_mes = pick_list(MESES, Some(nombre_mes(estado.mes_default)), |mes| {
        Message::MesDefaultSeleccionado(mes.to_string())
    })
    .style(|_theme, _status| pick_list::Style {
        background: Background::Color(estilos::BOTON_CONFIGURACION_LISTA),
        text_color: estilos::BOTON_CONFIGURACION_LISTA_TEXTO,
        border: Border::default(),
        placeholder_color: estilos::BOTON_CONFIGURACION_LISTA,
        handle_color: estilos::BOTON_CONFIGURACION_LISTA_TEXTO,
    })
    .menu_style(|_theme| menu::Style {
        text_color: estilos::GASTOS_TEXTO_SELECTOR,
        background: Background::Color(estilos::GASTOS_TEXTO_SELECTOR_FONDO),
        border: Border::default(),
        selected_text_color: estilos::GASTOS_TEXTO_SELECTOR_SELECCIONADO,
        selected_background: Background::Color(estilos::GASTOS_TEXTO_SELECTOR_FONDO_SELECCIONADO),
        shadow: Shadow::default(),
    });

    let selector_anio = pick_list(
        ANIOS,
        Some(estado.anio_default),
        Message::AnioDefaultSeleccionado,
    )
    .style(|_theme, _status| pick_list::Style {
        background: Background::Color(estilos::BOTON_CONFIGURACION_LISTA),
        text_color: estilos::BOTON_CONFIGURACION_LISTA_TEXTO,
        border: Border::default(),
        placeholder_color: estilos::BOTON_CONFIGURACION_LISTA,
        handle_color: estilos::BOTON_CONFIGURACION_LISTA_TEXTO,
    })
    .menu_style(|_theme| menu::Style {
        text_color: estilos::GASTOS_TEXTO_SELECTOR,
        background: Background::Color(estilos::GASTOS_TEXTO_SELECTOR_FONDO),
        border: Border::default(),
        selected_text_color: estilos::GASTOS_TEXTO_SELECTOR_SELECCIONADO,
        selected_background: Background::Color(estilos::GASTOS_TEXTO_SELECTOR_FONDO_SELECCIONADO),
        shadow: Shadow::default(),
    });

    let preferencias = container(
        column![
            text("Preferencias").size(24),
            row![text("Mes por defecto"), selector_mes,]
                .spacing(20)
                .align_y(Alignment::Center),
            row![text("Año por defecto"), selector_anio,]
                .spacing(20)
                .align_y(Alignment::Center),
        ]
        .spacing(15),
    )
    .width(Length::Fill);

    let categorias = container(
        column![
            text("Categorías").size(24),
            text_input("Nueva categoría", &estado.nueva_categoria,)
                .on_input(Message::CategoriaNombreCambiado),
            button("Agregar")
                .on_press(Message::AgregarCategoria)
                .style(|_theme, _status| button::Style {
                    background: Some(Background::Color(estilos::BOTONES_CONFIGURACION_AGREGAR,),),
                    text_color: estilos::TEXTO_CONFIGURACION_AGREGAR,
                    ..Default::default()
                }),
            column(estado.categorias.iter().map(|categoria| {
                row![
                    text(&categoria.nombre).width(Length::Fill),
                    button("Eliminar")
                        .on_press(Message::EliminarCategoria(categoria.id,),)
                        .style(|_theme, _status| button::Style {
                            background: Some(Background::Color(
                                estilos::BOTONES_CONFIGURACION_ELIMINAR,
                            ),),
                            text_color: estilos::TEXTO_CONFIGURACION_ELIMINAR,
                            ..Default::default()
                        }),
                ]
                .spacing(10)
                .into()
            }))
            .spacing(8),
        ]
        .spacing(15),
    )
    .width(Length::Fixed(250.0));

    let personas = container(
        column![
            text("Personas").size(24),
            text_input("Nueva persona", &estado.nueva_persona,)
                .on_input(Message::PersonaNombreCambiado),
            button("Agregar")
                .on_press(Message::AgregarPersona)
                .style(|_theme, _status| button::Style {
                    background: Some(Background::Color(estilos::BOTONES_CONFIGURACION_AGREGAR,),),
                    text_color: estilos::TEXTO_CONFIGURACION_AGREGAR,
                    ..Default::default()
                }),
            column(estado.personas.iter().map(|persona| {
                row![
                    text(&persona.nombre).width(Length::Fill),
                    button("Eliminar")
                        .on_press(Message::EliminarPersona(persona.id,),)
                        .style(|_theme, _status| button::Style {
                            background: Some(Background::Color(
                                estilos::BOTONES_CONFIGURACION_ELIMINAR,
                            ),),
                            text_color: estilos::TEXTO_CONFIGURACION_ELIMINAR,
                            ..Default::default()
                        }),
                ]
                .spacing(10)
                .into()
            }))
            .spacing(8),
        ]
        .spacing(15),
    )
    .width(Length::Fixed(250.0));

    let actualizaciones = container(
        column![
            text("Actualizaciones").size(24),
            text(format!("Versión actual: {}", env!("CARGO_PKG_VERSION"))),
            button(if estado.buscando_actualizacion {
                "Buscando..."
            } else {
                "Buscar actualizaciones"
            })
            .on_press_maybe(
                if estado.buscando_actualizacion || estado.descargando_actualizacion {
                    None
                } else {
                    Some(Message::BuscarActualizacion)
                }
            )
            .style(|_theme, _status| button::Style {
                background: Some(Background::Color(estilos::BOTONES_CONFIGURACION_AGREGAR,),),
                text_color: estilos::TEXTO_CONFIGURACION_AGREGAR,
                ..Default::default()
            }),
            if estado.busqueda_actualizacion_realizada {
                match &estado.actualizacion_disponible {
                    Some(version) => column![
                        text(format!("Hay una nueva versión disponible: {}", version)),
                        button(if estado.descargando_actualizacion {
                            "Descargando..."
                        } else {
                            "Descargar actualización"
                        })
                        .on_press_maybe(if estado.descargando_actualizacion {
                            None
                        } else {
                            Some(Message::DescargarActualizacion)
                        })
                        .style(|_theme, _status| button::Style {
                            background: Some(Background::Color(
                                estilos::BOTONES_CONFIGURACION_AGREGAR,
                            ),),
                            text_color: estilos::TEXTO_CONFIGURACION_AGREGAR,
                            ..Default::default()
                        }),
                    ]
                    .spacing(10),

                    None => {
                        column![text("No hay actualizaciones disponibles.")]
                    }
                }
            } else {
                column![]
            },
            if let Some(error) = &estado.error_actualizacion {
                text(format!("Error: {}", error))
            } else {
                text("")
            },
        ]
        .spacing(15),
    )
    .width(Length::Fill);

    container(
        column![
            preferencias,
            container(row![container(categorias), container(personas),].spacing(110),)
                .width(Length::Fill)
                .center_x(Length::Fill),
            actualizaciones,
            if let Some(error) = &estado.error_actualizacion {
                text(format!("Error: {}", error))
            } else {
                text("")
            },
        ]
        .spacing(30),
    )
    .padding(40)
    .width(Length::Fill)
    .height(Length::Fill)
    .style(|_theme| container::Style {
        background: Some(Background::Color(estilos::FONDO_CONFIGURACION)),
        ..Default::default()
    })
    .into()
}
