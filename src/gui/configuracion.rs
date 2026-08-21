use iced::widget::{button, column, container, pick_list, row, text, text_input};

use iced::{Alignment, Background, Border, Element, Length, Task};

use crate::models::{Categoria, Configuracion, Persona};
use crate::formato::{anios_alrededor, nombre_mes, numero_mes, MESES};

use crate::estilos;

#[derive(Debug, Clone)]
pub enum EliminacionPendiente {
    Categoria(Categoria),
    Persona(Persona),
}

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
    pub eliminacion_pendiente: Option<EliminacionPendiente>,
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
            eliminacion_pendiente: None,
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
    SolicitarEliminarCategoria(i32),
    EliminarCategoria(i32),

    AgregarPersona,
    SolicitarEliminarPersona(i32),
    EliminarPersona(i32),
    ConfirmarEliminacion,
    CancelarEliminacion,

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

        Message::SolicitarEliminarCategoria(id) => {
            estado.eliminacion_pendiente = estado
                .categorias
                .iter()
                .find(|categoria| categoria.id == id)
                .cloned()
                .map(EliminacionPendiente::Categoria);
            Task::none()
        }

        Message::AgregarPersona => {
            estado.nueva_persona.clear();
            Task::none()
        }

        Message::SolicitarEliminarPersona(id) => {
            estado.eliminacion_pendiente = estado
                .personas
                .iter()
                .find(|persona| persona.id == id)
                .cloned()
                .map(EliminacionPendiente::Persona);
            Task::none()
        }

        Message::CancelarEliminacion => {
            estado.eliminacion_pendiente = None;
            Task::none()
        }

        Message::ConfirmarEliminacion => {
            let mensaje = match estado.eliminacion_pendiente.take() {
                Some(EliminacionPendiente::Categoria(categoria)) => {
                    Message::EliminarCategoria(categoria.id)
                }
                Some(EliminacionPendiente::Persona(persona)) => {
                    Message::EliminarPersona(persona.id)
                }
                None => return Task::none(),
            };
            Task::done(mensaje)
        }

        Message::EliminarCategoria(_) | Message::EliminarPersona(_) => Task::none(),

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
                    } else {
                        // El instalador necesita reemplazar el ejecutable actual.
                        // Cerrar la aplicación evita que Windows lo mantenga bloqueado.
                        return iced::exit();
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
                        .on_press(Message::SolicitarEliminarCategoria(categoria.id,),)
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
                        .on_press(Message::SolicitarEliminarPersona(persona.id,),)
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

    let contenido = container(
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
    });

    let Some(eliminacion) = &estado.eliminacion_pendiente else {
        return contenido.into();
    };

    let (tipo, nombre) = match eliminacion {
        EliminacionPendiente::Categoria(categoria) => ("categoría", &categoria.nombre),
        EliminacionPendiente::Persona(persona) => ("persona", &persona.nombre),
    };

    container(
        column![
            container(
                column![
                    text(format!("¿Eliminar esta {tipo}?" )).size(26),
                    text(nombre).size(20),
                    text(format!(
                        "Vas a eliminar la {tipo} \"{nombre}\". Esta acción no se puede deshacer."
                    )),
                    row![
                        button("Cancelar")
                            .on_press(Message::CancelarEliminacion)
                            .style(estilos::estilo_boton_configuracion_agregar),
                        button("Sí, eliminar")
                            .on_press(Message::ConfirmarEliminacion)
                            .style(estilos::estilo_boton_configuracion_eliminar),
                    ]
                    .spacing(12),
                ]
                .spacing(16),
            )
            .padding(24)
            .width(Length::Fixed(460.0))
            .style(|_theme| container::Style {
                background: Some(Background::Color(estilos::FONDO_CONFIGURACION)),
                border: Border {
                    width: 1.0,
                    radius: 12.0.into(),
                    ..Border::default()
                },
                ..Default::default()
            }),
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .style(|_theme| container::Style {
        background: Some(Background::Color(estilos::FONDO_CONFIGURACION)),
        ..Default::default()
    })
    .into()
}
