# Device Infos

TP Link Router M7350 v3

## Photos

![up](assets/up.jpg)

![down](assets/down.jpg)

## Notes

### fastboot

If you remove the battery and plugin USB, lsusb shows:

```
Bus 001 Device 031: ID 18d1:d00d Google Inc. Xiaomi Mi/Redmi 2 (fastboot)
```

```
> fastboot devices
MDM9625	fastboot

```

Above disappears after a few seconds.

If you boot normal, it shows:

```
Bus 001 Device 032: ID 2357:0005 TP-Link M7350 4G Mi-Fi Router
```

Enter fastboot without bootloop

```
fastboot reboot bootloader
```

### Firmware

Device seems to run Android. You can get the firmware here:

https://static.tp-link.com/upload/firmware/2022/202206/20220607/M7350(EU)_V4_220606.zip

The Firmware is not crypted. You are able to unzip the .img and take a deeper look at it.

### .dtb files

The .dtb files of HW rev v3 and v4 are stored in [dtb_files](dtb_files/) and can be visualized with [dtvis](https://github.com/platform-system-interface/dtvis/)

## TODO

- [ ] Compare Kernel 3.4.0 with TP Link Sources
- [ ] Find Qualcomm debug stuff online
- [ ] Link v3 Firmware instead of v4

## Weblinks

### OpenWRT Board
- http://forum.archive.openwrt.org/viewtopic.php?id=72055
- http://forum.archive.openwrt.org/viewtopic.php?id=69257
- https://forum.openwrt.org/t/add-support-for-tp-link-m7350-v4/132119

### OEM

- http://www.tp-link.de/products/details/cat-5032_M7350.html

### Sourcecode

- https://archive.org/download/tp-link-gpl-source/LTE/M7350/
- https://static.tp-link.com/resources/gpl/M7350v3_en_gpl.tar.gz
