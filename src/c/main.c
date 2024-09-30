#include "./main.h"

typedef struct {
	int *stack;
	int *inc;
	int top;
	int capacity;
} CustomStack;

CustomStack *customStackCreate(int max_size)
{
	CustomStack *stack = malloc(sizeof(CustomStack));
	stack->top = -1;
	stack->inc = calloc(max_size, sizeof(int));
	stack->stack = malloc(sizeof(int) * max_size);
	stack->capacity = max_size;

	return stack;
}

void customStackFree(CustomStack *obj)
{
	free(obj->stack);
	free(obj->inc);
	free(obj);
}

void customStackPush(CustomStack *obj, int x);
int customStackPop(CustomStack *obj);
void customStackIncrement(CustomStack *obj, int k, int val);

void customStackPush(CustomStack *obj, int x)
{
	const int top = obj->top + 1;
	if (top >= obj->capacity) {
		return;
	}

	obj->top = top;
	obj->stack[top] = x;
}

int customStackPop(CustomStack *obj)
{
	if (obj->top < 0) {
		return -1;
	}

	const int top = obj->top;
	const int result = obj->stack[top] + obj->inc[top];
	if (top > 0) {
		obj->inc[top - 1] += obj->inc[top];
	}

	obj->inc[top] = 0;
	obj->top = top - 1;
	return result;
}

void customStackIncrement(CustomStack *obj, int k, int val)
{
	const int top = obj->top;
	if (top < 0) {
		return;
	}

	int idx = k - 1;
	if (idx > top) {
		idx = top;
	}

	obj->inc[idx] += val;
}
