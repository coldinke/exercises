#include <iostream>
#include <thread>

int global_counter = 0;
thread_local int thread_local_counter = 0;

void print_counters() {
    std::cout << "Global: " << global_counter++ << "\n";
    std::cout << "Thread Local:" << thread_local_counter++ << "\n";
}

int main() {
    std::thread{ print_counters}.join();
    std::thread{ print_counters}.join();
}