#define _GNU_SOURCE

#include <stdio.h>
#include <dlfcn.h>

#define ADD_ADDRESS 0x1169
#define SUB_ADDRESS 0x1181

typedef int (*FUNC_ADD) (int, int);
typedef int (*FUNC_SUB) (int, int);

static int (*main_origin)(int, char **, char **);

int main_hook(int argc, char **argv, char **envp) {
    printf("main hook\n");
    printf("Bypass address(main_origin) = %p\n", main_origin);

    void* handle = dlopen(NULL, RTLD_LAZY);
    FUNC_ADD add = (FUNC_ADD)(*(long*)handle + ADD_ADDRESS);
    FUNC_SUB sub = (FUNC_SUB)(*(long*)handle + SUB_ADDRESS);

    int result = 0;
    result = add(1, 2);
    printf("add(1, 2) = %d\n", result);
    result = sub(9, 8);
    printf("sub(9, 8) = %d\n", result);

    return 0;
}

int __libc_start_main(
    int (*main)(int, char **, char **),
    int argc,
    char **argv,
    int (*init)(int, char **, char **),
    void (*fini)(void),
    void (*rtld_fini)(void),
    void *stack_end ) {
    // Save the origin main function address
    main_origin = main;

    // Find the origin __libc_start_main() address
    typeof(&__libc_start_main) origin = dlsym(RTLD_NEXT, "__libc_start_main");

    // and call it with our custom main function
    return origin(main_hook, argc, argv, init, fini, rtld_fini, stack_end);
}

// gcc -fPIC -shared loader-exec.c -o hook_dyn.so
// LD_PRELOAD=./hook_dyn.so ./test_default