use super::*;

fn cmakelists_to_string(package_name: &str) -> Result<String, String> {
    let s = format!(
        r#"cmake_minimum_required(VERSION 3.5)
project({})

#--------------------------------------------------
# find dependencies
find_package(ament_cmake REQUIRED)
#--------------------------------------------------
if(NOT DEFINED CMAKE_SUPPRESS_DEVELOPER_WARNINGS)
     set(CMAKE_SUPPRESS_DEVELOPER_WARNINGS 1 CACHE INTERNAL "No dev warnings")
endif()
#--------------------------------------------------------------------
include(r2r_cargo.cmake)
#--------------------------------------------------------------------
# put ros package dependencies here.
r2r_cargo(
  std_msgs               # just to test that it works
  rcl                    # we need the c ros2 api
  rcl_action             # as of r2r 0.1.0, we also need the action api
)
#--------------------------------------------------------------------
# install binaries
if(WIN32)
  set(SUFFIX ".exe")
else()
  set(SUFFIX "")
endif()

install(PROGRAMS
${{CMAKE_SOURCE_DIR}}/target/colcon/{}_node
DESTINATION lib/${{PROJECT_NAME}}
)

# we need this for ros/colcon
ament_package()
"#,
        package_name, package_name,
    );

    Ok(s)
}

pub fn generate_cmakelists(package_name: &str, directory: &str) -> Result<(), String> {
    let file = format!("{}/CMakeLists.txt", directory);
    let content = cmakelists_to_string(package_name)?;
    write_content(&content, &file)?;
    Ok(())
}
