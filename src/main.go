package main

import "fmt"

type MyStack struct {
	queue []int
}

func Constructor() MyStack {
	return MyStack{queue: []int{}}
}

func (this *MyStack) Push(x int) {
	this.queue = append(this.queue, x)
	size := len(this.queue)
	for i := 1; i < size; i += 1 {
		top, queue := this.queue[0], this.queue[1:]
		this.queue = append(queue, top)
	}
}

func (this *MyStack) Pop() int {
	top, queue := this.queue[0], this.queue[1:]
	this.queue = queue

	return top
}

func (this *MyStack) Top() int {
	return this.queue[0]
}

func (this *MyStack) Empty() bool {
	return len(this.queue) < 1
}

func main() {
	myStack := Constructor()
	myStack.Push(1)
	myStack.Push(2)
	fmt.Println(myStack.Top())
	fmt.Println(myStack.Pop())
	fmt.Println(myStack.Empty())
}
