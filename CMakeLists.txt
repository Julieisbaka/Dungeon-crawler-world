cmake_minimum_required(VERSION 3.12)
project(DungeonCrawlerWorld)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Set better default paths for Visual Studio
set_property(DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR} PROPERTY VS_STARTUP_PROJECT ${PROJECT_NAME})
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY_DEBUG ${CMAKE_BINARY_DIR}/Debug)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY_RELEASE ${CMAKE_BINARY_DIR}/Release)

# Source files
set(SOURCES
    src/game/main.cpp
    src/physics/PhysicsSystem.cpp
)

# Create executable
add_executable(${PROJECT_NAME} ${SOURCES})

# Include directories - using relative paths for better portability
target_include_directories(${PROJECT_NAME} PRIVATE
    "${CMAKE_CURRENT_SOURCE_DIR}/include"
    "$ENV{USERPROFILE}/vcpkg/installed/x64-windows/include"  # Windows path for vcpkg GLM
    "/usr/include"  # Linux fallback path
)

# Enable warnings
if(MSVC)
    target_compile_options(${PROJECT_NAME} PRIVATE /W4)
else()
    target_compile_options(${PROJECT_NAME} PRIVATE -Wall -Wextra)
endif()
