__attribute__((visibility("default"))) int add(int a, int b) {
    return a + b;
}

__attribute__((visibility("hidden"))) int sub(int a, int b) {
    return a - b;
}

// gcc -fPIC -shared -o hello-dyn.so hello-dyn.c
/*
From Kimi:(仅供参考)
创建共享库：当你想要编译一个库以便它可以被多个程序共享时，你需要使用 -fPIC 选项。例如：
    gcc -fPIC -shared -o libexample.so example.c
这个命令会编译 example.c 并生成一个共享库 libexample.so。
编译动态加载的模块：如果你正在编译一个模块，该模块将被动态加载到一个已经运行的程序中，你也需要使用 -fPIC。
兼容性：在某些系统或特定情况下，即使不是创建共享库，使用 -fPIC 也可能提高代码的兼容性。
*/