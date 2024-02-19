# Install dropbear

## from source

Get the sourcecode

```bash
wget https://matt.ucc.asn.au/dropbear/releases/dropbear-2022.83.tar.bz2
```

Install cross compiling toolchain (ubuntu here)

```bash
sudo apt-get install gcc-arm-linux-gnueabihf binutils-arm-linux-gnueabihf
```

I modified this [tutorial](https://lists.ucc.gu.uwa.edu.au/pipermail/dropbear/2004q3/000022.html) from the dropbear Mailinglist and got a 285K binary

```bash
export CC=arm-linux-gnueabihf-gcc
export CFLAGS="-Os -ffunction-sections -fdata-sections"
export LDFLAGS="-Wl,--gc-sections"
./configure --disable-zlib 
make
```

## precompiled binary

Yeah, you need to trust me here

```bash
wget URL
```

