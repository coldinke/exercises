#include <iostream>
#include <thread>
#include <mutex>
#include <chrono>
#include <condition_variable>

std::mutex mtx;
std::condition_variable cv;
bool arrived = false;

void wait_for_arrival() {
    std::unique_lock<std::mutex> lck(mtx);
    cv.wait(lck, [] { return arrived; });
    std::cout << "Arrivaled\n";
}

void simulate_arrival() {
    std::this_thread::sleep_for(std::chrono::seconds(5));
    {
        std::lock_guard<std::mutex> lck(mtx);
        arrived = true;
    }
    cv.notify_one();
}

int main() {
    std::thread t1(wait_for_arrival);
    std::thread t2(simulate_arrival);

    t1.join();
    t2.join();
}