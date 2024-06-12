# appmana-comfyui-after-effects-plugin

This is a work in progress plugin for using ComfyUI from After Effects.

This plugin will expose common and custom workflows as **Effects** that correctly read and render image data in layers, similar to other conventional After Effects plugins.

This is not a script, windowed extension or other high-level wrapper.

It will support local and remote generation.

It is enabled by the [ComfyUI fork here](https://github.com/hiddenswitch/ComfyUI) which supports this and other developer-friendly approaches to using ComfyUI in different places and contexts.

## Development

Download the After Effects 2023 SDK and the Premiere Pro 2022 SDK
from https://developer.adobe.com/console/servicesandapis/ and extract `AfterEffectsSDK` and `Premiere Pro 22.0 C++ SDK`
directories to located in `./sdk`.

### Windows

This assumes you have set up your environment for development of ComfyUI following the instructions [here](https://github.com/AppMana/appmana-comfyui-after-effects-plugin/).

**Recommended**: Use Busybox as your shell.

| Type | Windows 64 bit | Windows 32 bit | History |
|------|----------------|----------------|---------|
| Stable | [Stable 64 bit](https://frippery.org/files/busybox/busybox64.exe) | [Stable 32 bit](https://frippery.org/files/busybox/busybox.exe) | [Browse](https://frippery.org/files/busybox/?C=M;O=D) |
| Pre-Release | [Pre-Release 64 bit](https://frippery.org/files/busybox/prerelease/busybox_pre64.exe) | [Pre-Release 32 bit](https://frippery.org/files/busybox/prerelease/busybox_pre.exe) | [Browse](https://frippery.org/files/busybox/prerelease/?C=M;O=D) |

Usually the pre-release 64-bit binary is the one you want. Then save it as `C:/Windows/sh.exe`, and set it up with `C:/Windows/sh.exe -ilX` as the command in Windows Terminal.

Then, install **Chocolatey**. These are prerequisites:

```shell
# must be separate
choco install -y gsudo
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