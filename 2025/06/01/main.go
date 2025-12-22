package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type operation int

const (
	addition operation = iota
	multiplication
)

func main() {
	operands, operators := getInput()
	var results []int

	for col := range len(operators) {
		switch operators[col] {
		case addition:
			result := 0

			for row := range len(operands) {
				result += operands[row][col]
			}

			results = append(results, result)

		case multiplication:
			result := 1

			for row := range len(operands) {
				result *= operands[row][col]
			}

			results = append(results, result)
		}
	}

	grandTotal := 0

	for _, result := range results {
		grandTotal += result
	}

	fmt.Println("grand total:", grandTotal)
}

func getInput() ([][]int, []operation) {
	var operands [][]int
	var operators []operation

	scanner := bufio.NewScanner(os.Stdin)

	for scanner.Scan() {
		text := scanner.Text()

		if text == "" {
			break
		}

		var operandRow []int

		for part := range strings.SplitSeq(text, " ") {
			trimmed := strings.TrimSpace(part)

			operand, err := strconv.Atoi(trimmed)
			if err != nil {
				switch trimmed {
				case "*":
					operators = append(operators, multiplication)

				case "+":
					operators = append(operators, addition)
				}

				continue
			}

			operandRow = append(operandRow, operand)
		}

		if len(operandRow) > 0 {
			operands = append(operands, operandRow)
		}
	}

	return operands, operators
}
