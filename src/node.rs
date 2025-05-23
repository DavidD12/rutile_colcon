use super::*;

fn main_to_string(package_name: &str) -> Result<String, String> {
    let s = format!(
        r#"use rutile::*;

#[tokio::main]
async fn main() -> Result<()> {{
    let mut node = Node::create("{}_node", "")?;

    node.spin();

    Ok(())
}}
"#,
        package_name
    );

    Ok(s)
}

pub fn generate_main(package_name: &str, directory: &str) -> Result<(), String> {
    let file = format!("{}/bin/{}_node.rs", directory, package_name);
    let content = main_to_string(package_name)?;
    write_content(&content, &file)?;
    Ok(())
}
