#include "./main.h"

typedef struct {
	int *queue;
	int front;
	int rear;
	int size;
	int capacity;
} MyCircularDeque;

MyCircularDeque *myCircularDequeCreate(int k)
{
	MyCircularDeque *deque = malloc(sizeof(MyCircularDeque));
	deque->queue = malloc(sizeof(int) * k);
	deque->front = 0;
	deque->rear = k - 1;
	deque->size = 0;
	deque->capacity = k;

	return deque;
}

void myCircularDequeFree(MyCircularDeque *obj)
{
	free(obj->queue);
	free(obj);
}

bool myCircularDequeInsertFront(MyCircularDeque *obj, int value);
bool myCircularDequeInsertLast(MyCircularDeque *obj, int value);
bool myCircularDequeDeleteFront(MyCircularDeque *obj);
bool myCircularDequeDeleteLast(MyCircularDeque *obj);
int myCircularDequeGetFront(MyCircularDeque *obj);
int myCircularDequeGetRear(MyCircularDeque *obj);
bool myCircularDequeIsEmpty(MyCircularDeque *obj);
bool myCircularDequeIsFull(MyCircularDeque *obj);

bool myCircularDequeInsertFront(MyCircularDeque *obj, int value)
{
	if (myCircularDequeIsFull(obj)) {
		return 0;
	}

	obj->front = (obj->front - 1 + obj->capacity) % obj->capacity;
	obj->queue[obj->front] = value;
	obj->size += 1;
	return 1;
}

bool myCircularDequeInsertLast(MyCircularDeque *obj, int value)
{
	if (myCircularDequeIsFull(obj)) {
		return 0;
	}

	obj->rear = (obj->rear + 1) % obj->capacity;
	obj->queue[obj->rear] = value;
	obj->size += 1;
	return 1;
}

bool myCircularDequeDeleteFront(MyCircularDeque *obj)
{
	if (myCircularDequeIsEmpty(obj)) {
		return 0;
	}

	obj->front = (obj->front + 1) % obj->capacity;
	obj->size -= 1;
	return 1;
}

bool myCircularDequeDeleteLast(MyCircularDeque *obj)
{
	if (myCircularDequeIsEmpty(obj)) {
		return 0;
	}

	obj->rear = (obj->rear - 1 + obj->capacity) % obj->capacity;
	obj->size -= 1;
	return 1;
}

int myCircularDequeGetFront(MyCircularDeque *obj)
{
	return myCircularDequeIsEmpty(obj) ? -1 : obj->queue[obj->front];
}

int myCircularDequeGetRear(MyCircularDeque *obj)
{
	return myCircularDequeIsEmpty(obj) ? -1 : obj->queue[obj->rear];
}

bool myCircularDequeIsEmpty(MyCircularDeque *obj)
{
	return obj->size < 1;
}

bool myCircularDequeIsFull(MyCircularDeque *obj)
{
	return obj->size >= obj->capacity;
}
