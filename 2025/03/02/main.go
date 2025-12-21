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
	var batteries [12]int

	for i := 0; i < 12; i++ {
		batteries[i] = len(bank) - 12 + i
	}

	// first battery
	for i := batteries[0] - 1; i >= 0; i-- {
		if bank[i] >= bank[batteries[0]] {
			batteries[0] = i
		}
	}

	// remaining batteries
	for i := 1; i < 12; i++ {
		for j := batteries[i] - 1; j > batteries[i-1]; j-- {
			if bank[j] >= bank[batteries[i]] {
				batteries[i] = j
			}
		}
	}

	joltage := 0

	for i, multiplier := 11, 1; i >= 0; i, multiplier = i-1, multiplier*10 {
		joltage += bank[batteries[i]] * multiplier
	}

	return joltage
}
