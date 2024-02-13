# mros
> The Micro Rust Operating System

`mros` is a microkernel library operating system currently being developed as part of a research project at the Pennsylvania State University.

## Build Instructions
To build the `x86_64` version of `mros` you can simply run `make`:

``` sh
$ make
```

If you want to build a different architecture (WIP) override the `ARCH` variable:

``` sh
$ make ARCH=aarch64
```

## Running `mros`

`mros` can be run in QEMU while testing by using the `make` command:

``` sh
$ make qemu
```

