use iced::{
    Alignment, Background, Border, Color, Element, Length,
    widget::{column, container, pick_list, row, text},
};

use crate::estilos;
use crate::formato::{MESES, formatear_monto, nombre_mes, numero_mes};
use crate::models::{GastoDetalle, GastoPorCategoria, GastoPorPersona, TotalMensual};

#[derive(Debug, Clone)]
pub enum Message { MesSeleccionado(String) }

pub struct Estado {
    pub mes: u32, pub anio: i32,
    pub total_mes: f64, pub total_mes_anterior: f64, pub diferencia_mes: f64,
    pub mayor_gasto: Option<GastoDetalle>,
    pub totales_mensuales: Vec<TotalMensual>,
    pub gastos_por_categoria: Vec<GastoPorCategoria>,
    pub gastos_por_persona: Vec<GastoPorPersona>,
}

impl Default for Estado {
    fn default() -> Self {
        Self { mes: 8, anio: 2026, total_mes: 0.0, total_mes_anterior: 0.0,
            diferencia_mes: 0.0, mayor_gasto: None, totales_mensuales: Vec::new(),
            gastos_por_categoria: Vec::new(), gastos_por_persona: Vec::new() }
    }
}

pub fn update(estado: &mut Estado, mensaje: Message) {
    match mensaje { Message::MesSeleccionado(mes) => {
        if let Some(numero) = numero_mes(&mes) { estado.mes = numero; }
    }}
}

pub fn view(estado: &Estado) -> Element<'_, Message> {
    let selector_mes = pick_list(MESES, nombre_mes(estado.mes), |mes| Message::MesSeleccionado(mes.to_string()))
        .width(Length::Fixed(165.0)).text_size(24)
        .style(|_theme, _status| pick_list::Style {
            text_color: estilos::GASTOS_TEXTO_MES,
            background: Background::Color(Color::TRANSPARENT),
            border: Border::default(),
            placeholder_color: estilos::GASTOS_TEXTO_MES,
            handle_color: estilos::GASTOS_TEXTO_MES,
        })
        .menu_style(estilos::estilo_menu_selector);

    let mayor_gasto = match &estado.mayor_gasto {
        Some(gasto) => column![
            texto_principal(formatear_monto(gasto.monto), 25),
            texto_secundario(&gasto.descripcion, 14),
        ].spacing(5),
        None => column![
            texto_principal("Sin gastos", 25),
            texto_secundario("Todavía no hay movimientos", 14),
        ].spacing(5),
    };

    let resumen = row![
        tarjeta_resumen("TOTAL DEL MES", Some(formatear_monto(estado.total_mes)), None),
        tarjeta_resumen("MAYOR GASTO", None, Some(mayor_gasto.into())),
        tarjeta_resumen("CAMBIO MENSUAL", Some(format!("{:+.1}%", estado.diferencia_mes)), None),
    ].spacing(16).width(Length::Fill);

    let cabecera = column![
        container(selector_mes).width(Length::Fill).center_x(Length::Fill),
        resumen,
    ].spacing(18);

    let contenido = row![
        panel("Evolución anual", "El mes seleccionado se destaca en azul.", grafico_evolucion(estado))
            .width(Length::FillPortion(3)),
        column![
            panel("Por categoría", "Cómo se distribuyó el total del mes.", grafico_categorias(estado)),
            panel("Por persona", "Participación de cada persona este mes.", grafico_personas(estado)),
        ].spacing(16).width(Length::FillPortion(2)),
    ].spacing(16).height(Length::Fill);

    container(column![cabecera, contenido].spacing(26))
        .width(Length::Fill).height(Length::Fill).padding(32)
        .style(|_theme| container::Style { background: Some(Background::Color(estilos::FONDO_ANALISIS)), ..Default::default() })
        .into()
}

fn texto_principal<'a>(contenido: impl iced::widget::text::IntoFragment<'a>, tamano: u32) -> iced::widget::Text<'a> {
    text(contenido).size(tamano).color(estilos::ANALISIS_TEXTO_PRINCIPAL)
}

fn texto_secundario<'a>(contenido: impl iced::widget::text::IntoFragment<'a>, tamano: u32) -> iced::widget::Text<'a> {
    text(contenido).size(tamano).color(estilos::ANALISIS_TEXTO_SECUNDARIO)
}

fn tarjeta_resumen<'a>(titulo: &'a str, valor: Option<String>, contenido: Option<Element<'a, Message>>) -> Element<'a, Message> {
    let detalle = contenido.unwrap_or_else(|| texto_principal(valor.unwrap_or_default(), 27).into());
    container(column![texto_secundario(titulo, 12), detalle].spacing(9))
        .width(Length::FillPortion(1)).padding(18).style(estilo_tarjeta).into()
}

