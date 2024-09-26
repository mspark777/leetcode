package main

import (
	"fmt"
)

type event struct {
	start int
	end   int
}

func min(l, r int) int {
	if l < r {
		return l
	}

	return r
}

func max(l, r int) int {
	if l > r {
		return l
	}

	return r
}

type MyCalendar struct {
	events []event
}

func Constructor() MyCalendar {
	return MyCalendar{events: []event{}}
}

func (this *MyCalendar) Book(start int, end int) bool {
	for _, event := range this.events {
		l := max(event.start, start)
		r := min(event.end, end)
		if l < r {
			return false
		}
	}

	this.events = append(this.events, event{start, end})
	return true
}

type input struct {
	names   []string
	heights []int
}

func main() {
	inputs := []input{
		{
			[]string{"Mary", "John", "Emma"},
			[]int{180, 165, 170},
		},
		{
			[]string{"Alice", "Bob", "Bob"},
			[]int{155, 185, 150},
		},
		{
			[]string{"IEO", "Sgizfdfrims", "QTASHKQ", "Vk", "RPJOFYZUBFSIYp", "EPCFFt", "VOYGWWNCf", "WSpmqvb"},
			[]int{17233, 32521, 14087, 42738, 46669, 65662, 43204, 8224},
		},
	}

	for _, input := range inputs {
		result := sortPeople(input.names, input.heights)
		fmt.Println(result)
	}
}
