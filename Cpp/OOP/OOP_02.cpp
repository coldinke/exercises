#include <algorithm>
#include <iostream>
#include <memory>
#include <vector>

namespace MyOOP {
class Shape {
public:
  virtual void draw() const = 0;
  virtual ~Shape() = default;
};

class Circle : public Shape {
public:
  void draw() const override { std::cout << "Drawing Circle" << std::endl; }
};

class Retangle : public Shape {
public:
  void draw() const override { std::cout << "Drawing Rectangle" << std::endl; }
};

class ColorDecorator : public Shape {
protected:
  std::shared_ptr<Shape> decoratedShape;

public:
  ColorDecorator(std::shared_ptr<Shape> shape) : decoratedShape(shape) {}
};

class RedColorDecorator : public ColorDecorator {
public:
  RedColorDecorator(std::shared_ptr<Shape> shape) : ColorDecorator(shape) {}
  void draw() const override {
    decoratedShape->draw();
    std::cout << "Coloring Red\n";
  }
};

class GreenColorDecorator : public ColorDecorator {
public:
  GreenColorDecorator(std::shared_ptr<Shape> shape) : ColorDecorator(shape) {}
  void draw() const override {
    decoratedShape->draw();
    std::cout << "Coloring Green\n";
  }
};

class CompositeShape : public Shape {
private:
  std::vector<std::shared_ptr<Shape>> shapes;

public:
  void addShape(std::shared_ptr<Shape> shape) { shapes.push_back(shape); }

  void draw() const override {
    for (const auto &shape : shapes) {
      shape->draw();
    }
  }
};
} // namespace MyOOP

using namespace MyOOP;

int main() {
  auto circle = std::make_shared<Circle>();
  auto rectangle = std::make_shared<Retangle>();

  auto redCircle = std::make_shared<RedColorDecorator>(circle);
  auto greenRectangle = std::make_shared<GreenColorDecorator>(rectangle);

  CompositeShape compositeShape;
  compositeShape.addShape(redCircle);
  compositeShape.addShape(greenRectangle);

  compositeShape.draw();

  return 0;
}