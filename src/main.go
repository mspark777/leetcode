package main

import "fmt"

type MyQueue struct {
	current []int
	buffer  []int
}

func Constructor() MyQueue {
	return MyQueue{
		current: []int{},
		buffer:  []int{},
	}
}

func (this *MyQueue) Push(x int) {
	this.buffer = append(this.buffer, x)
}

func (this *MyQueue) Pop() int {
	this.fillFromBuffer()

	current := this.current
	top := len(current) - 1
	i := current[top]
	this.current = current[:top]
	return i
}

func (this *MyQueue) Peek() int {
	this.fillFromBuffer()

	current := this.current
	return current[len(current)-1]
}

func (this *MyQueue) Empty() bool {
	size := len(this.buffer) + len(this.current)
	return size < 1
}

func (this *MyQueue) fillFromBuffer() {
	current := this.current
	buffer := this.buffer
	if len(current) < 1 {
		for i := len(buffer) - 1; i >= 0; i -= 1 {
			current = append(current, buffer[i])
		}
		this.current = current
		this.buffer = []int{}
	}
}

func main() {
	queue := Constructor()
	queue.Push(1)
	queue.Push(2)
	fmt.Println(queue.Peek())
	fmt.Println(queue.Pop())
	fmt.Println(queue.Empty())
}
