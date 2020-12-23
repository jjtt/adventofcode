#include <stdio.h>
#include <stdlib.h>

int dest(int p1, int p2, int p3, int current, int size) {
  while (1) {
    current = (current - 1) % size;
    if (current == 0) current = size;
    if (current != p1 && current != p2 && current != p3) return current;
  }
}


void p(int* indices, int size) {
  for (int i=1; i<=size; i++) {
    printf("%d, ", indices[i]);
  }
  printf("\n");
}


void main(int argc, char* argv[]) {
  printf("hello, world\n");
  int rounds = atoi(argv[1]);
  int size = atoi(argv[2]);

  printf("%d %d\n", rounds, size);

  int d = dest(1, 2, 3, 4, size);
  printf("%d\n", d);

  int* indices = calloc(size, sizeof(int));
  indices[3] = 0;
  indices[8] = 1;
  indices[9] = 2;
  indices[1] = 3;
  indices[2] = 4;
  indices[5] = 5;
  indices[4] = 6;
  indices[6] = 7;
  indices[7] = 8;
  for (int i=10; i<=size; i++) {
    indices[i-1] = i;
  }
  p(indices, size);

  int head[4];
  head[0] = 3;
  head[1] = 8;
  head[2] = 9;
  head[3] = 1;

  int newhead[4];

  for (int i=0; i<rounds; i++) {
    if (i%1000 == 0) {
      printf("Round: %d\n", i);
    }
    int d = dest(head[1], head[2], head[3], head[0], size);
    int di = indices[d];

    for (int n=1; n<=size; n++) {
      if (n == head[0]) {
        indices[n] = size - 1;
      } else if (n != head[1] && n != head[2] && n != head[3]) {
        if (indices[n] <= di) {
          indices[n] = (indices[n] - 4 + size) % size;
        } else {
          indices[n] = (indices[n] - 1 + size) % size;
        }
      } else {
        indices[n] = (indices[n] + (di-4) + size) % size;
      }

      if (indices[n] < 4) {
        newhead[indices[n]] = n;
      }
    }

    head[0] = newhead[0];
    head[1] = newhead[1];
    head[2] = newhead[2];
    head[3] = newhead[3];
  }


  p(indices, size);

  int one = indices[1];
  for (int c=1; c<=2; c++) {
    for (int i=1; i<=size; i++) {
      if (indices[i] == one + c) {
        printf("%d\n", i);
        break;
      }
    } 
  }
}
