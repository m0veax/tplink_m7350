#!/bin/sh

RES_FILE=./oled_res

PARSER_BIN=tpl_oled_res_parser
VIEWER_BIN=tpl_oled_res_viewer
TARGET_DIR=target/release

PARSER=$PARSER_BIN/$TARGET_DIR/$PARSER_BIN
VIEWER=$VIEWER_BIN/$TARGET_DIR/$VIEWER_BIN

$PARSER --extract $RES_FILE

for f in res/*.res; do
  b=$(basename $f .res)
  w=$(echo $b | cut -d'_' -f2)
  h=$(echo $b | cut -d'_' -f3)
  $VIEWER ${f} ${w} ${h} > res/$b.txt
done

echo Conversion done, here is a sample:

cat res/150_87_32.txt
