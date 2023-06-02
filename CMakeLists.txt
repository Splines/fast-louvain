cmake_minimum_required(VERSION 3.23) # to allow for File Sets feature,
# see https://cmake.org/cmake/help/latest/command/target_sources.html#file-sets
project(fast-louvain)

set(CMAKE_CXX_STANDARD 20)
set(CXX_STANDARD_REQUIRED ON)

# C++20 Module support
# https://github.com/Kitware/CMake/blob/master/Help/dev/experimental.rst#c20-module-apis
set(CMAKE_EXPERIMENTAL_CXX_MODULE_CMAKE_API "2182bf5c-ef0d-489a-91da-49dbc3090d2a")
set(CMAKE_EXPERIMENTAL_CXX_MODULE_DYNDEP ON)



# Generator
# apparently, the ninja generator (version 1.11 or above!)
# is necessary for C++20 modules to work [1].
# Setting the CMAKE_GENERATOR variable however does not work and CMake still
# used make instead of ninja. [2] suggest to use a PreLoad.cmake to solve this.
# To be honest, this is too much hazzle and if this simple task of setting
# this env variable and CMake reading it in does not work, then I can't
# help CMake. Will use "cmake -G ninja" instead and adjust the documentation
# accordingly.
#
# [1] https://stackoverflow.com/a/74321081/
# [1] https://stackoverflow.com/a/45247784/9655481
set(CMAKE_GENERATOR ninja)


# generate compile_commands.json for use with clangd
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)



# Add executable target
# New in CMake version 3.11:
# The source files can be omitted if they are added later using target_sources().
add_executable(louvain)

target_sources(louvain PUBLIC
    src/louvain.cpp
)

target_sources(louvain PUBLIC
    FILE_SET louvain_modules TYPE CXX_MODULES
    FILES
        src/math.cppm
)

# Compile options
target_compile_options(louvain PUBLIC
    -std=c++20 -Wall -Wextra -Wpedantic -Werror -fdiagnostics-color=always
)

