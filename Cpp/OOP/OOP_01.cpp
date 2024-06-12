#include <exception>
#include <iostream>
#include <stdexcept>
#include <string>
#include <system_error>

namespace MyOOP {
class Person {
public:
  Person(const std::string &name, int age) : name(name), age(age) {}

  std::string getName() const { return name; }

  void setName(const std::string newName) { name = newName; }

  int getAge() const { return age; }

  void setAge(int newAge) {
    if (newAge >= 0) {
      age = newAge;
    } else {
      throw std::invalid_argument("Age cannot be negative");
    }
  }

  void printInfo() const {
    std::cout << "Name: " << name << ", Age: " << age << std::endl;
  }

private:
  std::string name;
  int age;
};
} // namespace MyOOP

int main(int argc, char *argv[]) {

  try {

    MyOOP::Person person("Alice", 30);

    person.printInfo();

    person.setAge(20);

    person.printInfo();

    person.setAge(-8);
  } catch (const std::exception &ex) {
    std::cerr << "Error: " << ex.what() << std::endl;
  }

  return 0;
}