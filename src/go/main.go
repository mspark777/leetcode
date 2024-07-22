package main

import (
	"fmt"
	"sort"
)

type data struct {
	name   string
	height int
}

func sortPeople(names []string, heights []int) []string {
	dataList := make([]data, 0, len(names))
	for i, name := range names {
		dataList = append(dataList, data{name, heights[i]})
	}

	sort.Slice(dataList, func(l, r int) bool {
		return dataList[r].height < dataList[l].height
	})

	result := make([]string, 0, len(names))
	for _, data := range dataList {
		result = append(result, data.name)
	}

	return result
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
