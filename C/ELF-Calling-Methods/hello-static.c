__attribute__((visibility("default"))) int add(int a, int b) {
    return a + b + 10;
}

__attribute__((visibility("hidden"))) int sub(int a, int b) {
    return a - b;
}

// gcc -c hello-static.c -o hello-static.o
// ar -rcs test.a hello-static.o