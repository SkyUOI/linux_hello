# linux-hello

## KCM

### Introduction

KCM Linux Hello is a kcm which can connect to face recognition kernel.

There are two kernels you can use by now:

- noop. It is a dummy kernel which does nothing or is for testing.
- face-id. It is a kernel which can recognize face through face-id crate of Rust.

### Dependencies

To run KCM Linux Hello, some dependencies are required.

- Rust tool chain
- KDE Plasma (Wayland supported only)
- kcmshell6
- sea-orm-cli

you can install sea-orm-cli through cargo:

```bash
cargo install sea-orm-cli

```

### Build

If you are the first time to build KCM Linux Hello, you need to run the following command at the root of the project:

```bash
cargo xtask kcm prepare --kernel <KERNEL>
```

Then you can build or run KCM Linux Hello by running the following command:

```bash
# build
cargo xtask kcm build --kernel <KERNEL>
# run
cargo xtask kcm run --kernel <KERNEL>
```

### Install

If you want to install it to the system settings, you can run the following command:

```bash
cargo xtask kcm install --kernel <KERNEL>
```

More details about xtask can be seen through `cargo xtask kcm --help`.
