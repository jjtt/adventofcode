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

  int* indices = calloc(size+1, sizeof(int));
  indices[3] = 2;
  indices[2] = 7;
  indices[7] = 4;
  indices[4] = 6;
  indices[6] = 5;
  indices[5] = 1;
  indices[1] = 8;
  indices[8] = 9;
  indices[9] = 10;
  for (int i=10; i<size; i++) {
    indices[i] = i+1;
  }
  indices[size] = 3;

  p(indices, size);

  int cur = 3;

  for (int i=0; i<rounds; i++) {
    if (i%1000 == 0) {
      printf("Round: %d\n", i);
    }
    int p1 = indices[cur];
    int p2 = indices[p1];
    int p3 = indices[p2];
    int d = dest(p1, p2, p3, cur, size);

    indices[cur] = indices[p3];
    indices[p3] = indices[d];
    indices[d] = p1;

    cur = indices[cur];
  }


  p(indices, size);

  printf("%d, %d\n", indices[1], indices[indices[1]]);
}
