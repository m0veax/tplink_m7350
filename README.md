# Device Infos

TP Link Router

## Photos

![up](assets/up.jpg)

![down](assets/down.jpg)

## Notes

If you keep the Powerbutton pressed and plugin USB, lsusb shows:

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

Device seems to run Android. You can get the firmware here:

https://static.tp-link.com/upload/firmware/2022/202206/20220607/M7350(EU)_V4_220606.zip



## Weblinks

### OpenWRT Board
- http://forum.archive.openwrt.org/viewtopic.php?id=72055
- http://forum.archive.openwrt.org/viewtopic.php?id=69257
- https://forum.openwrt.org/t/add-support-for-tp-link-m7350-v4/132119

### OEM

- http://www.tp-link.de/products/details/cat-5032_M7350.html

### Sourcecode

https://archive.org/download/tp-link-gpl-source/LTE/M7350/