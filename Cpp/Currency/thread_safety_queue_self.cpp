#include <iostream>
#include <queue>
#include <thread>
#include <mutex>
#include <memory>
#include <condition_variable>

template <typename T>
class threadsafe_queue {
    mutable std::mutex m;
    std::condition_variable data_cond;
    std::queue<T> data_queue;

public:
    threadsafe_queue() {}
    void push(T new_value) {
        {
            std::lock_guard<std::mutex> lk { m };
            data_queue.push(new_value);
            std::cout << "push: " << new_value << "\n";
        }
        data_cond.notify_one();
    }

    void pop(T& value) {
        std::unique_lock<std::mutex> lk { m };
        data_cond.wait(lk, [this] { return !data_queue.empty(); });
        value = data_queue.front();
        std::cout << "pop: " << value << "\n";
        data_queue.pop();
    }

    std::shared_ptr<T> pop() {
        std::unique_lock<std::mutex> lk { m };
        data_cond.wait(lk, [this] {return !data_queue.empty(); });
        std::shared_ptr<T>res { std::make_shared<T>(data_queue.front()) };
        data_queue.pop();
        return res;
    }
    bool empty() const {
        std::lock_guard<std::mutex> lk(m);
        return data_queue.empty();
    }
};

void producer(threadsafe_queue<int>& q) {
    for (int i = 0; i < 5; ++i) {
        q.push(i);
    }
}

void consumer(threadsafe_queue<int>& q) {
    for (int i = 0; i < 5; ++i) {
        int value{};
        q.pop(value);
    }
}

int main() {
    threadsafe_queue<int> q;

    std::thread producer_thread(producer, std::ref(q));
    std::thread consumer_thread(consumer, std::ref(q));

    producer_thread.join();
    consumer_thread.join();
}

