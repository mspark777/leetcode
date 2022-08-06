package main

import (
	"fmt"
	"math"
)

func poorPigs(buckets int, minutesToDie int, minutesToTest int) int {
	fb := float64(buckets)
	fd := float64(minutesToDie)
	ft := float64(minutesToTest)
	result := math.Ceil(math.Log(fb) / math.Log(ft/fd+1))
	return int(result)
}

type input struct {
	buckets       int
	minutesToDie  int
	minutesToTest int
}

func main() {
	inputs := []*input{
		{
			buckets:       1000,
			minutesToDie:  15,
			minutesToTest: 60,
		},
		{
			buckets:       4,
			minutesToDie:  15,
			minutesToTest: 15,
		},
		{
			buckets:       4,
			minutesToDie:  15,
			minutesToTest: 30,
		},
	}

	for _, input := range inputs {
		buckets := input.buckets
		minutesToDie := input.minutesToDie
		minutesToTest := input.minutesToTest
		result := poorPigs(buckets, minutesToDie, minutesToTest)
		fmt.Println(result)
	}
}
