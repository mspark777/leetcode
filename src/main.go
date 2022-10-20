package main

import (
	"fmt"
)

func intToRoman(num int) string {
	M := []string{"", "M", "MM", "MMM"}
	C := []string{"", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"}
	X := []string{"", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"}
	I := []string{"", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"}

	mi := num / 1000
	ci := (num % 1000) / 100
	xi := (num % 100) / 10
	ii := num % 10

	return fmt.Sprint(M[mi], C[ci], X[xi], I[ii])
}

func main() {
	inputs := []int{3, 58, 1994}

	for _, num := range inputs {
		result := intToRoman(num)
		fmt.Println(result)
	}
}
