# Compile Kerne

## Get the sources

https://github.com/m0veax/tplink_m7350-kernel

We checked out the linux release tag `v3.4` and rsynced the tp-link kernel sources over it using following command:

```
rsync -I -c --recursive /path/to/tplink_kernel/ /path/to/linux/
```

## extracted config

We extracted `/proc/config.gz`. You can find the deflated config [here](config)
