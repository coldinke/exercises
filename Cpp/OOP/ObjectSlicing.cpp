#include <iostream>

class Base {
public:
    Base() { std::cout << "Base Constructor..." << std::endl; }
    virtual ~Base() { std::cout << "Base Destructor..." << std::endl; }
    virtual void foo() { std::cout << "Base::foo()" << std::endl; }
};

class Derived : public Base {
public:
    Derived() { std::cout << "Derived Constructor..." << std::endl; }
    ~Derived() { std::cout << "Derived Destructor..." << std::endl; }
    void foo() override { std::cout << "Derived::foo()" << std::endl; }
    int value = 42;
};

int main() {
    Derived d;
    Base* ptr = &d; // 指针切割，value被切掉
    ptr->foo(); // 正确输出 Derived::foo()

    Base b = d; // 对象切割，value被切掉且Derived构造/析构未被调用
    b.foo(); // 输出Base::foo()
}