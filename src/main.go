package main

import "fmt"

type MyCircularQueue struct {
	queue []int
	begin int
	end   int
	size  int
}

func Constructor(k int) MyCircularQueue {
	return MyCircularQueue{
		queue: make([]int, k),
		begin: 0,
		end:   0,
		size:  0,
	}
}

func (this *MyCircularQueue) EnQueue(value int) bool {
	if this.IsFull() {
		return false
	}

	end := this.end
	this.queue[end] = value
	this.end = this.nextIndex(end)
	this.size += 1

	return true
}

func (this *MyCircularQueue) DeQueue() bool {
	if this.IsEmpty() {
		return false
	}

	this.begin = this.nextIndex(this.begin)
	this.size -= 1

	return true
}

func (this *MyCircularQueue) Front() int {
	if this.IsEmpty() {
		return -1
	} else {
		return this.queue[this.begin]
	}
}

func (this *MyCircularQueue) Rear() int {
	if this.IsEmpty() {
		return -1
	}

	end := this.end
	queue := this.queue
	tail := end - 1
	if tail < 0 {
		tail = len(queue) - 1
	}

	return queue[tail]
}

func (this *MyCircularQueue) IsEmpty() bool {
	return this.size < 1
}

func (this *MyCircularQueue) IsFull() bool {
	return this.size >= len(this.queue)
}

func (this *MyCircularQueue) nextIndex(cur int) int {
	return (cur + 1) % len(this.queue)
}

func main() {
	queue := Constructor(3)
	fmt.Println(queue.EnQueue(1))
	fmt.Println(queue.EnQueue(2))
	fmt.Println(queue.EnQueue(3))
	fmt.Println(queue.EnQueue(4))
	fmt.Println(queue.Rear())
	fmt.Println(queue.IsFull())
	fmt.Println(queue.DeQueue())
	fmt.Println(queue.EnQueue(4))
	fmt.Println(queue.Rear())
}
