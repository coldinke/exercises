#include <iostream>
#include <thread>
#include <mutex>
#include <assert.h>

int n = 0;
std::mutex n_mutex;

int add(int  times) {
  for (int i = 0; i < times; i++) {
    std::lock_guard<std::mutex> lock_guard(n_mutex);
    n += 1;
  }
  return 0;
}

int main(void) {
  std::thread t1(add, 1000);
  std::thread t2(add, 1000);

  t1.join();
  t2.join();

  assert (n == 2000);
  return 0;
}
