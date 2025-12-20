package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	var banks [][]int

	scanner := bufio.NewScanner(os.Stdin)

	for scanner.Scan() {
		text := scanner.Text()

		if text == "" {
			break
		}

		var bank []int

		for _, rune := range text {
			joltage := int(rune - '0')
			bank = append(bank, joltage)
		}

		banks = append(banks, bank)
	}

	total := 0

	for _, bank := range banks {
		total += findLargestJoltage(bank)
	}

	fmt.Println("Total joltage:", total)
}

func findLargestJoltage(bank []int) int {
	largest := 0

	for first := 0; first < len(bank)-1; first++ {
		for second := first + 1; second < len(bank); second++ {
			joltage := bank[first]*10 + bank[second]
			largest = max(largest, joltage)
		}
	}

	return largest
}
