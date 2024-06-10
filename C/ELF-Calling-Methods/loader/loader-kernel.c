#include <linux/init.h>
#include <linux/module.h>

MODULE_LICENSE("GPL");

typedef int (*FUNC_ADD) (int a, int b);
typedef int (*FUNC_SUB) (int a, int b);

static int loader_init(void) {
    printk("loader-kernel init...\n");

    FUNC_ADD add = (FUNC_ADD);
    FUNC_SUB sub = (FUNC_SUB);

    int result = 0;
    result = add(1, 2);
    printk("loader-kernel add(1, 2) = %d\n", result);
    result = sub(9, 8);
    printk("loader-kernel sub(9, 8) = %d\n", result);

    return 0;
}

static void loader_exit(void) {
    printk("loader-kernel exit...\n");
}

module_init(loader_init);
module_exit(loader_exit);