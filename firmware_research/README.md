# firmware

The firmware resides in a NAND flash and is split into various parts.
The exact boot flow is unclear. Notes on other Qualcomm platforms are in the
[Postmarket OS wiki](https://wiki.postmarketos.org/wiki/Category:Qualcomm).

For some details on boot security, look at this [talk from BlackHat USA 2019](
https://i.blackhat.com/USA-19/Thursday/us-19-Pi-Exploiting-Qualcomm-WLAN-And-Modem-Over-The-Air.pdf).

## partition table

Given the [MIBIB](https://boxmatrix.info/wiki/Property:MIBIB), we can learn a
bit more, using information encoded in the [coreboot project's Qualcoom tool](
https://github.com/coreboot/coreboot/blob/master/util/qualcomm/mbn_tools.py).

The partition table partially looks like the one in Yuriy Serdyuk and Alexey
Kondikov's [presentation from HITB 2023](
https://conference.hitb.org/hitbsecconf2023hkt/materials/D1T1%20-%20FrankeNAND%20%E2%80%93%20Extracting%20Info%20From%20Automotive%20Internet%20Units%20-%20Alexey%20Kondikov.pdf).

A few partitions are named as per the [Android architecture specification](
https://source.android.com/docs/core/architecture/partitions).

| #  | start        | end          | name         | meaning                   |
| -- | ------------ | ------------ | ------------ | ------------------------- |
|  0 | `0x00000000` | `0x00140000` | `sbl`        | Secondary Boot Loader     |
|  1 | `0x00140000` | `0x00280000` | `mibib`      | Multi Image Boot Info Blk |
|  2 | `0x00280000` | `0x00d80000` | `efs2`       | RAM file system           |
|  3 | `0x00d80000` | `0x010e0000` | `sdi`        | Secure Debug Image        |
|  4 | `0x010e0000` | `0x01440000` | `tz`         | Arm TrustZone firmware    |
|  5 | `0x01440000` | `0x01500000` | `mba`        | Modem Boot Authenticator  |
|  6 | `0x01500000` | `0x01860000` | `rpm`        | Resource Power Manager FW |
|  7 | `0x01860000` | `0x04b20000` | `qdsp`       | Qualcomm DSP firmware     |
|  8 | `0x04b20000` | `0x04b60000` | `pad`        | Padding, empty (`ffff`)   |
|  9 | `0x04b60000` | `0x04c40000` | `appsbl`     | Applications Bootloader   |
| 10 | `0x04c40000` | `0x05680000` | `apps`       | Android image, kernel etc |
| 11 | `0x05680000` | `0x056c0000` | `scrub`      | empty (all `ffff`)        |
| 12 | `0x056c0000` | `0x09820000` | `cache`      | Android cache (?)         |
| 13 | `0x09820000` | `0x09c80000` | `misc`       | Android misc (?)          |
| 14 | `0x09c80000` | `0x0a6e0000` | `recovery`   | U-Boot derivative (?)     |
| 15 | `0x0a6e0000` | `0x0a840000` | `fota`       | Firmware Over The Air Upd |
| 16 | `0x0a840000` | `0x0a880000` | `recoveryfs` | empty (all `ffff`)        |
| 17 | `0x0a880000` | `0x0cd20000` | `system`     | Linux rootfs with `/boot` |
| 18 | `0x0cd20000` | `0x10000000` | `userdata`   | user data, settings etc   |

Some partitions can be dumped via `adb pull /dev/mtd${PART_NO}ro`.
Partitins 1-7 fail, unfortunately, causing a reset. The firmware probably sets
up read protection on those ranges for security/obscurity reasons.

## boot flow

Qualcomm document a flow that can be roughly mapped to the above partitions:
https://www.qualcomm.com/news/onq/2017/01/secure-boot-and-image-authentication-mobile-tech

Roughly: `mask ROM --> SPL --> Applications Bootloader --> OS`

The applications boot loader is apparently a derivative of Little Kernel (lk).
See https://github.com/littlekernel/lk/blob/7538a6df673de6b73221fdbd1045928615673413/top/main.c#L84
Those strings also appear in partition 9, `"welcome to lk"`, `"bootstrap2"`,
and some others. This is where `fastboot` is implemented and loads the kernel
as an Android image. Partition 10 holds such a kernel image, and it has the
cmdline in it that we also see in-system, except for the additional trailing
`androidboot.serialno=MDM9625 androidboot.baseband=msm`.
To compare, run: `adb shell "cat /proc/cmdline"`.

See also:
https://web.archive.org/web/20160402060151/https://developer.qualcomm.com/qfile/28821/lm80-p0436-1_little_kernel_boot_loader_overview.pdf

From Qualcomm's LK boot loader docs, we learn that `fastboot oem unlock` allows
us to run `fastboot boot`. This works; see also `fastboot oem device-info`.

Using a hex editor, we can modify the cmdline at the top of partition 10.
For a simple test, change `ttyHSL0` into `ttyHSL1`. It won't matter for us.
Then boot it: `fastboot boot mtd10ro`
And lo and behold, you will now see `ttyHSL1` in the cmdline. :tada:
