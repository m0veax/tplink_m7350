#!/usr/bin/env bash

mkbootimg \
	--kernel zImage-3.4.0 \
	-o mtd \
	--cmdline " hallo noinitrd root=/dev/mtdblock17 rw rootfstype=yaffs2 console=ttyHSL0,115200,n8 androidboot.hardware=qcom ehci-hcd.park=3 g-android.rx_trigger_enabled=1" \
	--header_version 0 \
	--base 0x300000 \
	--pagesize 2048 \
	--tags_offset 0x6500000

cat mtd qcdt > boot_image

# field `unused` (dtb header version?) must be != 0
echo -en '\0001' | dd of=boot_image bs=1 seek=40 conv=notrunc
