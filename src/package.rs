use super::*;

fn package_to_string(package_name: &str) -> Result<String, String> {
    let mut s = String::new();

    s += &format!(
        r#"<?xml version="1.0"?>
<?xml-model href="http://download.ros.org/schema/package_format3.xsd" schematypens="http://www.w3.org/2001/XMLSchema"?>
<package format="3">
  <name>{}</name>
  <version>0.0.1</version>
  <description>Genereted with rutile_colcon</description>
  <maintainer email="david.doose@onera.fr">David Doose</maintainer>
  <license>TODO: License declaration</license>

  <buildtool_depend>ament_cmake</buildtool_depend>

  <build_depend>rcl</build_depend>
  <build_depend>std_msgs</build_depend>
  <!-- TODO DEPEND -->

  <exec_depend>rcl</exec_depend>
  <exec_depend>std_msgs</exec_depend>

  <!-- -->
  <export>
    <build_type>ament_cmake</build_type>
  </export>
</package>
"#,
        package_name
    );

    Ok(s)
}

pub fn generate_package(package_name: &str, directory: &str) -> Result<(), String> {
    let file = format!("{}/package.xml", directory);
    let content = package_to_string(package_name)?;
    write_content(&content, &file)?;
    Ok(())
}
