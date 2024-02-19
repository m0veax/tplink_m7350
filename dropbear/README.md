# dropbear

## install dropbear

Get a dropbear binary by a method of your choice.

- [Activate telnet](../tp-opener/README.md)
- [start adbd](../README.md#start-adbd)
- Connect device per USB to your computer

```
adb devices
adb push dropbear /bin/dropbear
```



## precompiled binary

Yeah, you need to trust me here

```bash
wget https://github.com/m0veax/tplink_m7350/raw/main/dropbear/dropbear
```

## from source

Get the sourcecode

```bash
wget https://matt.ucc.asn.au/dropbear/releases/dropbear-2022.83.tar.bz2
```

Install cross compiling toolchain (ubuntu here)

```
sudo apt-get install gcc-arm-linux-gnueabihf binutils-arm-linux-gnueabihf libcrypt-dev:armhf
```

Get [standalone libcrypt](https://github.com/mkj/dropbear/issues/143#issuecomment-1114174803)

```
sget https://github.com/besser82/libxcrypt/releases/download/v4.4.36/libxcrypt-4.4.36.tar.xz
tar -xvf libxcrypt-4.4.36.tar.xz  
cd libxcrypt-4.4.36
./configure --prefix=/usr/arm-linux-gnueabihf  CC=arm-linux-gnueabihf-gcc \
--host=arm
sudo make install
```

I modified this [tutorial](https://lists.ucc.gu.uwa.edu.au/pipermail/dropbear/2004q3/000022.html) from the dropbear Mailinglist and got a 375K binary

```bash
export CC=arm-linux-gnueabihf-gcc
export CFLAGS="-Os -ffunction-sections -fdata-sections -I/usr/bin/arm-linux-gnueabihf/include"
export LDFLAGS="-Wl,--gc-sections -L/usr/bin/arm-linux-gnueabihf/lib"
./configure --disable-zlib --prefix=/usr/bin/arm-linux-gnueabihf --host=arm
make
```

