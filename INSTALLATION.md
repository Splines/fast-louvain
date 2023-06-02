# Installation additional notes

## Install newest clang version on Ubuntu (e.g. in WSL)

- [LLVM Debian/Ubuntu nightly packages](https://apt.llvm.org/)
- [More detailed instructions to install these packages](https://askubuntu.com/a/1310730/1701650)
- [Cannot import name '_gi' bug during add-apt-repository](https://stackoverflow.com/a/67575251/)

---

```
sudo update-alternatives --install /usr/bin/clang clang "/usr/bin/clang-16" 100
```
where 100 is the priority of this alternative that we set very high.

---

During the build with Ninja, you may encounter this error
```
"CMAKE_CXX_COMPILER_CLANG_SCAN_DEPS-NOTFOUND" -format=p1689 -- /usr/bin/clang++-16 .....
```
This is already reported to LLVM including a quick fix [here](https://github.com/llvm/llvm-project/issues/61006#issuecomment-1529064924).

## Helpful links
- [Clangd Project page](https://clangd.llvm.org/troubleshooting)
- [Reasonable Clangd config in VSCode](https://stackoverflow.com/a/59820115/9655481)
- [C++ Compiler support](https://en.cppreference.com/w/cpp/compiler_support)
