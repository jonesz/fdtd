// ricker_bug.c
#include <math.h>
#include <stdio.h>

static double cdtds, ppw = 0;

double ezInc(double time, double location) {
  double arg;
  arg = M_PI * ((cdtds * time - location) / ppw - 1.0);
  arg = arg * arg;
  return (1.0 - 2.0 * arg) * exp(-arg);
}

int main() {
  ppw = 20;
  cdtds = 1.0 / (sqrt(2.0));
  for (int i = 0; i < 300; i++) {
    printf("%lf\n", ezInc(i, 0.0));
  }
}
