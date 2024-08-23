/*
 * Calculate blocksizes for devices found in /dev/mtd
 */

const parts = {
  "sbl"        : { begin: 0x00000000, end: 0x00140000, },
  "mibib"      : { begin: 0x00140000, end: 0x00280000, },
  "efs2"       : { begin: 0x00280000, end: 0x00d80000, },
  "sdi"        : { begin: 0x00d80000, end: 0x010e0000, },
  "tz"         : { begin: 0x010e0000, end: 0x01440000, },
  "mba"        : { begin: 0x01440000, end: 0x01500000, },
  "rpm"        : { begin: 0x01500000, end: 0x01860000, },
  "qdsp"       : { begin: 0x01860000, end: 0x04b20000, },
  "pad"        : { begin: 0x04b20000, end: 0x04b60000, },
  "appsbl"     : { begin: 0x04b60000, end: 0x04c40000, },
  "apps"       : { begin: 0x04c40000, end: 0x05680000, },
  "scrub"      : { begin: 0x05680000, end: 0x056c0000, },
  "cache"      : { begin: 0x056c0000, end: 0x09820000, },
  "misc"       : { begin: 0x09820000, end: 0x09c80000, },
  "recovery"   : { begin: 0x09c80000, end: 0x0a6e0000, },
  "fota"       : { begin: 0x0a6e0000, end: 0x0a840000, },
  "recoveryfs" : { begin: 0x0a840000, end: 0x0a880000, },
  "system"     : { begin: 0x0a880000, end: 0x0cd20000, },
  "userdata"   : { begin: 0x0cd20000, end: 0x10000000, },
};

let calculatedEntries = [];

function Entry(name, address, length, sizeKB) {

	this.name = name;
	this.address = address;
	this.length = length;
	this.sizeKB = sizeKB;
}

Object.entries(parts).forEach(([name, range], i) => {
  const size = range.end - range.begin;
  const entry = new Entry(
    name.toString(),
    "0x"+size.toString(16),
    size.toString(),
    size/1024,
  )
  calculatedEntries.push(entry);
});

console.table(calculatedEntries,
  [
    "name",
    "address",
    "length",
    "sizeKB"
  ]);

