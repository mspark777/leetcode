#include "./lib.h"
#include <stdio.h>

#define ListNode list_node

struct ListNode *detectCycle(struct ListNode *head) {
  struct ListNode *fast = head;
  struct ListNode *slow = head;

  while ((fast != NULL) && (fast->next != NULL)) {
    fast = fast->next->next;
    slow = slow->next;

    if (fast == slow) {
      break;
    }
  }

  if ((fast == NULL) || (fast->next == NULL)) {
    return NULL;
  }

  fast = head;
  while (fast != slow) {
    fast = fast->next;
    slow = slow->next;
  }

  return fast;
}

int problem0(void) {
  int vals[] = {3, 2, 0, -4};
  int val_count = sizeof(vals) / sizeof(vals[0]);
  int pos = 1;

  struct list_node *head = new_cycle_list(vals, val_count, pos);
  struct list_node *result = detectCycle(head);

  int val = result != NULL ? result->val : -1;
  free_list_node(head, result);

  return val;
}

int problem1(void) {
  int vals[] = {1, 2};
  int val_count = sizeof(vals) / sizeof(vals[0]);
  int pos = 0;

  struct list_node *head = new_cycle_list(vals, val_count, pos);
  struct list_node *result = detectCycle(head);

  int val = result != NULL ? result->val : -1;
  free_list_node(head, result);

  return val;
}

int problem2(void) {
  int vals[] = {1};
  int val_count = sizeof(vals) / sizeof(vals[0]);
  int pos = -1;

  struct list_node *head = new_cycle_list(vals, val_count, pos);
  struct list_node *result = detectCycle(head);

  int val = result != NULL ? result->val : -1;
  free_list_node(head, result);

  return val;
}

typedef int (*problem_t)(void);
int main() {
  const problem_t problems[] = {problem0, problem1, problem2};

  for (unsigned long i = 0; i < sizeof(problems) / sizeof(problems[0]);
       i += 1) {
    const int result = problems[i]();
    printf("%d\n", result);
  }
  return 0;
}
