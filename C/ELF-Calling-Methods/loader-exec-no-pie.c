#define _GNU_SOURCE

#include <stdio.h>
#include <dlfcn.h>

static int(*main_origin)(int, char**, char**);

int add_hook(int a, int b) {
    printf("Hooked add(%d, %d)\n", a, b);
    return a + b + 10;
}

int sub_hook(int a, int b) {
    printf("Hooked sub(%d, %d)\n", a, b);
    return a - b - 10;
}

int hook_main(int argc, char **argv, char **envp) {
    printf("hooked main\n");

    int result = 0;
    result = add_hook(1, 2);
    printf("add(1, 2) + 10 = %d\n", result);

    return 0;
}

int __libc_start_main(
    int (*main)(int, char **, char **),
    int argc,
    char **argv,
    int (*init)(int, char **, char **),
    void (*fini)(void),
    void (*rtld_fini)(void),
    void *stack_end) {
    // Save the real main function address
    main_origin = main;

    // Find the real __libc_start_main()...
    typeof(&__libc_start_main) orig = dlsym(RTLD_NEXT, "__libc_start_main");

    // ... and call it with our custom main function
    return orig(hook_main, argc, argv, init, fini, rtld_fini, stack_end);
}

// gcc -fPIC -shared loader-exec-no-pie.c -o hook_static.so
// LD_PRELOAD=./hook_static.so ./test_no_pie