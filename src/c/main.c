#include "./lib.h"
#include <stdio.h>
#include <stdlib.h>

#define ListNode list_node

typedef struct {
  struct ListNode *head;
} Solution;

Solution *solutionCreate(struct ListNode *head) {
  Solution *solution = malloc(sizeof(Solution));
  solution->head = head;

  return solution;
}

int solutionGetRandom(Solution *obj) {
  int scope = 1;
  int result = 0;

  for (struct ListNode *curr = obj->head; curr != NULL; curr = curr->next) {
    if ((rand() % scope) == 0) {
      result = curr->val;
    }
    scope += 1;
  }

  return result;
}

void solutionFree(Solution *obj) { free(obj); }

void problem0(void) {
  const int vals[] = {1, 2, 3};
  int val_count = sizeof(vals) / sizeof(vals[0]);

  Solution *solution = solutionCreate(new_list(vals, val_count));
  printf("%d\n", solutionGetRandom(solution));
  printf("%d\n", solutionGetRandom(solution));
  printf("%d\n", solutionGetRandom(solution));
  printf("%d\n", solutionGetRandom(solution));
  printf("%d\n", solutionGetRandom(solution));

  free_list_node(solution->head, NULL);
  solutionFree(solution);
}

typedef void (*problem_t)(void);
int main() {
  const problem_t problems[] = {problem0};

  for (unsigned long i = 0; i < sizeof(problems) / sizeof(problems[0]);
       i += 1) {
    problems[i]();
  }
  return 0;
}
