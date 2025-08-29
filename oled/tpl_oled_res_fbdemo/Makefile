all:
	cargo build --release --target armv7-unknown-linux-musleabihf

run:
	adb push target/arm-unknown-linux-gnueabi/release/tpl_oled_res_fbdemo /fbdemo
	adb shell /fbdemo
