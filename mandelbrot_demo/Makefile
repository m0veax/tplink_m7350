# Makefile to compile mandel.c for ARM, remove old file, push new file to device and run it

# Variables
TARGET = mandel
SOURCE = mandel.c
CC = arm-linux-gnueabihf-gcc
ADB = adb
DEVICE_PATH = /home/root/$(TARGET)

.PHONY: all clean push run

# Default target
all: clean compile push run

# Clean up any existing mandel binary
clean:
	@echo "Removing existing mandel binary..."
	@rm -f $(TARGET)

# Compile the source file
compile:
	@echo "Compiling $(SOURCE)..."
	$(CC) -o $(TARGET) $(SOURCE) -static

# Remove the old file from the device
push:
	@echo "Removing existing file on device..."
	$(ADB) shell rm -f $(DEVICE_PATH)

# Push the new binary to the device
push:
	@echo "Pushing new mandel binary to device..."
	$(ADB) push $(TARGET) $(DEVICE_PATH)

# Run the new binary on the device
run:
	@echo "Running mandel on the device..."
	$(ADB) shell $(DEVICE_PATH)

# Clean up
clean:
	@echo "Cleaning up..."
	@rm -f $(TARGET)
