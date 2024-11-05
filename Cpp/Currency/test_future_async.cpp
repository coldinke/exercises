#include <iostream>
#include <thread>
#include <future>
#include <chrono>


int task(int n) {
    std::cout << "Async task thread ID: " << std::this_thread::get_id() << "\n" ;
    return n * n;
}

int compute_value() {
    std::this_thread::sleep_for(std::chrono::seconds(3));
    return 42;
}

int main() {
    std::future<int> future = std::async(task, 10);
    std::cout << "main: " << std::this_thread::get_id() << "\n";
    std::cout << std::boolalpha << future.valid() << "\n";
    std::cout << future.get() << "\n";
    std::cout << std::boolalpha << future.valid() << "\n";

    std::future<int> compute_future = std::async(compute_value);
    std::cout << "Doing other task...\n";
    int result = compute_future.get();
    std::cout << "The result is " << result << "\n";
    return 0;
}