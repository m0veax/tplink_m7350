# Device Infos

TP Link Router M7350 v3

CPU in v3: Qualcomm MSM 9625 (Flattened Device Tree), model: Qualcomm MSM 9625V2.1 MTP

There are later hardware revisions:

- [M7350 v4](README-v4.md)

# Community

Join our [Matrix Channel](https://matrix.to/#/!hUtDhlRLVIQJzRgCpE:zehka.net?via=yip.gay&via=matrix.org&via=chaospott.de)

## TOC

- [Device Infos](#device-infos)
  - [TOC](#toc)
  - [Photos](#photos)
  - [Notes](#notes)
    - [fastboot](#fastboot)
    - [Firmware](#firmware)
      - [binwalk](#binwalk)
    - [Findings](#findings)
      - [`./system/etc/{passwd-,shadow}`](#systemetcpasswd-shadow)
      - [`./system/etc/lighttpd.user`](#systemetclighttpduser)
      - [`./system/sbin`](#systemsbin)
      - [`./META-INF/com/google/android/updater-script`](#meta-infcomgoogleandroidupdater-script)
      - [Webinterface RCE to start telnet](#webinterface-rce-to-start-telnet)
    - [.dtb files](#dtb-files)
    - [Testpoint and Bootpoint PBL](#testpoint-and-bootpoint-pbl)
    - [Backup methods](#backup-methods)
    - [Start adbd](#start-adbd)
    - [Stop adbd](#stop-adbd)
  - [TODO](#todo)
  - [Weblinks](#weblinks)
    - [OpenWRT Board](#openwrt-board)
    - [4pda](#4pda)
    - [OEM](#oem)
    - [Sourcecode](#sourcecode)

## Photos

![top](assets/v3-top.jpg)

![bottom](assets/v3-bottom.jpg)

## Notes

### board components

SoC: [Qualcomm MDM9225](https://www.qualcomm.com/products/technology/modems/snapdragon-modems-4g-lte-x5)

Quick note on Qualcomm terms:
- MDM is [_Mobile Data Modem_](https://www.qualcomm.com/news/releases/2011/02/qualcomm-delivers-faster-mobile-broadband-experience-new-higher-speed-lte)
- MSM is [_Mobile Station Modem_](https://www.qualcomm.com/news/releases/1997/03/qualcomm-announces-next-generation-mobile-station-modem)

Flash: 2Gbit (256MB) [Winbond W71NW20GF3FW](https://www.winbond.com/hq/product/code-storage-flash-memory/nand-based-mcp/index.html?__locale=en&partNo=W71NW20GF3FW)

mobile wireless: [Skyworks SKY77629](https://www.skyworksinc.com/Products/Amplifiers/SKY77629)

### kernel

Based on the official sources from kernel.org and with vendor code `rsync`ed
over, we are working on getting this to build in 2024.

<https://github.com/m0veax/tplink_m7350-kernel>

A config from a real device: [`kernel/config`](kernel/config)

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

Qualcomm [documents their fastboot commands](
https://docs.qualcomm.com/bundle/publicresource/topics/80-70014-4/fastboot.html).

The following `getvar` commands yield results:

| variable            | result           |
| ------------------- | ---------------- |
| `version`           | `0.5`            |
| `kernel`            | `lk`             |
| `max-download-size` | `0x2f00000`      |
| `product`           | `MDM9625`        |
| `serialno`          | `MDM9625`        |

### Display

The display is attached to the SPI bus via the controller at `0xf992_4000`.
We could extract that information from the device trees.

The SPI controller compat string is `qcom,spi-qup-v2`.
In the vendor kernel, the SPI driver is `drivers/spi/spi_qsd.c`.
In mainline Linux, it is `drivers/spi/spi-qup.c`.

The OLED display is called
- `tplink,oleds90319` (node behind `qcom,spi-qup-v2`)
- `tp,oled_pt` -> `qcom,oled_s90319_pt`

Indeed, the side of the display (white frame) reads: `BLB-S90319B-1`

There is no such thing in the vendor kernel sources, nor do Google or Bing yield
anything. So it is unclear what exactly the display driver is.
It looks like a 128x128 monochrome display, similar to _SSD1306_ / _SH1107_.

From the kernel config we dumped:
```
CONFIG_OLED=y
# CONFIG_OLED_SSD1306_PT is not set
CONFIG_OLED_S90319_PT=y
```

The binary `/usr/bin/oledd` is started via `/etc/init.d/start_oledd`.
It accesses the OLED display via sysfs:

- `/sys/class/display/oled/backlight_on`
- `/sys/class/display/oled/panel_on`
- `/sys/class/display/oled/oled_buffer`

We can echo `1` / `0` to the `*_on` files to play with the display.
And we can write to the buffer ourselves, though how it works is not yet clear.
Playing around showed that the display panel really supports colors. :rainbow:

If you want to have some fun:

```sh
/etc/init.d/start_oledd stop
echo 1 > /sys/class/display/oled/backlight_on
echo 1 > /sys/class/display/oled/panel_on
cat /dev/random > /sys/class/display/oled/oled_buffer
```

This will endlessly draw rectangles and show pixel garbage. Press Ctrl+C to stop.

<details>
  <summary>DeviceTree excerpt</summary>

```
  spi@f9924000 {
    compatible = "qcom,spi-qup-v2";
    reg = <0xf9924000 0x1000>;
    interrupts = <0x00 0x60 0x00>;
    spi-max-frequency = <0x17d7840>;
    #address-cells = <0x01>;
    #size-cells = <0x00>;
    gpios = <0x02 0x07 0x00 0x02 0x05 0x00 0x02 0x04 0x00>;
    cs-gpios = <0x02 0x06 0x00>;

    qcom-spi-oled@1 {
      compatible = "tplink,oleds90319";
      reg = <0x01>;
      spi-max-frequency = <0x927c00>;
    };
  };

  oled {
    compatible = "tp,oled_pt";

    qcom,oled_s90319 {
      compatible = "qcom,oled_s90319_pt";
      qcom,oled-cs-gpio = <0x02 0x06 0x00>;
      qcom,oled-rsx-gpio = <0x02 0x15 0x00>;
      qcom,oled-reset-gpio = <0x02 0x14 0x00>;
      qcom,oled-vdd0-gpio = <0x02 0x16 0x00>;
      qcom,oled-vdd1-gpio = <0x02 0x17 0x00>;
      qcom,oled-boost-en-gpio = <0x02 0x3d 0x00>;
    };
  };
```
</details>

<details>
  <summary>kernel log excerpt</summary>

```
[    2.042245] s90319_spi_probe successed!
[    2.045067] oled_90319_panel_init success.
[    2.049204] oled_probe
[    2.051692] oled_s90319_probe
[    2.054716] oled init success!
```
</details>

### Firmware

Device seems to run Android, without `/dev/binder`. You can get the firmware here:

[TP-Link Support Page](https://www.tp-link.com/de/support/download/m7350/#Firmware)

The Firmware is not crypted. You are able to take a deeper look into the configs.

#### binwalk

```
binwalk boot.img

DECIMAL       HEXADECIMAL     DESCRIPTION
--------------------------------------------------------------------------------
0             0x0             Android bootimg, kernel size: 3564792 bytes, kernel addr: 0x308000, ramdisk size: 0 bytes, ramdisk addr: 0x308000, product name: ""
2048          0x800           Linux kernel ARM boot executable zImage (little-endian)
18403         0x47E3          gzip compressed data, maximum compression, from Unix, last modified: 1970-01-01 00:00:00 (null date)
3567616       0x367000        Qualcomm device tree container, version: 1, DTB entries: 55
3569664       0x367800        Flattened device tree, size: 49302 bytes, version: 17
3620864       0x374000        Flattened device tree, size: 49218 bytes, version: 17
3672064       0x380800        Flattened device tree, size: 49088 bytes, version: 17
3721216       0x38C800        Flattened device tree, size: 48730 bytes, version: 17
3770368       0x398800        Flattened device tree, size: 49193 bytes, version: 17
3821568       0x3A5000        Flattened device tree, size: 48516 bytes, version: 17
3870720       0x3B1000        Flattened device tree, size: 47693 bytes, version: 17
```

### Findings

#### `./system/etc/{passwd-,shadow}`

```
root:C98ULvDZe7zQ2:0:0:root:/home/root:/bin/sh
```

Quick search for the hash gives us `oelinux123` as a possible value. We need to check that later.

Source: https://svson.xyz/posts/zte-dongle/part4/

#### `./system/etc/lighttpd.user`

```
admin:admin
```

#### `./system/sbin`

Firmware seems to contain an `adbd`. We need to find a way to start it.

#### `./META-INF/com/google/android/updater-script`

Paths to Files and creating symlinks for autostart ect. Lets try to modify that to activate adb.

#### Webinterface RCE to start telnet
In the linked 4pda forum thread is a poc for a Remote Code Execution vuln which allows to start the telnet daemon. There are only windows scripts linked right now. We should build a shellscript to invoke it.

More about this [here](webinterface_rce_telnet/README.md)

We implemented a [rust command line tool](tp-opener/README.md) and a [curl based shell script](open.sh).

With [open.sh](open.sh) the login is done automaticly.

There is a ruby implementation too [https://github.com/ecdsa521/tpown/tree/main](https://github.com/ecdsa521/tpown/tree/main)

### .dtb files

The .dtb files of HW rev v3 and v4 are stored in [dtb_files](dtb_files/) and can be visualized with [dtvis](https://github.com/platform-system-interface/dtvis/)

### Testpoint and Bootpoint PBL

There has been posted images on 4PDA to points in another revisions. Could be the same for our device. Take a look [here](assets/4pda/README.md)

### Backup methods

4PDA has found several ways to backup the installed firmware.

[https://4pda.to/forum/index.php?showtopic=669936&view=findpost&p=110738476](https://4pda.to/forum/index.php?showtopic=669936&view=findpost&p=110738476)

### Start adbd


```
usb_composition
902B
nyy
```

persistent adbd connection should be etablished now

Per cable on your client:

```
adb shell
```

### Stop adbd

After a reboot, the access point seems to be down. So you need to deactivate adbd again

```
adb shell
usb_composition
tplink
nyy
```


## TODO

- [ ] Compare Kernel 3.4.0 with TP Link Sources
- [ ] Find Qualcomm debug stuff online
- [x] implement script to start telnet based on the vuln quoted in the 4pad forum
- [ ] Explore Android / iOS App to find hidden Endpoints
- [ ] Can we do Stuff with the ISP Files from the Download Section?
- [ ] Try to get OpenWRT running on the Device
- [x] Find a way to start `adbd`
- [x] Link v3 Firmware instead of v4


## Weblinks

### OpenWRT Board
- http://forum.archive.openwrt.org/viewtopic.php?id=72055
- http://forum.archive.openwrt.org/viewtopic.php?id=69257
- https://forum.openwrt.org/t/add-support-for-tp-link-m7350-v4/132119

### 4pda
- https://4pda-to.translate.goog/forum/index.php?showtopic=669936&st=100&_x_tr_sl=auto&_x_tr_tl=de&_x_tr_hl=de&_x_tr_pto=wapp#entry95895999 (translated)
- https://4pda.to/forum/index.php?showtopic=669936&st=100#entry95895999 (russian)

### OEM

- http://www.tp-link.de/products/details/cat-5032_M7350.html

### Sourcecode

- https://archive.org/download/tp-link-gpl-source/LTE/M7350/
- https://static.tp-link.com/resources/gpl/M7350v3_en_gpl.tar.gz
