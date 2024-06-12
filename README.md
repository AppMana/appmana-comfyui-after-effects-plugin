# appmana-comfyui-after-effects-plugin

Use ComfyUI from After Effects

## Development

Download the After Effects and Premiere 2022 SDKs from https://developer.adobe.com/console/servicesandapis/ and
extract `AfterEffectsSDK` and `Premiere Pro 22.0 C++ SDK` directories to located in `./sdk`.

**Windows Prerequisites**

```shell
# must be separate
choco install -y visualstudio2022buildtools
# includes clang + msvc
choco install -y visualstudio2022-workload-vctools --package-parameters "--add Microsoft.VisualStudio.Component.VC.Llvm.ClangToolset --add Microsoft.VisualStudio.Component.VC.Llvm.Clang"
choco install -y rustup.install
# verify
rustc --version
cargo --version
# include standard library
rustup component add rust-src
# rust build tool "just"
cargo install just
```

**Build**

```powershell
export LIBCLANG_PATH="C:/Program Files (x86)/Microsoft Visual Studio/2022/BuildTools/VC/Tools/Llvm/x64/bin"
just build
```

### Contributions

Initialized from https://github.com/AdrianEddy/after-effects/blob/master/examples/portable/build.rs