mod factorio;

fn main() {
    let Some(install_dir) = factorio::find_factorio_install_dir() else {
        println!("Factorio install dir not found");
        println!("Please update the code to include the correct path on your machine");
        return;
    };

    let Some(config_dir) = factorio::find_factorio_config_dir() else {
        println!("Factorio config dir not found");
        println!("Please update the code to include the correct path on your machine");
        return;
    };

    println!("Factorio install dir: {:?}", install_dir);
    println!("Factorio config dir: {:?}", config_dir);

    if dialoguer::Confirm::new()
        .with_prompt("Do you want to export data?")
        .default(false)
        .interact()
        .unwrap()
    {
        let _ = std::fs::remove_dir_all("preset/k2se");
        std::fs::create_dir_all("preset/k2se").unwrap();
        factorio::export::export(factorio::export::ExportArgs {
            mod_directory: &config_dir,
            factorio_dir: &install_dir,
            output_dir: std::env::current_dir()
                .unwrap()
                .join("preset")
                .join("k2se")
                .as_path(),
        });
    }
}
