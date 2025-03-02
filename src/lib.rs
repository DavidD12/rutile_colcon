use log::{error, info};
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::Command;

pub mod package;
pub use package::*;

pub mod cmakelists;
pub use cmakelists::*;

pub mod dummy;
pub use dummy::*;

pub mod r2r_cargo;
pub use r2r_cargo::*;

pub mod cargo;
pub use cargo::*;

pub mod node;
pub use node::*;

pub(crate) fn folder_exists_or_create<S: Into<String>>(path: S) -> Result<(), String> {
    let path: String = path.into();
    if Path::new(&path).exists() {
        info!("path {} exists", path);
        Ok(())
    } else {
        match fs::create_dir(&path) {
            Ok(_) => {
                info!("folder {} created", path);
                Ok(())
            }
            Err(e) => {
                error!("{}", e);
                Err(e.to_string())
            }
        }
    }
}

pub(crate) fn write_content(content: &str, destination: &str) -> Result<(), String> {
    match File::create(destination) {
        Ok(mut file) => match write!(file, "{}", content) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}

pub(crate) fn append_content(content: &str, destination: &str) -> Result<(), String> {
    match OpenOptions::new().append(true).open(destination) {
        Ok(mut file) => match write!(file, "{}", content) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}

pub fn generate(package_name: &str, directory: &str) -> Result<(), Box<dyn std::error::Error>> {
    //------------------------- Folders -------------------------
    info!("create folders ({})", directory);
    folder_exists_or_create(directory)?;
    folder_exists_or_create(format!("{}/bin", directory))?;
    //------------------------- Cargo -------------------------
    info!("generate Cargo.toml");
    generate_cargo(package_name, directory)?;
    //------------------------- Colcon Pacakage -------------------------
    info!("generate colcon");
    generate_package(package_name, directory)?;
    generate_cmakelists(package_name, directory)?;
    generate_dummy(package_name, directory)?;
    generate_r2r_cargo(package_name, directory)?;
    //------------------------- Rust Pacakage -------------------------
    info!("generate rust");
    generate_main(package_name, directory)?;
    //
    Ok(())
}
