# firmware

The firmware resides in a NAND flash and is split into various parts.
The exact boot flow is unclear. Notes on other Qualcomm platforms are in the
[Postmarket OS wiki](https://wiki.postmarketos.org/wiki/Category:Qualcomm).

Given the [MIBIB](https://boxmatrix.info/wiki/Property:MIBIB), we can learn a
bit more, using information encoded in the [coreboot project's Qualcoom tool](
https://github.com/coreboot/coreboot/blob/master/util/qualcomm/mbn_tools.py).

For some details on boot security, look at this [talk from BlackHat USA 2019](
https://i.blackhat.com/USA-19/Thursday/us-19-Pi-Exploiting-Qualcomm-WLAN-And-Modem-Over-The-Air.pdf).

## partition table

| #  | start        | end          | name         | meaning                   |
| -- | ------------ | ------------ | ------------ | ------------------------- |
|  0 | `0x00000000` | `0x00140000` | `sbl`        | Secondary Boot Loader     |
|  1 | `0x00140000` | `0x00280000` | `mibib`      | Multi Image Boot Info Blk |
|  2 | `0x00280000` | `0x00d80000` | `efs2`       | RAM file system           |
|  3 | `0x00d80000` | `0x010e0000` | `sdi`        |   |
|  4 | `0x010e0000` | `0x01440000` | `tz`         | Arm TrustZone firmware    |
|  5 | `0x01440000` | `0x01500000` | `mba`        | Modem Boot Authenticator  |
|  6 | `0x01500000` | `0x01860000` | `rpm`        | Resource Power Manager FW |
|  7 | `0x01860000` | `0x04b20000` | `qdsp`       | Qualcomm DSP firmware     |
|  8 | `0x04b20000` | `0x04b60000` | `pad`        |   |
|  9 | `0x04b60000` | `0x04c40000` | `appsbl`     |   |
| 10 | `0x04c40000` | `0x05680000` | `apps`       |   |
| 11 | `0x05680000` | `0x056c0000` | `scrub`      |   |
| 12 | `0x056c0000` | `0x09820000` | `cache`      |   |
| 13 | `0x09820000` | `0x09c80000` | `misc`       |   |
| 14 | `0x09c80000` | `0x0a6e0000` | `recovery`   | U-Boot derivative (?)     |
| 15 | `0x0a6e0000` | `0x0a840000` | `fota`       | Firmware Over The Air Upd |
| 16 | `0x0a840000` | `0x0a880000` | `recoveryfs` | empty (all `ffff`)        |
| 17 | `0x0a880000` | `0x0cd20000` | `system`     | Linux rootfs with `/boot` |
| 18 | `0x0cd20000` | `0x10000000` | `userdata`   | user data, settings etc   |
