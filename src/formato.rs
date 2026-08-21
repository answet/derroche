pub const MESES: [&str; 12] = [
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

pub fn numero_mes(nombre: &str) -> Option<u32> {
    MESES
        .iter()
        .position(|mes| *mes == nombre)
        .map(|indice| indice as u32 + 1)
}

pub fn nombre_mes(numero: u32) -> Option<&'static str> {
    // `checked_sub` evita que el mes 0 desborde al convertirlo a índice.
    numero
        .checked_sub(1)
        .and_then(|indice| MESES.get(indice as usize).copied())
}

pub fn anios_alrededor(anio: i32) -> Vec<i32> {
    ((anio - 5)..=(anio + 5)).collect()
}

pub fn formatear_monto(monto: f64) -> String {
    if !monto.is_finite() {
        return "Monto inválido".to_string();
    }

    let negativo = monto < 0.0;
    // Trabajar con centavos evita mostrar `,100` cuando el redondeo supera 99.
    let centavos = (monto.abs() * 100.0).round() as u64;
    let parte_entera = centavos / 100;
    let parte_decimal = centavos % 100;

    let digitos = parte_entera.to_string();
    let mut entero_formateado = String::new();

    for (i, caracter) in digitos.chars().enumerate() {
        if i > 0 && (digitos.len() - i).is_multiple_of(3) {
            entero_formateado.push('.');
        }

        entero_formateado.push(caracter);
    }

    let signo = if negativo { "-" } else { "" };

    if parte_decimal == 0 {
        format!("{}${}", signo, entero_formateado)
    } else {
        format!("{}${},{:02}", signo, entero_formateado, parte_decimal)
    }
}

#[cfg(test)]
mod tests {
    use super::{anios_alrededor, formatear_monto, nombre_mes, numero_mes};

    #[test]
    fn convierte_meses_validos_e_invalidos() {
        assert_eq!(numero_mes("Enero"), Some(1));
        assert_eq!(numero_mes("Diciembre"), Some(12));
        assert_eq!(numero_mes("Mes inválido"), None);
        assert_eq!(nombre_mes(8), Some("Agosto"));
        assert_eq!(nombre_mes(0), None);
    }

    #[test]
    fn formatea_montos_con_separadores_locales() {
        assert_eq!(formatear_monto(1234.5), "$1.234,50");
        assert_eq!(formatear_monto(-10.0), "-$10");
        assert_eq!(formatear_monto(999.999), "$1.000");
    }

    #[test]
    fn genera_un_rango_de_anios_desplazable() {
        assert_eq!(anios_alrededor(2030), (2025..=2035).collect::<Vec<_>>());
    }
}
