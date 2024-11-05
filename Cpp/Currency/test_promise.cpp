#include <iostream>
#include <thread>
#include <future>


void calculateSquare(std::promise<int> promiseObj, int num) {
    std::this_thread::sleep_for(std::chrono::seconds(1));

    promiseObj.set_value(num * num);
}

int main() {
    std::promise<int> promiseObj;

    std::future<int> futureObj = promiseObj.get_future();

    int num = 5;
    std::thread t(calculateSquare, std::move(promiseObj), num);

    int result = futureObj.get();
    std::cout << num << " pow 2: " << result << std::endl;

    t.join();
}