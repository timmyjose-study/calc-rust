#include "lib.rs.h"
#include <iostream>

using namespace com::bridge;

int main(int argc, char *argv[]) {
  int x, y;
  std::cin >> x >> y;

  std::cout << x << +" + " << y << " = " << demo::add(x, y) << std::endl;
  std::cout << x << +" - " << y << " = " << demo::sub(x, y) << std::endl;
  std::cout << x << +" * " << y << " = " << demo::mul(x, y) << std::endl;
  std::cout << x << +" / " << y << " = " << demo::div(x, y) << std::endl;

  return 0;
}
