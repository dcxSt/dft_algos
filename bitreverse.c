#include <stdio.h>

/*
 * Takes bits of positive number and reverses them, assuming that
 * the number is nbits wide (also a positive int). 
 */
int bitreverse(int number, int nbits) {
  int reversed = 0;
  int i=1;
  while(1) {
    reversed |= (number&1);
    if(i==nbits) break;
    reversed<<=1; number>>=1;
    i++;
  }
  return reversed;
}

int main() {
  int n=1;
  printf("%i reversed %i\n",n,bitreverse(n,2));
  n=2;
  printf("%i reversed %i\n",n,bitreverse(n,2));
  n=5;
  printf("%i reversed %i\n",n,bitreverse(n,4));
  n=0;
  printf("%i reversed %i\n",n,bitreverse(n,4));
  return 0;
}