fn panel<'a>(titulo: &'a str, subtitulo: &'a str, contenido: Element<'a, Message>) -> iced::widget::Container<'a, Message> {
    container(column![texto_principal(titulo, 20), texto_secundario(subtitulo, 13), contenido].spacing(7))
        .width(Length::Fill).height(Length::Fill).padding(20).style(estilo_tarjeta)
}

fn estilo_tarjeta(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(estilos::ANALISIS_TARJETA)),
        border: Border { color: estilos::ANALISIS_BORDE, width: 1.0, radius: 12.0.into() },
        ..Default::default()
    }
}

fn totales_del_anio(totales: &[TotalMensual], anio: i32) -> Vec<f64> {
    (1..=12).map(|mes| totales.iter().find(|item| item.anio == anio && item.mes == mes)
        .map(|item| item.total).unwrap_or(0.0)).collect()
}

fn grafico_evolucion(estado: &Estado) -> Element<'_, Message> {
    let valores = totales_del_anio(&estado.totales_mensuales, estado.anio);
    let maximo = valores.iter().copied().fold(0.0_f64, f64::max);
    let mut barras = row![].spacing(12).align_y(Alignment::End);
    for (indice, valor) in valores.iter().enumerate() {
        let mes = (indice + 1) as u32;
        let seleccionado = mes == estado.mes;
        let altura = if maximo > 0.0 { ((valor / maximo) * 260.0).max(6.0) as f32 } else { 6.0 };
        let barra = container(text("")).width(Length::Fill).height(Length::Fixed(altura))
            .style(move |_theme| container::Style {
                background: Some(Background::Color(if seleccionado { estilos::ANALISIS_ACENTO } else { estilos::ANALISIS_BARRA_FONDO })),
                border: Border { radius: 5.0.into(), ..Border::default() }, ..Default::default()
            });
        barras = barras.push(column![
            texto_secundario(if *valor > 0.0 { formatear_monto(*valor) } else { String::new() }, 10),
            barra,
            text(nombre_mes(mes).unwrap_or("")).size(11).color(if seleccionado { estilos::ANALISIS_ACENTO } else { estilos::ANALISIS_TEXTO_SECUNDARIO }),
        ]
        .width(Length::FillPortion(1))
        .align_x(Alignment::Center)
        .spacing(9));
    }
    container(barras).height(Length::Fill).width(Length::Fill).center_x(Length::Fill)
        .align_y(Alignment::End).padding([18, 0]).into()
}

fn grafico_categorias(estado: &Estado) -> Element<'_, Message> {
    lista_distribucion(estado.gastos_por_categoria.iter().map(|item| (&item.categoria, item.total)),
        "Todavía no hay gastos en este período.", estilos::ANALISIS_ACENTO)
}

fn grafico_personas(estado: &Estado) -> Element<'_, Message> {
    lista_distribucion(estado.gastos_por_persona.iter().map(|item| (&item.persona, item.total)),
        "Todavía no hay gastos en este período.", estilos::ANALISIS_ACENTO_SUAVE)
}

fn lista_distribucion<'a>(items: impl Iterator<Item = (&'a String, f64)>, vacio: &'a str, color: Color) -> Element<'a, Message> {
    let items: Vec<_> = items.collect();
    if items.is_empty() {
        return container(texto_secundario(vacio, 14)).height(Length::Fill).center_y(Length::Fill).into();
    }
    let maximo = items.iter().map(|(_, total)| *total).fold(0.0_f64, f64::max);
    let mut filas = column![].spacing(12).padding([9, 0]);
    for (nombre, total) in items {
        let ancho = if maximo > 0.0 { ((total / maximo) * 150.0).max(4.0) as f32 } else { 4.0 };
        let progreso = container(container(text("")).width(Length::Fixed(ancho)).height(Length::Fixed(8.0))
            .style(move |_theme| container::Style { background: Some(Background::Color(color)), border: Border { radius: 4.0.into(), ..Border::default() }, ..Default::default() }))
            .height(Length::Fixed(8.0)).width(Length::FillPortion(3))
            .style(|_theme| container::Style { background: Some(Background::Color(estilos::ANALISIS_BARRA_FONDO)), border: Border { radius: 4.0.into(), ..Border::default() }, ..Default::default() });
        filas = filas.push(row![
            texto_principal(nombre, 14).width(Length::FillPortion(2)),
            progreso,
            texto_secundario(formatear_monto(total), 12).width(Length::FillPortion(2)),
        ].spacing(10).align_y(Alignment::Center));
    }
    filas.into()
}
