#include <iostream>
#include <vector>
#include <numeric>
#include <iterator>
#include <thread>

template<typename ForwardIt>
auto sum(ForwardIt first, ForwardIt last) {
    using value_type = std::iter_value_t<ForwardIt>;
    std::size_t num_threads = std::thread::hardware_concurrency();
    std::ptrdiff_t distance = std::distance(first, last);

    if (distance > 1024000) {
        std::size_t chunk_size = distance / num_threads;
        std::size_t remainder = distance % num_threads;

        std::vector<value_type> results{ num_threads };
        std::vector<std::thread> threads;

        auto start = first;
        for (std::size_t i = 0; i < num_threads; ++i) {
            auto end = std::next(start, chunk_size + (i < remainder ? 1 : 0));
            threads.emplace_back([start, end, &results, i] {
                results[i] = std::accumulate(start, end, value_type{});
            });
            start = end;
        }

        for (auto& thread : threads) {
            thread.join();
        }

        value_type total_sum = std::accumulate(results.begin(), results.end(), value_type{});
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
    for (std::size_t i = 0; i <= 10240001u; ++i) {
        vecs.push_back(std::to_string(10));
    }
    result = sum(vecs.begin(), vecs.end());
    std::cout << result << '\n';
}