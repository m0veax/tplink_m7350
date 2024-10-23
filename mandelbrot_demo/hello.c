#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <unistd.h> // For close(), sleep()
#include <fcntl.h>  // For open()
#include <math.h>   // For sin(), cos()

#define WIDTH 128
#define HEIGHT 128
#define OLED_BUFFER_PATH "/sys/class/display/oled/oled_buffer"
#define BUFFER_SIZE (WIDTH * HEIGHT / 8) // 1024 bytes for 128x128 bitmap

// Function to generate a Mandelbrot set with zoom and pan
void generate_mandelbrot(uint8_t *buffer, double center_real, double center_imaginary, double zoom) {
    const int max_iterations = 64;
    // Create a temporary buffer for the dithering
    int temp_buffer[WIDTH][HEIGHT] = {0}; // Temporary buffer to store pixel values

    for (int y = 0; y < HEIGHT; y++) {
        for (int x = 0; x < WIDTH; x++) {
            // Map pixel position to a point in the complex plane
            double real = center_real + (x * (3.0 / zoom) / WIDTH) - (3.0 / (2 * zoom));
            double imaginary = center_imaginary + (y * (3.0 / zoom) / HEIGHT) - (3.0 / (2 * zoom));

            double z_real = real;
            double z_imaginary = imaginary;
            int iteration;

            // Determine if the point is in the Mandelbrot set
            for (iteration = 0; iteration < max_iterations; iteration++) {
                double z_real2 = z_real * z_real;
                double z_imaginary2 = z_imaginary * z_imaginary;

                if (z_real2 + z_imaginary2 > 4.0) {
                    break; // Point is outside the Mandelbrot set
                }
                z_imaginary = 2.0 * z_real * z_imaginary + imaginary; // z = z^2 + c
                z_real = z_real2 - z_imaginary2 + real;
            }

            // Determine the pixel's color based on the number of iterations
            // Calculate the pixel value (0 for black, 255 for white)
            int pixel_value = (iteration == max_iterations) ? 0 : 255;

            // Apply Floyd-Steinberg dithering
            int old_pixel_value = temp_buffer[x][y];
            int new_pixel_value = old_pixel_value + pixel_value; // Add the new pixel value
            if (new_pixel_value < 0) new_pixel_value = 0;
            if (new_pixel_value > 255) new_pixel_value = 255;
            temp_buffer[x][y] = new_pixel_value;

            // Calculate error
            int error = pixel_value - old_pixel_value;

            // Distribute the error to neighboring pixels
            if (x < WIDTH - 1) temp_buffer[x + 1][y] += error * 7 / 16;   // Right
            if (x > 0 && y < HEIGHT - 1) temp_buffer[x - 1][y + 1] += error * 3 / 16; // Bottom-left
            if (y < HEIGHT - 1) temp_buffer[x][y + 1] += error * 5 / 16;  // Bottom
            if (x < WIDTH - 1 && y < HEIGHT - 1) temp_buffer[x + 1][y + 1] += error * 1 / 16; // Bottom-right
        }
    }

    // Convert temporary buffer to the final display buffer
    for (int y = 0; y < HEIGHT; y++) {
        for (int x = 0; x < WIDTH; x++) {
            int byte_index = (y * WIDTH + x) / 8;
            int bit_index = (y * WIDTH + x) % 8;

            // Set the corresponding bit in the display buffer
            if (temp_buffer[x][y] > 128) { // Threshold for setting white
                buffer[byte_index] |= (1 << (7 - bit_index)); // Set bit to 1 (white)
            } else {
                buffer[byte_index] &= ~(1 << (7 - bit_index)); // Set bit to 0 (black)
            }
        }
    }
}


int main() {
    // Allocate memory for the header and the display buffer
    uint8_t *buffer = (uint8_t *)calloc(BUFFER_SIZE + 4, sizeof(uint8_t));
    if (!buffer) {
        fprintf(stderr, "Memory allocation failed!\n");
        return 1;
    }

    // Set the header: 00 00 80 80
    buffer[0] = 0x00;
    buffer[1] = 0x00;
    buffer[2] = 0x80; // Width = 128
    buffer[3] = 0x80; // Height = 128

    double center_real = -0.8; // Initial center of the Mandelbrot set
    double center_imaginary = 0.2; // Initial center of the Mandelbrot set
    double zoom = 1.0; // Initial zoom level

    // Animate the Mandelbrot set
    for (int i = 0; i < 100; i++) { // Change the loop count for longer animation
        // Generate the Mandelbrot set with the current zoom level and center
        generate_mandelbrot(buffer + 4, center_real, center_imaginary, zoom);

        // Open the OLED buffer file for writing
        int fd = open(OLED_BUFFER_PATH, O_WRONLY);
        if (fd < 0) {
            perror("Error opening OLED buffer");
            free(buffer);
            return 1;
        }

        // Write the buffer (header + bitmap data) to the OLED buffer
        ssize_t bytes_written = write(fd, buffer, BUFFER_SIZE + 4);
        if (bytes_written < 0) {
            perror("Error writing to OLED buffer");
        }

        // Close the file descriptor
        close(fd);

        // Adjust zoom level and center for the next frame
        zoom *= 1.05; // Increase zoom
        center_real += 0.01 * (rand() % 3 - 1); // Random pan effect
        center_imaginary += 0.01 * (rand() % 3 - 1); // Random pan effect

        usleep(50000); // Sleep for 50 ms for animation speed
    }

    free(buffer);
    return 0;
}
