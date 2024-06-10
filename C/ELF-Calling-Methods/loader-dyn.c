#include <dlfcn.h>
#include <stdio.h>

#define ADD_ADDRESS 0x10F9
#define SUB_ADDRESS 0x1111

typedef int (*FUNC_ADD) (int, int);
typedef int (*FUNC_SUB) (int, int);

int main(int argc, char* argv[]) {
    void* handle = dlopen("./hello-dyn.so", RTLD_LAZY);

    FUNC_ADD add_sym = dlsym(handle, "add");
    FUNC_ADD add_address = (FUNC_ADD)(*(long*)handle + ADD_ADDRESS);
    FUNC_SUB sub = (FUNC_SUB)(*(long*)handle + SUB_ADDRESS);

    int result = 0;
    result = add_sym(11, 2);
    printf("add_sym`(11, 2) = %d\n", result);
    result = add_address(100, 2);
    printf("add_address(100, 2) = %d\n", result);
    result = sub(9, 8);
    printf("sub(9, 8) = %d\n", result);

    dlclose(handle);

    return 0;
}

// gcc loader-dyn.c -o test_so