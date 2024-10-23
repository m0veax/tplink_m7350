# Mandelbrot Display Demo

This is a simple demo containing a source, a demo makefile with a binary, and an strace binary to test the display output. 

## Important Notes
- This has been tested with the stock firmware, meaning no colors and no bit depth whatsoever.
- Be sure to run `/etc/init.d/start_oledd stop` before testing; otherwise, the system UI ruins the entire experience.

## Using strace
The `strace` might be useful if you need to play with the code and experience random segfaults. For me, any `printf` caused a segfault, so you need to bypass it by using `unistd`. Here's an example:

```c
#include <unistd.h>

int main() {
    const char *message = "Hello, World!
";
    write(STDOUT_FILENO, message, 14);  // Length of the message
    return 0;
}
```

The `stdio` import itself doesn't cause the segfault, so feel free to use it. 

## Building and Running
After running `make` in your terminal in this directory and ensuring your device is connected with ADB and fully functional, you should see the Mandelbrot set appear on your display.

**Note:** Be sure to stop the `oledd` service while the display is on; otherwise, you might not get it back on without the service running.
