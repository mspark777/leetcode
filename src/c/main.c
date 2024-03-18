#include <stdio.h>

void swap(int* a, int* b) {
  int l = a[0];
  int r = a[1];
  a[0] = b[0];
  a[1] = b[1];
  b[0] = l;
  b[1] = r;
}

int partition(int** points, int low, int high) {
  int pivot = points[high][1];
  int i = low - 1;
  for (int j = low; j < high; j++) {
    if (points[j][1] <= pivot) {
      i++;
      swap(points[i], points[j]);
    }
  }

  swap(points[i + 1], points[high]);

  return i + 1;
}

void quickSort(int** points, int low, int high) {
  if (low < high) {
    int pi = partition(points, low, high);

    quickSort(points, low, pi - 1);
    quickSort(points, pi + 1, high);
  }
}

int findMinArrowShots(int** points, int pointsSize, int* pointsColSize) {
  *pointsColSize = 2;

  quickSort(points, 0, pointsSize - 1);
  int result = 1;
  int i = 0;
  int j = 1;
  while (j < pointsSize) {
    if (points[i][1] < points[j][0]) {
      result++;
      i = j;
    }
    j++;
  }

  return result;
}

int main() {
  printf("Hello, World!\n");
  return 0;
}
