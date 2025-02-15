cmake_minimum_required(VERSION 3.12)
project(DungeonCrawlerWorld)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Set better default paths for Visual Studio
set_property(DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR} PROPERTY VS_STARTUP_PROJECT ${PROJECT_NAME})
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY_DEBUG ${CMAKE_BINARY_DIR}/Debug)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY_RELEASE ${CMAKE_BINARY_DIR}/Release)

# Find required packages
find_package(glfw3 CONFIG REQUIRED)
find_package(glm CONFIG REQUIRED)

# Source files
set(SOURCES
    src/game/main.cpp
    src/game/Entity.cpp
    src/core/Window.cpp
    src/core/Game.cpp
    src/core/InputManager.cpp
    src/core/AssetManager.cpp
    src/physics/PhysicsSystem.cpp
)

# Create executable
add_executable(${PROJECT_NAME} ${SOURCES})

# Link libraries
target_link_libraries(${PROJECT_NAME} PRIVATE
    glfw
    glm::glm
)

# Include directories
target_include_directories(${PROJECT_NAME} PRIVATE
    "${CMAKE_CURRENT_SOURCE_DIR}/include"
)

# Enable warnings
if(MSVC)
    target_compile_options(${PROJECT_NAME} PRIVATE /W4)
else()
    target_compile_options(${PROJECT_NAME} PRIVATE -Wall -Wextra)
endif()
