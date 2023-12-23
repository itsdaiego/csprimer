#include <stdio.h>


//print depth
//print address of stack
void call_itself(int counter) {
  counter++;

  printf("counter: %d\n", counter);
  printf("stack address: %p\n", &counter);
  printf("--------------------\n");

  call_itself(counter);
}

int main(int argc, char *argv[]) {
  call_itself(0);
  return 0;
}
