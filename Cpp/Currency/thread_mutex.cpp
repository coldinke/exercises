#include <iostream>
#include <string>
#include <thread>
#include <mutex>

std::mutex m;
std::mutex mtx;

// using mutex;
void f() {
    m.lock();
    std::cout << "current thread id: " << std::this_thread::get_id() << '\n';
    m.unlock();
}

// using lock_guard;
void f1() {
    std::lock_guard<std::mutex> lc { m };
    std::cout << std::this_thread::get_id() << "\n";
}

void thread_function(int id) {
    // std::mutex::try_lock();
    if (mtx.try_lock()) {
        std::cout << "Thread: " << id << " get the mutex lock\n";
        std::this_thread::sleep_for(std::chrono::milliseconds(100));
        mtx.unlock();
        std::cout << "Thread: " << id << " release the mutex lock\n";
    }
    else {
        std::cout << "Thread: " << id << " get the mutex lock failed.\n";
    }
}

int main() {
    std::vector<std::thread> threads;
    for (std::size_t i = 0; i < 10; ++i) {
        threads.emplace_back(f1);
    }

    for (auto& thread : threads) {
        thread.join();
    }

    std::thread t1(thread_function, 1);
    std::thread t2(thread_function, 2);

    t1.join();
    t2.join();
}