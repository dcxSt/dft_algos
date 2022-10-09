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

// Function to reverse bits of a number
unsigned int rb(unsigned int num , int r) 
{
  int reverse = 0;
  for (int i=0;i<r;i++) {
    reverse <<= 1;
    reverse |= (num&1);
    num >>= 1;
  }
  return reverse;
}

int main() {
  // TESTS
  int n=1;
  printf("%i reversed %i\n",n,bitreverse(n,2));
  n=2;
  printf("%i reversed %i\n",n,bitreverse(n,2));
  n=5;
  printf("%i reversed %i\n",n,bitreverse(n,4));
  n=0;
  printf("%i reversed %i\n",n,bitreverse(n,4));
  printf("---testing rb instead of bitreverse---\n");
  n=1;
  printf("%i reversed %i\n",n,rb(n,2));
  n=2;
  printf("%i reversed %i\n",n,rb(n,2));
  n=5;
  printf("%i reversed %i\n",n,rb(n,4));
  n=0;
  printf("%i reversed %i\n",n,rb(n,4));
  return 0;
}


