use iced::widget::{button, column, row, container};
use iced::{Element, Task, Length, Alignment, Background, Border};
use iced::border::Radius;

use super::{gastos, analisis, configuracion};

use crate::estilos;
use crate::models::{Categoria, Persona, GastoDetalle, TotalMensual, GastoPorCategoria, GastoPorPersona, Configuracion};

const ICONO: &[u8] = include_bytes!("../../assets/icono.png");

pub fn run() -> iced::Result {
    let imagen = image::load_from_memory(ICONO)
        .expect("No se pudo cargar el icono")
        .into_rgba8();

    let (ancho, alto) = imagen.dimensions();

    let icono = iced::window::icon::from_rgba(imagen.into_raw(), ancho, alto)
        .expect("No se pudo cargar el icono");

    iced::application(inicializar, update, view)
        .window(iced::window::Settings {
            icon: Some(icono),

            ..Default::default()
        })
        .run()
}

fn inicializar() -> (Estado, Task<Message>) {
    let estado = Estado {
        pantalla: Pantalla::Gastos,
        ..Estado::default()
    };

    let tarea = Task::perform(
        async {
            let conexion =
                crate::database::conectar()
                    .map_err(|error| error.to_string())?;

            crate::database::inicializar_db(&conexion)
                .map_err(|error| error.to_string())?;

            let configuracion =
                crate::repository::obtener_configuracion(&conexion)
                    .map_err(|error| error.to_string())?;

            let categorias =
                crate::repository::obtener_categorias(&conexion)
                    .map_err(|error| error.to_string())?;

            let personas =
                crate::repository::obtener_personas(&conexion)
                    .map_err(|error| error.to_string())?;

            let gastos =
                crate::repository::obtener_gastos_detalle(&conexion)
                    .map_err(|error| error.to_string())?;

            Ok::<_, String>((
                configuracion,
                categorias,
                personas,
                gastos,
            ))
        },
        |resultado| match resultado {
            Ok((
                configuracion,
                categorias,
                personas,
                gastos,
            )) => {
                Message::DatosInicialesCargados(
                    configuracion,
                    categorias,
                    personas,
                    gastos,
                )
            }

            Err(error) => {
                Message::ErrorCarga(error)
            }
        },
    );

    (estado, tarea)
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pantalla {
    Gastos,
    Analisis,
    Configuracion,
}

impl Default for Pantalla {
    fn default() -> Self {
        Pantalla::Gastos
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    DatosInicialesCargados(
        Configuracion,
        Vec<Categoria>,
        Vec<Persona>,
        Vec<GastoDetalle>,
    ),

    CargarDatos,
    DatosCargados(Vec<Categoria>, Vec<Persona>, Vec<GastoDetalle>,),
    DatosAnalisisCargados(f64, f64, Option<GastoDetalle>, Vec<TotalMensual>, Vec<GastoPorCategoria>, Vec<GastoPorPersona>),

    ErrorCarga(String),

    MostrarGastos,
    MostrarAnalisis,
    MostrarConfiguracion,

    Gastos(gastos::Message),
    Analisis(analisis::Message),
    Configuracion(configuracion::Message),
}

#[derive(Default)]
struct Estado {
    pantalla: Pantalla,
    gastos: gastos::Estado,
    analisis: analisis::Estado,
    configuracion: configuracion::Estado,
}

fn cargar_datos() -> Result<(Vec<Categoria>, Vec<Persona>, Vec<GastoDetalle>), String> {
    let conexion =
        crate::database::conectar()
            .map_err(|error| error.to_string())?;

    crate::database::inicializar_db(&conexion)
        .map_err(|error| error.to_string())?;

    let categorias =
        crate::repository::obtener_categorias(&conexion)
            .map_err(|error| error.to_string())?;

    let personas =
        crate::repository::obtener_personas(&conexion)
            .map_err(|error| error.to_string())?;

    let gastos =
        crate::repository::obtener_gastos_detalle(&conexion)
            .map_err(|error| error.to_string())?;

    Ok((categorias, personas, gastos))
}

fn cargar_datos_analisis(
    mes: u32,
    anio: i32,
) -> Result<(f64, f64, Option<GastoDetalle>, Vec<TotalMensual>, Vec<GastoPorCategoria>, Vec<GastoPorPersona>), String> {

    let conexion =
        crate::database::conectar()
            .map_err(|error| error.to_string())?;

    crate::database::inicializar_db(&conexion)
        .map_err(|error| error.to_string())?;

    let total_mes =
        crate::repository::obtener_total_mes(&conexion, mes, anio)
            .map_err(|error| error.to_string())?;

    let (mes_anterior, anio_anterior) =
        if mes == 1 {
            (12, anio - 1)
        } else {
            (mes - 1, anio)
        };

    let total_mes_anterior =
        crate::repository::obtener_total_mes(
            &conexion,
            mes_anterior,
            anio_anterior,
        )
        .map_err(|error| error.to_string())?;

    let mayor_gasto =
        crate::repository::obtener_mayor_gasto_mes(&conexion, mes, anio)
            .map_err(|error| error.to_string())?;

    let totales_mensuales =
        crate::repository::obtener_totales_mensuales(&conexion)
            .map_err(|error| error.to_string())?;

    let gastos_por_categoria =
        crate::repository::obtener_gastos_por_categoria(
            &conexion,
            mes,
            anio,
        )
        .map_err(|error| error.to_string())?;

    let gastos_por_persona =
        crate::repository::obtener_gastos_por_persona(
            &conexion,
            mes,
            anio,
        )
        .map_err(|error| error.to_string())?;

    Ok((
        total_mes,
        total_mes_anterior,
        mayor_gasto,
        totales_mensuales,
        gastos_por_categoria,
        gastos_por_persona,
    ))
}

fn cargar_datos_configuracion()
    -> Result<
        (
            Configuracion,
            Vec<Categoria>,
            Vec<Persona>,
        ),
        String,
    >
{
    let conexion =
        crate::database::conectar()
            .map_err(|error| error.to_string())?;

    crate::database::inicializar_db(&conexion)
        .map_err(|error| error.to_string())?;

    let configuracion =
        crate::repository::obtener_configuracion(&conexion)
            .map_err(|error| error.to_string())?;

    let categorias =
        crate::repository::obtener_categorias(&conexion)
            .map_err(|error| error.to_string())?;

    let personas =
        crate::repository::obtener_personas(&conexion)
            .map_err(|error| error.to_string())?;

    Ok((
        configuracion,
        categorias,
        personas,
    ))
}

fn update(estado: &mut Estado, mensaje: Message) -> Task<Message> {
    match mensaje {
        Message::MostrarGastos => {
            estado.pantalla = Pantalla::Gastos;

            estado.gastos.mes = estado.configuracion.mes_default;

            estado.gastos.anio = estado.configuracion.anio_default as u32;

            Task::none()
        }

        Message::MostrarConfiguracion => {
            estado.pantalla = Pantalla::Configuracion;

            Task::perform(
                async { cargar_datos_configuracion() },
                |resultado| match resultado {
                    Ok((configuracion, categorias, personas)) => {
                        Message::Configuracion(
                            configuracion::Message::DatosCargados(
                                configuracion,
                                categorias,
                                personas
                            )
                        )
                    }

                    Err(error) => {
                        println!("Error al cargar datos de configuracion: {error}");

                        Message::Configuracion(
                            configuracion::Message::DatosCargados(
                                Configuracion {
                                    mes_default: 8,
                                    anio_default: 2026,
                                },
                                Vec::new(),
                                Vec::new(),
                            )
                        )
                    }
                },
            )
        }

        Message::Gastos(mensaje) => {
            gastos::update(&mut estado.gastos, mensaje)
                .map(Message::Gastos)
        }

        Message::CargarDatos => {
            Task::perform(
                async { cargar_datos() },
                |resultado| match resultado {
                    Ok((categorias, personas, gastos)) => {
                        Message::DatosCargados(categorias, personas, gastos)
                    }

                    Err(error) => {
                        println!("Error al cargar datos: {error}");
                        Message::DatosCargados(Vec::new(), Vec::new(), Vec::new())
                    }
                },
            )
        }

        Message::DatosCargados(categorias, personas, gastos) => {
            estado.gastos.categorias = categorias.clone();
            estado.gastos.personas = personas.clone();
            estado.gastos.gastos = gastos;

            estado.gastos.categoria = estado
                .gastos
                .categorias
                .iter()
                .find(|categoria| categoria.nombre == "Sin Categoria")
                .cloned();

            estado.gastos.persona = estado
                .gastos
                .personas
                .iter()
                .find(|persona| persona.nombre == "General")
                .cloned();

            estado.configuracion.categorias = categorias;
            estado.configuracion.personas = personas;

            estado.configuracion.nueva_categoria.clear();
            estado.configuracion.nueva_persona.clear();

            Task::none()
        }

        Message::MostrarAnalisis => {
            estado.pantalla = Pantalla::Analisis;

            estado.analisis.mes = estado.configuracion.mes_default;
            estado.analisis.anio = estado.configuracion.anio_default;

            let mes = estado.analisis.mes;
            let anio = estado.analisis.anio;

            Task::perform(
                async move {
                    cargar_datos_analisis(mes, anio)
                },
                |resultado| match resultado {
                    Ok((total, total_anterior, mayor_gasto, totales, gastos_por_categoria, gastos_por_persona)) => {
                        Message::DatosAnalisisCargados(
                            total,
                            total_anterior,
                            mayor_gasto,
                            totales,
                            gastos_por_categoria,
                            gastos_por_persona,
                        )
                    }

                    Err(error) => {
                        println!("Error al cargar datos de analisis: {error}");

                        Message::DatosAnalisisCargados(
                            0.0,
                            0.0,
                            None,
                            Vec::new(),
                            Vec::new(),
                            Vec::new(),
                        )
                    }
                },
            )
        }

        Message::Analisis(mensaje) => {
            let es_cambio_mes = matches!(
                mensaje,
                analisis::Message::MesSeleccionado(_)
            );

            analisis::update(
                &mut estado.analisis,
                mensaje,
            );

            if es_cambio_mes {
                let mes = estado.analisis.mes;
                let anio = estado.analisis.anio;

                Task::perform(
                    async move {
                        cargar_datos_analisis(mes, anio)
                    },
                    |resultado| match resultado {
                        Ok((total, total_anterior, mayor_gasto, totales, gastos_por_categoria, gastos_por_persona)) => {
                            Message::DatosAnalisisCargados(
                                total,
                                total_anterior,
                                mayor_gasto,
                                totales,
                                gastos_por_categoria,
                                gastos_por_persona,
                            )
                        }

                        Err(error) => {
                            println!("Error al cargar datos de analisis: {error}");

                            Message::DatosAnalisisCargados(
                                0.0,
                                0.0,
                                None,
                                Vec::new(),
                                Vec::new(),
                                Vec::new(),
                            )
                        }
                    },
                )
            } else {
                Task::none()
            }
        }

        Message::DatosAnalisisCargados(
            total,
            total_anterior,
            mayor_gasto,
            totales,
            gastos_por_categoria,
            gastos_por_persona,
        ) => {
            estado.analisis.total_mes = total;
            estado.analisis.total_mes_anterior = total_anterior;
            estado.analisis.mayor_gasto = mayor_gasto;
            estado.analisis.totales_mensuales = totales;
            estado.analisis.gastos_por_categoria = gastos_por_categoria;
            estado.analisis.gastos_por_persona = gastos_por_persona;

            if total_anterior == 0.0 {
                estado.analisis.diferencia_mes = 0.0;
            } else {
                estado.analisis.diferencia_mes =
                    ((total - total_anterior) / total_anterior) * 100.0;
            }

            Task::none()
        }

        Message::ErrorCarga(error) => {
            estado.gastos.error = Some(error);

            Task::none()
        }

        Message::Configuracion(mensaje) => {
            match mensaje {
                configuracion::Message::GuardarConfiguracion => {
                    let mes = estado.configuracion.mes_default;
                    let anio = estado.configuracion.anio_default;

                    Task::perform(
                        async move {
                            let conexion =
                                crate::database::conectar()
                                    .map_err(|error| error.to_string())?;

                            crate::repository::actualizar_configuracion(
                                &conexion,
                                mes,
                                anio,
                            )
                            .map_err(|error| error.to_string())?;

                            Ok::<(), String>(())
                        },
                        |resultado| {
                            if let Err(error) = resultado {
                                println!(
                                    "Error al guardar configuracion: {error}"
                                );
                            }

                            Message::Configuracion(
                                configuracion::Message::CargarDatos
                            )
                        },
                    )
                }

                configuracion::Message::AgregarCategoria => {
                    let nombre =
                        estado.configuracion.nueva_categoria
                            .trim()
                            .to_string();

                    if nombre.is_empty() {
                        return Task::none();
                    }

                    Task::perform(
                        async move {
                            let conexion =
                                crate::database::conectar()
                                    .map_err(|error| error.to_string())?;

                            crate::repository::agregar_categoria(
                                &conexion,
                                &nombre,
                            )
                            .map_err(|error| error.to_string())?;

                            Ok::<(), String>(())
                        },
                        |resultado| {
                            match resultado {
                                Ok(()) => Message::CargarDatos,

                                Err(error) => {
                                    println!(
                                        "Error al agregar categoria: {error}"
                                    );

                                    Message::CargarDatos
                                }
                            }
                        },
                    )
                }

                configuracion::Message::AgregarPersona => {
                    let nombre =
                        estado.configuracion.nueva_persona
                            .trim()
                            .to_string();

                    if nombre.is_empty() {
                        return Task::none();
                    }

                    Task::perform(
                        async move {
                            let conexion =
                                crate::database::conectar()
                                    .map_err(|error| error.to_string())?;

                            crate::repository::agregar_persona(
                                &conexion,
                                &nombre,
                            )
                            .map_err(|error| error.to_string())?;

                            Ok::<(), String>(())
                        },
                        |resultado| {
                            match resultado {
                                Ok(()) => Message::CargarDatos,

                                Err(error) => {
                                    println!(
                                        "Error al agregar persona: {error}"
                                    );

                                    Message::CargarDatos
                                }
                            }
                        },
                    )
                }

                configuracion::Message::EliminarCategoria(id) => {
                    Task::perform(
                        async move {
                            let conexion =
                                crate::database::conectar()
                                    .map_err(|error| error.to_string())?;

                            crate::repository::eliminar_categoria(
                                &conexion,
                                id,
                            )
                            .map_err(|error| error.to_string())?;

                            Ok::<(), String>(())
                        },
                        |resultado| {
                            match resultado {
                                Ok(()) => Message::CargarDatos,

                                Err(error) => {
                                    println!(
                                        "Error al eliminar categoria: {error}"
                                    );

                                    Message::CargarDatos
                                }
                            }
                        },
                    )
                }

                configuracion::Message::EliminarPersona(id) => {
                    Task::perform(
                        async move {
                            let conexion =
                                crate::database::conectar()
                                    .map_err(|error| error.to_string())?;

                            crate::repository::eliminar_persona(
                                &conexion,
                                id,
                            )
                            .map_err(|error| error.to_string())?;

                            Ok::<(), String>(())
                        },
                        |resultado| {
                            match resultado {
                                Ok(()) => Message::CargarDatos,

                                Err(error) => {
                                    println!(
                                        "Error al eliminar persona: {error}"
                                    );

                                    Message::CargarDatos
                                }
                            }
                        },
                    )
                }

                mensaje => {
                    configuracion::update(
                        &mut estado.configuracion,
                        mensaje,
                    )
                    .map(Message::Configuracion)
                }
            }
        }

        Message::DatosInicialesCargados(
            configuracion,
            categorias,
            personas,
            gastos,
        ) => {
            estado.configuracion.mes_default = configuracion.mes_default;
            estado.configuracion.anio_default = configuracion.anio_default;
            estado.configuracion.categorias = categorias.clone();
            estado.configuracion.personas = personas.clone();
            estado.gastos.mes = configuracion.mes_default;
            estado.gastos.anio = configuracion.anio_default as u32;
            estado.gastos.categorias = categorias;
            estado.gastos.personas = personas;
            estado.gastos.gastos = gastos;
            estado.analisis.mes = configuracion.mes_default;
            estado.analisis.anio = configuracion.anio_default;

            Task::none()
        }
    }
}

// fn view(estado: &Estado) -> Element<'_, Message> {
//     let contenido = match estado.pantalla {
//         Pantalla::Gastos => gastos::view(&estado.gastos).map(Message::Gastos),
//         Pantalla::Analisis => analisis::view(&estado.analisis).map(Message::Analisis),
//         Pantalla::Configuracion => configuracion::view(&estado.configuracion).map(Message::Configuracion),
//     };
//
//     row![
//         sidebar(estado),
//
//         container(contenido)
//             .width(Length::Fill)
//             .height(Length::Fill)
//     ]
//     .width(Length::Fill)
//     .height(Length::Fill)
//     .into()
// }

fn view(estado: &Estado) -> Element<'_, Message> {
    let contenido = match estado.pantalla {
        Pantalla::Gastos => gastos::view(&estado.gastos).map(Message::Gastos),
        Pantalla::Analisis => analisis::view(&estado.analisis).map(Message::Analisis),
        Pantalla::Configuracion => configuracion::view(&estado.configuracion).map(Message::Configuracion),
    };

    column![
        barra_navegacion(estado),

        container(contenido)
            .width(Length::Fill)
            .height(Length::Fill)
    ]
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn barra_navegacion(estado: &Estado) -> Element<'static, Message> {
    let color_gastos = if estado.pantalla == Pantalla::Gastos {
        estilos::FONDO_GASTOS
    } else {
        estilos::BOTONES_SIDEBAR_INACTIVOS
    };

    let color_analisis = if estado.pantalla == Pantalla::Analisis {
        estilos::FONDO_ANALISIS
    } else {
        estilos::BOTONES_SIDEBAR_INACTIVOS
    };

    let color_configuracion = if estado.pantalla == Pantalla::Configuracion {
        estilos::FONDO_CONFIGURACION
    } else {
        estilos::BOTONES_SIDEBAR_INACTIVOS
    };

    container(
        row![
            container(
                button("Gastos")
                    .on_press(Message::MostrarGastos)
                    .style(|_theme, _status| button::Style {
                        background: None,
                        text_color: estilos::TEXTO_SIDEBAR,
                        border: Border::default(),
                        ..Default::default()
                    })
            )
            .padding([8, 16])
            .style(move |_theme| container::Style {
                background: Some(Background::Color(color_gastos)),
                border: Border {
                    color: color_gastos,
                    width: 0.0,
                    radius: Radius {
                        top_left: 14.0,
                        top_right: 14.0,
                        bottom_right: 0.0,
                        bottom_left: 0.0,
                    },
                },
                ..Default::default()
            }),

            container(
                button("Analisis")
                    .on_press(Message::MostrarAnalisis)
                    .style(|_theme, _status| button::Style {
                        background: None,
                        text_color: estilos::TEXTO_SIDEBAR,
                        border: Border::default(),
                        ..Default::default()
                    })
            )
            .padding([8, 16])
            .style(move |_theme| container::Style {
                background: Some(Background::Color(color_analisis)),
                border: Border {
                    color: color_analisis,
                    width: 0.0,
                    radius: Radius {
                        top_left: 14.0,
                        top_right: 14.0,
                        bottom_right: 0.0,
                        bottom_left: 0.0,
                    },
                },
                ..Default::default()
            }),

            container(
                button("Configuracion")
                    .on_press(Message::MostrarConfiguracion)
                    .style(|_theme, _status| button::Style {
                        background: None,
                        text_color: estilos::TEXTO_SIDEBAR,
                        border: Border::default(),
                        ..Default::default()
                    })
            )
            .padding([8, 16])
            .style(move |_theme| container::Style {
                background: Some(Background::Color(color_configuracion)),
                border: Border {
                    color: color_configuracion,
                    width: 0.0,
                    radius: Radius {
                        top_left: 14.0,
                        top_right: 14.0,
                        bottom_right: 0.0,
                        bottom_left: 0.0,
                    },
                },
                ..Default::default()
            }),
        ]
        .spacing(10)
        .align_y(Alignment::End),
    )
    .width(Length::Fill)
    .height(Length::Shrink)
    .padding(iced::Padding {
        top: 10.0,
        right: 10.0,
        bottom: 0.0,
        left: 10.0,
    })
    .style(|_theme| container::Style {
        background: Some(Background::Color(estilos::FONDO_SIDEBAR)),
        border: Border::default(),
        ..Default::default()
    })
    .into()
}

// fn sidebar(estado: &Estado) -> Element<'static, Message> {
//     let color_gastos = if estado.pantalla == Pantalla::Gastos {
//         estilos::FONDO_GASTOS
//     } else {
//         estilos::BOTONES_SIDEBAR_INACTIVOS
//     };
//
//     let color_analisis = if estado.pantalla == Pantalla::Analisis {
//         estilos::FONDO_ANALISIS
//     } else {
//         estilos::BOTONES_SIDEBAR_INACTIVOS
//     };
//
//     let color_configuracion = if estado.pantalla == Pantalla::Configuracion {
//         estilos::FONDO_CONFIGURACION
//     } else {
//         estilos::BOTONES_SIDEBAR_INACTIVOS
//     };
//
//     container(
//         column![
//             container(
//                 button("Gastos")
//                     .on_press(Message::MostrarGastos)
//                     .style(|_theme, _status| button::Style {
//                         background: None,
//                         text_color: estilos::TEXTO_SIDEBAR,
//                         border: Border::default(),
//                         ..Default::default()
//                     })
//             )
//             .width(Length::Fill)
//             .padding([10, 20])
//             .style(move |_theme| container::Style {
//                 background: Some(Background::Color(color_gastos)),
//                 border: Border {
//                     color: color_gastos,
//                     width: 1.0,
//                     radius: Radius {
//                         top_left: 14.0,
//                         top_right: 0.0,
//                         bottom_right: 0.0,
//                         bottom_left: 14.0,
//                     },
//                 },
//                 ..Default::default()
//             }),
//
//             container(
//                 button("Analisis")
//                     .on_press(Message::MostrarAnalisis)
//                     .style(|_theme, _status| button::Style {
//                         background: None,
//                         text_color: estilos::TEXTO_SIDEBAR,
//                         border: Border::default(),
//                         ..Default::default()
//                     })
//             )
//             .width(Length::Fill)
//             .padding([10, 20])
//             .style(move |_theme| container::Style {
//                 background: Some(Background::Color(color_analisis)),
//                 border: Border {
//                     color: color_analisis,
//                     width: 1.0,
//                     radius: Radius {
//                         top_left: 14.0,
//                         top_right: 0.0,
//                         bottom_right: 0.0,
//                         bottom_left: 14.0,
//                     },
//                 },
//                 ..Default::default()
//             }),
//
//             container(
//                 button("Configuracion")
//                     .on_press(Message::MostrarConfiguracion)
//                     .style(|_theme, _status| button::Style {
//                         background: None,
//                         text_color: estilos::TEXTO_SIDEBAR,
//                         border: Border::default(),
//                         ..Default::default()
//                     })
//             )
//             .width(Length::Fill)
//             .padding([10, 20])
//             .style(move |_theme| container::Style {
//                 background: Some(Background::Color(color_configuracion)),
//                 border: Border {
//                     color: color_configuracion,
//                     width: 1.0,
//                     radius: Radius {
//                         top_left: 14.0,
//                         top_right: 0.0,
//                         bottom_right: 0.0,
//                         bottom_left: 14.0,
//                     },
//                 },
//                 ..Default::default()
//             }),
//         ]
//         .spacing(20)
//         .align_x(Alignment::Center),
//     )
//     .width(Length::Fixed(180.0))
//     .height(Length::Fill)
//     .padding(iced::Padding {
//         left: 25.0,
//         right: 0.0,
//         top: 0.0,
//         bottom: 0.0,
//     })
//     .center_x(Length::Fixed(180.0))
//     .center_y(Length::Fill)
//     .style(|_theme| container::Style {
//         background: Some(Background::Color(estilos::FONDO_SIDEBAR)),
//         ..Default::default()
//     })
//     .into()
// }
