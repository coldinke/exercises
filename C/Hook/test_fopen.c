#include <stdio.h>

int main() {
    FILE *file = fopen("test.txt", "w");
    if (file) {
        printf("File opened successfully.\n");
        fprintf(file, "Hello world");
        fclose(file);
    } else {
        printf("Failed to open file.\n");
    }
    return 0;
}
