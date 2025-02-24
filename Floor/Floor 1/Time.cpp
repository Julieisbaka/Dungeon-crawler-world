#include <random>

int generateRandomTime() {
  std::random_device rd;
  std::mt19937 gen(rd());
  std::normal_distribution<> d(15, 2); // Mean 15, standard deviation 2

  int randomTime = std::lround(d(gen));
  if (randomTime < 12)
    randomTime = 12;
  if (randomTime > 20)
    randomTime = 20;

  return randomTime;
}
