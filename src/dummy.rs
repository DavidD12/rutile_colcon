use super::*;

fn dummy_to_string(_: &str) -> Result<String, String> {
    Ok("".to_string())
}

pub fn generate_dummy(package_name: &str, directory: &str) -> Result<(), String> {
    let file = format!("{}/dummy.c", directory);
    let content = dummy_to_string(package_name)?;
    write_content(&content, &file)?;
    Ok(())
}
