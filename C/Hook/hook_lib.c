// hook_printf.c
#include <stdio.h>
#include <stdarg.h>
#include <dlfcn.h>

// 定义原始 printf 函数的函数指针类型
typedef int (*original_printf_t)(const char *format, ...);
static original_printf_t original_printf = NULL;

// 自定义 printf 函数
__attribute__((visibility("default")))
int my_printf(const char *format, ...) {
    if (!original_printf) {
        // 获取原始 printf 函数的地址
        original_printf = (original_printf_t)dlsym(RTLD_NEXT, "printf");
    }

    // 自定义逻辑：在输出原始内容之前添加 "Hooked: "
    va_list args;    
    va_start(args, format);

    char new_format[1024];
    snprintf(new_format, sizeof(new_format), "[HOOK] %s", format);

    int result = original_printf(new_format, args);
    va_end(args);

    return result;
}

// 在动态库加载时自动执行的初始化函数
__attribute__((constructor))
void init() {
    printf("Hook library loaded.\n");
}
