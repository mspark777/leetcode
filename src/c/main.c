#include <stdio.h>

int hours_required(const int *piles, const int pile_size, const int k) {
  if (k == 0) {
    unsigned int t = 0;
    t = ~t;
    return (int)(t >> 1);
  }

  int hours = 0;
  for (int i = 0; i < pile_size; i += 1) {
    const int pile = piles[i];
    hours += (pile % k) != 0;
    hours += pile / k;
  }

  return hours;
}

int minEatingSpeed(const int *piles, const int pile_size, const int h) {
  long long sum = 0;
  int max_pile = 0;

  for (int i = 0; i < pile_size; i += 1) {
    const int pile = piles[i];
    sum += pile;
    if (pile > max_pile) {
      max_pile = pile;
    }
  }

  int left = (int)(sum / h);
  int right = max_pile;
  while (left < right) {
    const int middle = (left + right) / 2;
    const int required = hours_required(piles, pile_size, middle);
    if (required > h) {
      left = middle + 1;
    } else {
      right = middle;
    }
  }

  return left;
}

int problem0(void) {
  int piles[] = {3, 6, 7, 11};
  const int pile_size = sizeof(piles) / sizeof(piles[0]);
  const int h = 8;
  const int result = minEatingSpeed(piles, pile_size, h);
  return result;
}

int problem1(void) {
  int piles[] = {30, 11, 23, 4, 20};
  const int pile_size = sizeof(piles) / sizeof(piles[0]);
  const int h = 5;
  const int result = minEatingSpeed(piles, pile_size, h);
  return result;
}

int problem2(void) {
  int piles[] = {30, 11, 23, 4, 20};
  const int pile_size = sizeof(piles) / sizeof(piles[0]);
  const int h = 6;
  const int result = minEatingSpeed(piles, pile_size, h);
  return result;
}

int problem3(void) {
  int piles[] = {332484035, 524908576, 855865114, 632922376, 222257295,
                 690155293, 112677673, 679580077, 337406589, 290818316,
                 877337160, 901728858, 679284947, 688210097, 692137887,
                 718203285, 629455728, 941802184};
  const int pile_size = sizeof(piles) / sizeof(piles[0]);
  const int h = 823855818;
  const int result = minEatingSpeed(piles, pile_size, h);
  return result;
}

typedef int (*problem_t)(void);
int main() {
  const problem_t problems[] = {problem0, problem1, problem2, problem3};

  for (unsigned long i = 0; i < sizeof(problems) / sizeof(problems[0]);
       i += 1) {
    const int result = problems[i]();
    printf("%d\n", result);
  }
  return 0;
}
