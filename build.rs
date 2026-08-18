use std::path::PathBuf;

fn main() {
    if cfg!(target_os = "windows") {
        let manifest_dir = PathBuf::from(
            std::env::var("CARGO_MANIFEST_DIR").unwrap()
        );

        let icono = manifest_dir.join("assets/icono.ico");

        let mut res = winresource::WindowsResource::new();
        res.set_icon(icono.to_str().unwrap());
        res.compile().unwrap();
    }
}
