# Derroche

Aplicación de escritorio para registrar y analizar gastos personales. Permite organizar consumos por categoría y persona, consultar resúmenes mensuales y mantener los datos de forma local.

## Funcionalidades

- Alta, edición y eliminación de gastos.
- Registro de descripción, importe, fecha, categoría y persona.
- Filtrado mensual y ordenamiento de la lista de gastos.
- Panel de análisis con total mensual, comparación con el mes anterior, mayor gasto y gráficos de evolución, categorías y personas.
- Administración de categorías, personas y período predeterminado.
- Persistencia local con SQLite; la aplicación crea la base de datos y sus valores iniciales automáticamente.
- Búsqueda de actualizaciones publicadas en el repositorio de GitHub; en Windows permite descargar y ejecutar el instalador.

## Tecnologías

- [Rust](https://www.rust-lang.org/)
- [Iced](https://iced.rs/) para la interfaz gráfica
- [SQLite](https://www.sqlite.org/) mediante `rusqlite`
- `chrono`, `directories`, `reqwest` y `serde_json`

## Requisitos

- Rust con Cargo (edición 2024; se recomienda el canal estable actual).
- Las dependencias nativas necesarias para compilar aplicaciones gráficas en tu sistema operativo.

## Ejecutar desde el código fuente

```bash
git clone https://github.com/answet/derroche.git
cd derroche
cargo run --release
```

Para una compilación de desarrollo:

```bash
cargo run
```

También puedes comprobar el proyecto sin ejecutar la interfaz:

```bash
cargo check
```

## Datos locales

Derroche guarda su información en una base de datos SQLite llamada `derroche.db`, dentro del directorio local de datos que corresponde a la aplicación **Derroche** en cada sistema operativo. No requiere un servidor ni una cuenta: los gastos permanecen en el equipo.

Al iniciarse por primera vez crea las tablas necesarias y añade los valores iniciales `Sin Categoria` y `General`.

## Compilar para distribución

```bash
cargo build --release
```

El ejecutable se genera en `target/release/`. Para Windows, el proyecto incluye `installer/derroche.iss`, un script de Inno Setup para crear el instalador `Derroche-Setup.exe`.

## Estructura del proyecto

```text
src/
├── gui/            # Pantallas de gastos, análisis y configuración
├── database.rs      # Conexión e inicialización de SQLite
├── repository.rs    # Consultas y operaciones de datos
├── models.rs        # Modelos de dominio
├── updater.rs       # Consulta y descarga de actualizaciones
└── main.rs          # Punto de entrada
assets/              # Iconos e ilustraciones
installer/           # Configuración del instalador de Windows
```
