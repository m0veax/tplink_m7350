# Kernel

## Sources

**NOTE: This is work in progress and not yet working!**

<https://github.com/m0veax/tplink_m7350-kernel>

<https://github.com:orangecms/linux/tree/tp-link-m7350-v3-rework>

We committed an initial [Linux](https://kernel.org) `3.4` release and `rsync`ed
the TP-Link v2 kernel sources over it:

```sh
rsync -I -c --recursive /path/to/tplink_kernel/ /path/to/linux/
```

We then applied additional fixes for it to compile.

## Compiling

Using Linaro's latest GCC 4 toolchain, TL;DR

```sh
#!/bin/sh

export ARCH=arm

export BASEDIR=/path/to/toolchain
# https://releases.linaro.org/components/toolchain/binaries/latest-4/
export TARGET=arm-linux-gnueabi
export TOOLCHAIN=gcc-linaro-4.9.4-2017.01-x86_64_${TARGET}

export CROSS_COMPILE=${TARGET}-
export PATH=${BASEDIR}/${TOOLCHAIN}/bin:$PATH

${TARGET}-gcc -v

make m7350-un-v3_defconfig
make $@
```

## config dump

We dumped and extracted [`/proc/config.gz` from a v3 device](config-v3).

## dmesg

We have dumped a [`dmesg` log](dmesg-v3.log).
