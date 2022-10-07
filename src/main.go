package main

import "fmt"

type MyCalendarThree struct {
	vals map[int]int
	lazy map[int]int
}

func Constructor() MyCalendarThree {
	return MyCalendarThree{
		vals: map[int]int{},
		lazy: map[int]int{},
	}
}

func (this *MyCalendarThree) Book(start, end int) int {
	this.Update(start, end-1, 0, 1000000000, 1)
	return this.vals[1]
}

func (this *MyCalendarThree) Update(start, end, left, right, idx int) {
	if (start > right) || (end < left) {
		return
	}

	vals := this.vals
	lazy := this.lazy

	if (start <= left) && (right <= end) {
		vals[idx] += 1
		lazy[idx] += 1
	} else {
		mid := (left + right) / 2
		idx2 := idx * 2
		idx21 := idx2 + 1

		this.Update(start, end, left, mid, idx2)
		this.Update(start, end, mid+1, right, idx21)

		val2 := vals[idx2]
		val21 := vals[idx21]

		if val2 > val21 {
			vals[idx] = lazy[idx] + val2
		} else {
			vals[idx] = lazy[idx] + val21
		}
	}
}

func main() {
	obj := Constructor()
	fmt.Println(obj.Book(10, 20))
	fmt.Println(obj.Book(50, 60))
	fmt.Println(obj.Book(10, 40))
	fmt.Println(obj.Book(5, 15))
	fmt.Println(obj.Book(5, 10))
	fmt.Println(obj.Book(25, 55))
}
