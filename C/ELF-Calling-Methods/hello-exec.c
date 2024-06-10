#include <stdio.h>

int add(int a, int b) {
    return a + b;
}

int sub(int a, int b) {
    return a - b;
}

int main(int argc, char* argv[]) {
    printf("[hello-exec]\n");

    int c = add(1, 2);
    printf("add(1, 2) = %d\n", c);

    int d = sub(2, 4);
    printf("sub(2, 4) = %d\n", d);

    return 0;
}

// Position Independent Executable (default: -fPIC)
// gcc hello-exec.c -o test_default
// Absolute Address Code
// gcc -no-pie hello-exec.c -o test_no_pie