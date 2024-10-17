use std::path::Path;

mod factorio;

fn main() {
    let _ = std::fs::remove_dir_all("preset/k2se");
    std::fs::create_dir_all("preset/k2se").unwrap();

    factorio::export::export(factorio::export::ExportArgs {
        mod_directory: Path::new("C:/Users/VictorKoenders/AppData/Roaming/Factorio"),
        factorio_dir: Path::new(
            "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Factorio\\bin\\x64\\",
        ),
        output_dir: std::env::current_dir()
            .unwrap()
            .join("preset")
            .join("k2se")
            .as_path(),
    });
}
