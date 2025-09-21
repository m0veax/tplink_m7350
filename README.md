# TP-Link M7350

The [TP-Link M7350](http://www.tp-link.de/products/details/cat-5032_M7350.html)
is a series of portable 4G routers roughly equivalent to the Orbic RC400L as
found in the US and also used for the [Rayhunter project](
https://efforg.github.io/rayhunter/supported-devices.html).

## Device infos

The devices are generally based on Qualcomm SoCs.

There are multiple hardware revisions. We have taken them apart and added
photos, notes on specific parts and possible hardware modification:

- [M7350 v2](README-v2.md)
- [M7350 v3](README-v3.md)
- [M7350 v4](README-v4.md)
- [M7350 v5](README-v5.md)
- [M7350 v6](README-v6.md)
- [M7350 v8](README-v8.md)
- [M7350 v9](README-v9.md)

## OEM support

Note that the sources are a bit messy. For example, we have found the display
driver for earlier devices to be in the v2 and v4 tarballs, but not in v3.

- <https://www.tp-link.com/en/support/download/m7350/>
- <https://archive.org/download/tp-link-gpl-source/LTE/M7350/>

## Community

Join our [Matrix Channel](https://matrix.to/#/!hUtDhlRLVIQJzRgCpE:zehka.net?via=yip.gay&via=matrix.org&via=chaospott.de)

## Related projects

- [Rayhunter repository](https://github.com/EFForg/rayhunter)
    - [porting to the M7350](https://github.com/m0veax/rayhunter-tplink-m7350) (obsolete)
- [femto8](https://github.com/untitaker/femto8/tree/framebuffer) - femto8 is an open-source reimplementation of the PICO-8 fantasy console adapted for the TP-Link M7350

## TOC

- [Device Infos](#device-infos)
- [OEM support](#oem-support)
- [TOC](#toc)
- [Notes](#notes)
    - [fastboot](#fastboot)
    - [mount SD card](#sdcard)
    - [Firmware](#firmware)
        - [`./system/etc/{passwd-,shadow}`](#systemetcpasswd-shadow)
        - [`./system/etc/lighttpd.user`](#systemetclighttpduser)
        - [`./system/sbin`](#systemsbin)
        - [`./META-INF/com/google/android/updater-script`](#meta-infcomgoogleandroidupdater-script)
        - [Webinterface RCE to start telnet](#telnet)
    - [.dtb files](#dtb-files)
    - [Testpoint and Bootpoint PBL](#testpoint-and-bootpoint-pbl)
    - [Backup methods](#backup-methods)
    - [Start adbd](#start-adbd)
    - [Stop adbd](#stop-adbd)
- [TODO](#todo)
- [Weblinks](#weblinks)
    - [OpenWRT discussions](#openwrt-discussions)
    - [4pda](#4pda)

## Notes

Qualcomm has different kinds of chips:

- MDM is [_Mobile Data Modem_](https://www.qualcomm.com/news/releases/2011/02/qualcomm-delivers-faster-mobile-broadband-experience-new-higher-speed-lte)
- MSM is [_Mobile Station Modem_](https://www.qualcomm.com/news/releases/1997/03/qualcomm-announces-next-generation-mobile-station-modem)
- APQ is [_Application Processor_](https://www.qualcomm.com/products/technology/processors/application-processors/apq8053) 

See also <https://www.qualcomm.com/products/technology/processors> and
<https://www.ntia.gov/files/ntia/qc_comments_on_firstnet_noi.pdf>.

The chips on the portable routers as described here are mostly MDMs.
They are powerful enough for small applications.

## Kernel

Based on the [official Linux kernel sources](https://kernel.org/) and with
vendor code `rsync`ed over, we are working on getting them to build in 2025.

<https://github.com/m0veax/tplink_m7350-kernel>

A config from a real v3 device: [`kernel/config`](kernel/config)

## fastboot

If you remove the battery and plugin USB, `lsusb` briefly shows:

```
Bus 001 Device 031: ID 18d1:d00d Google Inc. Xiaomi Mi/Redmi 2 (fastboot)
```

Then `fastboot devices` gets:

```
MDM9625	fastboot
```

The device disappears after a few seconds.

If you boot normally, it shows:

```
Bus 001 Device 032: ID 2357:0005 TP-Link M7350 4G Mi-Fi Router
```

To enter fastboot permanently (until reset), run `fastboot reboot bootloader`.

An easier way to enter fastboot is to get a telnet root shell, and then:
```
/ # reboot-bootloader
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

You can use `fastboot` to [run a custom kernel](firmware_research/README.md).

On the v2 revision, a fastboot device briefly appears on normal boot, without 
removing the battery.

## sdcard

An SD card with the stock system needs to be FAT32 formatted:

```sh
sudo mkfs.vfat -F 32 /dev/mmcblk0
```

Afterwards, via `adb shell`, run `mount /dev/mmcblk0p1 /mnt/` on the device, and
you can access the SD card at `/mnt/` now.

Bonus:

```
> usb_composition 
> 902B
```

Your SD Card will be served as usb device afterwards

## Firmware

The main system seems to be based on Android, however without `/dev/binder`.

We have extracted the root file system for a better understanding.

See [more detailed notes on the firmware](firmware_research/README.md) from
further research regarding other partitions.

### `./system/etc/{passwd-,shadow}`

```
root:C98ULvDZe7zQ2:0:0:root:/home/root:/bin/sh
```

A quick search for the hash gives us `oelinux123` as a possible value.
We have confirmed this to be the password.

Source: https://svson.xyz/posts/zte-dongle/part4/

### `./system/etc/lighttpd.user`

```
admin:admin
```

### `./system/sbin`

The firmware contains an `adbd`. [ADB access](#start-adbd) can be obtained
permanently.

It also contains a few `reboot` scripts: `reboot-recovery` and `reboot-bootloader`:

```
/ # cat /sbin/reboot-recovery
#! /bin/sh

echo 2 > /etc/reboot-cookie
reboot
/ # cat /sbin/reboot-bootloader
#! /bin/sh

echo 1 > /etc/reboot-cookie
reboot
```

### `./META-INF/com/google/android/updater-script`

This contains paths to files and creates symlinks for autostart etc.

### telnet

In the linked 4pda forum thread is a PoC for a Remote Code Execution (RCE)
vulnerability which allows to start the telnet daemon. There are only Windows
scripts linked right now. We have developed our own tools thusly.

More about this [here](webinterface_rce_telnet/README.md)

We implemented a [Rust command line tool](tp-opener/README.md) and a
[curl based shell script](open.sh). The latter performs the login automaticly.

There is a [Ruby implementation](https://github.com/ecdsa521/tpown/tree/main](
https://github.com/ecdsa521/tpown/tree/main) as well.

### .dtb files

The .dtb files of HW rev v3 and v4 are stored in [dtb_files](dtb_files/) and can be visualized with [dtvis](https://github.com/platform-system-interface/dtvis/)

### Testpoint and Bootpoint PBL

There are [photos on 4PDA from other variants of the same general board design](
assets/4pda/README.md). They are similar for our device.

### Backup methods

4PDA has found [several ways to backup the installed firmware](
https://4pda.to/forum/index.php?showtopic=669936&view=findpost&p=110738476).

### Start adbd

Via [telnet](#telnet), run the `usb_composition` command on the device as follows:

```sh
usb_composition
902B
nyy
```

Persistent `adbd` connection should be etablished now.

Now via a USB cable on your laptop, `adb shell` will get you a shell.

### Stop adbd

After a reboot, the access point seems to be down.
To deactivate `adbd` again:

```
adb shell
usb_composition
tplink
nyy
```

## TODO

- [ ] Compare Kernel 3.4.0 with TP Link Sources
- [ ] Find Qualcomm debug stuff online
- [x] implement script to start telnet based on the vuln quoted in the 4pda forum
- [ ] Explore Android / iOS App to find hidden Endpoints
- [ ] Can we do Stuff with the ISP Files from the Download Section?
- [ ] Try to get OpenWRT running on the Device
- [x] Find a way to start `adbd`
- [x] Link v3 Firmware instead of v4

## Weblinks

### OpenWRT discussions

- <http://forum.archive.openwrt.org/viewtopic.php?id=72055>
- <http://forum.archive.openwrt.org/viewtopic.php?id=69257>
- <https://forum.openwrt.org/t/add-support-for-tp-link-m7350-v4/132119>

### 4pda

- <https://4pda-to.translate.goog/forum/index.php?showtopic=669936&st=100&_x_tr_sl=auto&_x_tr_tl=de&_x_tr_hl=de&_x_tr_pto=wapp#entry95895999> (translated)
- <https://4pda.to/forum/index.php?showtopic=669936&st=100#entry95895999> (Russian)
