#include <iostream>
#include <vector>
#include <iterator>
#include <numeric>
#include <thread>
#include <future>

template<typename ForwardIt>
auto sum(ForwardIt first, ForwardIt last) {
    using value_type = std::iter_value_t<ForwardIt>;
    std::size_t num_threads = std::thread::hardware_concurrency();
    std::ptrdiff_t distance = std::distance(first, last);

    if (distance > 1024000) {
        std::size_t chunk_size = distance / num_threads;
        std::size_t remainder = distance % num_threads;

        std::vector<std::packaged_task<value_type()>> tasks;
        std::vector<std::future<value_type>> futures(num_threads);

        std::vector<std::thread> threads;

        auto start = first;
        for (std::size_t i = 0; i < num_threads; ++i) {
            auto end = std::next(start, chunk_size + (i < remainder ? 1 : 0));
            tasks.emplace_back(std::packaged_task<value_type()>{ [start, end, i] {
                return std::accumulate(start, end, value_type{});
            }});
            start = end;
            futures[i] = tasks[i].get_future();
            threads.emplace_back(std::move(tasks[i]));
        }

        for (auto& thread : threads) {
            thread.join();
        }

        value_type total_sum {};
        for (std::size_t i = 0; i < num_threads; ++i) {
            total_sum += futures[i].get();
        }
        return total_sum;
    }

    value_type total_sum = std::accumulate(first, last, value_type{});
    return total_sum;
}

int main() {
    std::vector<std::string> vecs{ "1", "2", "3", "4" };
    auto result = sum(vecs.begin(), vecs.end());
    std::cout << result << '\n';

    vecs.clear();
    for (std::size_t i = 0; i <= 1024001u; ++i) {
        vecs.push_back(std::to_string(i));
    }
    result = sum(vecs.begin(), vecs.end());
    std::cout << result << '\n';
}