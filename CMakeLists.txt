cmake_minimum_required(VERSION 3.10)

# Set the project name and version
project(DungeonCrawlerWorld VERSION 0.0.3.9)

# Specify the C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Add the executable
# add_executable(DungeonCrawlerWorld src/main.cpp)

# Include directories
# include_directories(${PROJECT_SOURCE_DIR}/include)

# Link libraries
# target_link_libraries(DungeonCrawlerWorld PRIVATE some_library)

# Add subdirectories if they exist
# if(EXISTS "${PROJECT_SOURCE_DIR}/src")
#     add_subdirectory(src)
# endif()
# Add source files from the Floor directory
# file(GLOB_RECURSE FLOOR_SOURCES ${PROJECT_SOURCE_DIR}/Floor/*.cpp)
# add_executable(DungeonCrawlerWorld ${FLOOR_SOURCES})

# Add the Scheme directory for JSON schema validation
# file(GLOB_RECURSE SCHEME_FILES ${PROJECT_SOURCE_DIR}/Scheme/*.json)
# add_custom_target(copy_schemas ALL
#     COMMAND ${CMAKE_COMMAND} -E copy_directory
#     ${PROJECT_SOURCE_DIR}/Scheme $<TARGET_FILE_DIR:DungeonCrawlerWorld>/Scheme
# )

# Ensure schemas are copied before building the executable
# add_dependencies(DungeonCrawlerWorld copy_schemas)