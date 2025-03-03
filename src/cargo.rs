use super::*;

fn cargo_to_string(package_name: &str) -> Result<String, String> {
    let s = format!(
        r#"
[profile.colcon]
inherits = "release"

[[bin]]
name = "{}_node"
path = "bin/{}_node.rs"
"#,
        package_name, package_name
    );

    Ok(s)
}

pub fn generate_cargo(
    package_name: &str,
    directory: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let _output = Command::new("cargo")
        .arg("init")
        .arg("--bin")
        .arg(directory)
        .output()?;
    let _output = Command::new("cargo")
        .arg("add")
        .arg("rutile_r2r")
        .current_dir(directory)
        .output()?;
    let _output = Command::new("rm")
        .arg("main.rs")
        .current_dir(directory.to_string() + "/src/")
        .output()?;
    //
    let file = format!("{}/Cargo.toml", directory);
    let content = cargo_to_string(package_name)?;
    append_content(&content, &file)?;
    Ok(())
}
