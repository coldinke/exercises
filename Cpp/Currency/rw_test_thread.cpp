#include <iostream>
#include <map>
#include <string>
#include <shared_mutex>
#include <thread>
#include <chrono>

using namespace std::chrono_literals;

class Settings {
    std::map<std::string, std::string> data_;
    mutable std::shared_mutex mutex_;

public:
    void set(const std::string& key, const std::string& value) {
        std::lock_guard<std::shared_mutex> lock(mutex_);
        data_[key] = value;
    }

    std::string get(const std::string& key) const {
        std::shared_lock<std::shared_mutex> lock{mutex_};
        const auto it = data_.find(key);
        return (it != data_.end()) ? it->second : "";
    }
};

void readSettings(const Settings& settings, const std::string& key, int threadId) {
    std::cout << "ThreadID: " << threadId << ", Reading settings, " << key << ", Value: " << settings.get(key) << "\n";
}

void writeSettings(Settings& settings, const std::string& key, const std::string& value, int threadId) {
    settings.set(key, value);
    std::cout << "ThreadID: " << threadId << ", Writing settings, " << key << " to " << value << "\n";
}


int main() {
    Settings settings;

    std::thread writer{ [&settings] {
        for (int i = 0; i < 5; ++i) {
            writeSettings(settings, "key"+std::to_string(i), "value"+std::to_string(i), 1);
            std::this_thread::sleep_for(100ms);
        }
    }};

    std::thread reader1{ [&settings] {
        for (int i = 0; i < 5; ++i) {
            readSettings(settings, "key"+std::to_string(i), 2);
            std::this_thread::sleep_for(100ms);
        }
    }};

    std::thread reader2{ [&settings] {
        for (int i = 0; i < 5; ++i) {
            readSettings(settings, "key"+std::to_string(i), 3);
            std::this_thread::sleep_for(100ms);
        }
    }};

    writer.join();
    reader1.join();
    reader2.join();
}