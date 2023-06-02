# Fast Louvain
A fast and modern C++ implementation of the Louvain algorithm.

| :arrows_counterclockwise:   | This project is currently under development. Once a first workable version is accomplished, I will publish a release. |
|---------------|:-------------------------|

This project uses C++20 modules. You need Clang v16 or higher.
You also need Ninja v1.11 or higher for modules to work in CMake.
I know this might be a restriction for you, but modules are worth the effort
upgrading your compiler and build system.


## Install
```
git clone https://github.com/Splines/fast-louvain.git
cd fast-louvain/
```
There are some additional notes for the installation, see [here](./INSTALLATION.md)


## Build
```
mkdir build
cd build/
cmake -G Ninja ..
ninja
```

(watch out, you need to use `Ninja`, not `ninja`)

## Run
```
cd build/
./louvain
```