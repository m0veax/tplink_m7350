run:
	cargo build --release --target arm-unknown-linux-gnueabi
	adb push target/arm-unknown-linux-gnueabi/release/tpl_oled_res_fbdemo /fbdemo
	adb shell /fbdemo
