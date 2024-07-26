#include <iostream>
#include <thread>
#include <string>
#include <chrono>
#include <iomanip>

using namespace std::chrono_literals;

// without arguments
void hello() {
    std::cout << "Hello world!\n";
}

// with arguments
void add(const int& a, int b) {
    std::cout << "add()\n";
    std::cout << "a's address " << &a << " b's address" << &b << "\n";
    std::cout << a + b << "\n";
}

int main() {
    unsigned int n = std::thread::hardware_concurrency();
    std::cout << "Support: " << n << " thread concurrency.\n";
    // without argu
    std::thread t{ hello };
    // with argu 
    int a = 3, b = 3;
    std::cout << "a's address " << &a << " b's address" << &b << "\n";
    std::thread t1{ add, std::ref(a), b }; // reference by arguments.
    // using std::this_thread
    // using std::this_thread::get_id()
    std::thread t3{
        [] {
            std::cout << "t3: thread_id" << std::this_thread::get_id() << "\n";
        }
    };
    // using std::this_thread::get_id()
    std::thread t4{
        [] {
            std::this_thread::sleep_for(std::chrono::seconds(3));
        }
    };
    // using std::this_thread::get_id()
    std::thread t5{
        [] {
            // 获取当前时间点
    auto now = std::chrono::system_clock::now();

    // 设置要等待的时间点为当前时间点之后的5秒
    auto wakeup_time = now + 5s;

    // 输出当前时间
    auto now_time = std::chrono::system_clock::to_time_t(now);
    std::cout << "Current time:\t\t" << std::put_time(std::localtime(&now_time), "%H:%M:%S") << std::endl;

    // 输出等待的时间点
    auto wakeup_time_time = std::chrono::system_clock::to_time_t(wakeup_time);
    std::cout << "Waiting until:\t\t" << std::put_time(std::localtime(&wakeup_time_time), "%H:%M:%S") << std::endl;

    // 等待到指定的时间点
    std::this_thread::sleep_until(wakeup_time);

    // 输出等待结束后的时间
    now = std::chrono::system_clock::now();
    now_time = std::chrono::system_clock::to_time_t(now);
    std::cout << "Time after waiting:\t" << std::put_time(std::localtime(&now_time), "%H:%M:%S") << std::endl;
}
    };

    // join
    t.join();
    t1.join();
    t3.join();
    t4.join();
    t5.join();
}