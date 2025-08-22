#!/bin/sh

# ImageMagick won't do it, apparently, so we use ffmpeg instead.
# https://github.com/ImageMagick/ImageMagick/discussions/2787

ffmpeg -vcodec png -i $1 -vcodec rawvideo -f rawvideo -pix_fmt rgb565 $2
