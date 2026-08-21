use iced::widget::{button, column, container, pick_list, row, text, text_input};

use iced::{Alignment, Background, Border, Element, Length, Task};

use crate::models::{Categoria, Configuracion, Persona};
use crate::formato::{anios_alrededor, nombre_mes, numero_mes, MESES};

use crate::estilos;

pub struct Estado {
    pub mes_default: u32,
    pub anio_default: i32,

    pub categorias: Vec<Categoria>,
    pub personas: Vec<Persona>,

    pub nueva_categoria: String,
    pub nueva_persona: String,

    pub actualizacion_disponible: Option<(String, Option<String>)>,
    pub buscando_actualizacion: bool,
    pub busqueda_actualizacion_realizada: bool,
    pub error_actualizacion: Option<String>,
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
            busqueda_actualizacion_realizada: false,
            error_actualizacion: None,
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
    ActualizacionEncontrada(
        Result<Option<(String, Option<String>)>, String>
    ),

    #[cfg(target_os = "windows")]
    DescargarActualizacion,
    #[cfg(target_os = "windows")]
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
            estado.error_actualizacion = None;

            Task::perform(
                crate::updater::buscar_actualizacion(),
                Message::ActualizacionEncontrada,
            )
        }

        Message::ActualizacionEncontrada(resultado) => {
            estado.buscando_actualizacion = false;
            estado.busqueda_actualizacion_realizada = true;

            match resultado {
                Ok(actualizacion) => {
                    estado.actualizacion_disponible = actualizacion;
                }

                Err(error) => {
                    estado.error_actualizacion = Some(error);
                }
            }

            Task::none()
        }

        #[cfg(target_os = "windows")]
        Message::DescargarActualizacion => {
            if let Some(url) = estado
                .actualizacion_disponible
                .as_ref()
                .and_then(|(_, url)| url.as_ref())
            {
                Task::perform(
                    crate::updater::descargar_actualizacion(url.clone()),
                    Message::ActualizacionDescargada,
                )
            } else {
                Task::none()
            }
        }

        #[cfg(target_os = "windows")]
        Message::ActualizacionDescargada(resultado) => {
            match resultado {
                Ok(ruta) => {
                    if let Err(error) =
                        std::process::Command::new(&ruta).spawn()
                    {
                        estado.error_actualizacion = Some(
                            format!(
                                "No se pudo ejecutar el instalador: {error}"
                            )
                        );
                    }
                }

                Err(error) => {
                    estado.error_actualizacion = Some(error);
                }
            }

            Task::none()
        }
    }
}

pub fn view(estado: &Estado) -> Element<'_, Message> {
    let anios = anios_alrededor(estado.anio_default);

    let selector_mes = pick_list(MESES, nombre_mes(estado.mes_default), |mes| {
        Message::MesDefaultSeleccionado(mes.to_string())
    })
    .style(|_theme, _status| pick_list::Style {
        background: Background::Color(estilos::BOTON_CONFIGURACION_LISTA),
        text_color: estilos::BOTON_CONFIGURACION_LISTA_TEXTO,
        border: Border::default(),
        placeholder_color: estilos::BOTON_CONFIGURACION_LISTA,
        handle_color: estilos::BOTON_CONFIGURACION_LISTA_TEXTO,
    })
    .menu_style(estilos::estilo_menu_selector);

    let selector_anio = pick_list(
        anios,
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
    .menu_style(estilos::estilo_menu_selector);

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
                .style(estilos::estilo_boton_configuracion_agregar),
            column(estado.categorias.iter().map(|categoria| {
                row![
                    text(&categoria.nombre).width(Length::Fill),
                    button("Eliminar")
                        .on_press(Message::EliminarCategoria(categoria.id,),)
                        .style(estilos::estilo_boton_configuracion_eliminar),
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
                .style(estilos::estilo_boton_configuracion_agregar),
            column(estado.personas.iter().map(|persona| {
                row![
                    text(&persona.nombre).width(Length::Fill),
                    button("Eliminar")
                        .on_press(Message::EliminarPersona(persona.id,),)
                        .style(estilos::estilo_boton_configuracion_eliminar),
                ]
                .spacing(10)
                .into()
            }))
            .spacing(8),
        ]
        .spacing(15),
    )
    .width(Length::Fixed(250.0));

    let contenido_actualizaciones = column![
        text("Actualizaciones").size(24),

        text(format!(
            "Versión actual: {}",
            env!("CARGO_PKG_VERSION")
        )),

        button(if estado.buscando_actualizacion {
            "Buscando..."
        } else {
            "Buscar actualizaciones"
        })
        .on_press_maybe(if estado.buscando_actualizacion {
            None
        } else {
            Some(Message::BuscarActualizacion)
        })
        .style(estilos::estilo_boton_configuracion_agregar),

        if estado.busqueda_actualizacion_realizada {
            match &estado.actualizacion_disponible {
                Some((version, _)) => {
                    text(format!(
                        "Hay una nueva versión disponible: {}",
                        version
                    ))
                }

                None => {
                    text("No hay actualizaciones disponibles.")
                }
            }
        } else {
            text("")
        },
    ]
    .spacing(15);

    #[cfg(target_os = "windows")]
    let boton_descargar = button("Descargar actualización")
        .on_press(Message::DescargarActualizacion)
        .style(estilos::estilo_boton_configuracion_agregar);

    #[cfg(not(target_os = "windows"))]
    let boton_descargar = text("");

    let actualizaciones = if estado.actualizacion_disponible.is_some()
    {
        container(
            column![
                contenido_actualizaciones,
                boton_descargar,
            ]
            .spacing(15),
        )
        .width(Length::Shrink)
    } else {
        container(contenido_actualizaciones)
            .width(Length::Shrink)
    };

    container(
        column![
            preferencias,

            container(
                row![
                    container(categorias),
                    container(personas),
                ]
                .spacing(110)
            )
            .width(Length::Fill)
            .center_x(Length::Fill),

            container(actualizaciones)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::Start)
                .align_y(Alignment::End),
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
