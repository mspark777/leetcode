package main

import "fmt"
import "strconv"

func offset(year int) []int {
	if ((year%4 == 0) && (year%100 != 0)) || (year%400 == 0) {
		return []int{0, 31, 60, 91, 121, 152,
			182, 213, 244, 274, 305, 335}
	}

	return []int{0, 31, 59, 90, 120, 151,
		181, 212, 243, 273, 304, 334}
}

func dayOfYear(date string) int {
	year, _ := strconv.Atoi(date[0:4])
	month, _ := strconv.Atoi(date[5:7])
	day, _ := strconv.Atoi(date[8:])
	return offset(year)[month-1] + day
}

type input struct {
	date string
}

func main() {
	inputs := []input{
		{
			date: "2019-01-09",
		},
		{
			date: "2019-02-10",
		},
		{
			date: "2008-10-10",
		},
		{
			date: "2016-02-09",
		},
	}

	for _, input := range inputs {
		result := dayOfYear(input.date)
		fmt.Println(result)
	}
}
