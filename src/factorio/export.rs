use std::path::Path;
use sysinfo::System;

pub struct ExportArgs<'a> {
    pub mod_directory: &'a Path,
    pub factorio_dir: &'a Path,
    pub output_dir: &'a Path,
}

pub fn export(args: ExportArgs) {
    let factorio_exe = if cfg!(target_os = "windows") {
        args.factorio_dir.join("factorio.exe")
    } else {
        args.factorio_dir.join("factorio")
    };
    std::fs::create_dir_all(args.output_dir).unwrap();

    for cmd in [
        "--dump-data",
        "--dump-icon-sprites",
        "--dump-prototype-locale",
    ] {
        println!("> {factorio_exe:?} {cmd}");
        let process = std::process::Command::new(&factorio_exe)
            .arg(cmd)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .current_dir(args.output_dir)
            .spawn()
            .expect("Failed to execute command");

        let output = process.wait_with_output().expect("Failed to wait on child");

        let stdout = String::from_utf8_lossy(&output.stdout);

        if stdout.contains("Initializing Steam API") {
            println!("Launching on steam, waiting for factorio to come online...");
            while !is_factorio_running(cmd) {
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            println!("Factorio launched, waiting for it to close...");
            while is_factorio_running(cmd) {
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }

        std::fs::write(
            args.output_dir
                .join(format!("{}.stdout.log", cmd.trim_matches('-'))),
            &output.stdout,
        )
        .unwrap();
        std::fs::write(
            args.output_dir
                .join(format!("{}.stderr.log", cmd.trim_matches('-'))),
            &output.stderr,
        )
        .unwrap();
    }

    println!(
        "Copying from {:?}",
        args.mod_directory.join("script-output")
    );
    println!("  to {:?}", args.output_dir);

    fs_extra::dir::copy(
        args.mod_directory.join("script-output").join(""),
        args.output_dir,
        &fs_extra::dir::CopyOptions::new().overwrite(true),
    )
    .unwrap();
}

pub fn is_factorio_running(expected_arg: &str) -> bool {
    let system = System::new_all();
    #[cfg(target_os = "windows")]
    const PROCESS_NAME: &str = "factorio.exe";
    #[cfg(not(target_os = "windows"))]
    const PROCESS_NAME: &str = "factorio";

    for process in system.processes_by_name(std::ffi::OsStr::new("factorio")) {
        if process.name().eq_ignore_ascii_case(PROCESS_NAME) {
            let cmd = process.cmd();
            if cmd.is_empty() {
                // We don't have access, assume this is our process
                return true;
            }
            // Check if this is the factorio instance with the expected argument
            if cmd.iter().any(|arg| arg.to_string_lossy() == expected_arg) {
                return true;
            }
        }
    }
    false
}
