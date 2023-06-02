cmake_minimum_required(VERSION 3.0) # version was not particularly selected
project(fast-louvain)

set(CMAKE_CXX_STANDARD 20)
set(CXX_STANDARD_REQUIRED ON)

# generate compile_commands.json for use with clangd
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Source files
set(SOURCES
    src/louvain.cpp
)

# Add executable target
add_executable(louvain
    ${SOURCES}
)

# Compile options
target_compile_options(louvain PUBLIC -Wall -Wextra -Wpedantic -Werror -fdiagnostics-color=always)

