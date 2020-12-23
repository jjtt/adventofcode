#include <stdio.h>
#include <stdlib.h>

int dest(int p1, int p2, int p3, int current, int size) {
  while (1) {
    current = (current - 1) % size;
    if (current == 0) current = size;
    if (current != p1 && current != p2 && current != p3) return current;
  }
}


void p(int* nums, int size) {
  for (int i=0; i<size; i++) {
    printf("%d, ", nums[i]);
  }
  printf("\n");
}


void main(int argc, char* argv[]) {
  printf("hello, world\n");
  int rounds = atoi(argv[1]);
  int size = atoi(argv[2]);

  int* nums = calloc(size, sizeof(int));
  nums[0] = 3;
  nums[1] = 8;
  nums[2] = 9;
  nums[3] = 1;
  nums[4] = 2;
  nums[5] = 5;
  nums[6] = 4;
  nums[7] = 6;
  nums[8] = 7;
  for (int i=10; i<=size; i++) {
    nums[i-1] = i;
  }
  p(nums, size);

  int head[4];

  int b = 0;

  for (int i=0; i<rounds; i++) {
    if (i%1000 == 0) {
      printf("Round: %d\n", i);
    }
    head[0] = nums[(0+b)%size];
    head[1] = nums[(1+b)%size];
    head[2] = nums[(2+b)%size];
    head[3] = nums[(3+b)%size];
    int d = dest(head[1], head[2], head[3], head[0], size);

    int n;
    for (n=0; n<size; n++) {
      if (nums[(b-n+size)%size] == d) {
        break;
      }
      nums[(b-n+3+size)%size] = nums[(b-n+size)%size];
    }
    nums[(b-n+3+size)%size] = head[3];
    nums[(b-n+2+size)%size] = head[2];
    nums[(b-n+1+size)%size] = head[1];
    b = (b+4+size)%size;
  }


  p(nums, size);
}
