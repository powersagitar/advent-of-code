package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type worksheet [][]rune

type operation int

const (
	addition operation = iota
	multiplication
)

const rows int = 4

func main() {
	worksheet, operations := getInput()

	var results []int

	result := resetResult(operations[0])
	operation := 0

	for col := range len(worksheet[0]) {
		if isDividerCol(worksheet, col) {
			results = append(results, result)
			operation++
			result = resetResult(operations[operation])

			continue
		}

		var operand []rune

		for row := range rows {
			operand = append(operand, worksheet[row][col])
		}

		operandInt, err := strconv.Atoi(strings.TrimSpace(string(operand)))
		if err != nil {
			panic("unable to parse operand")
		}

		switch operations[operation] {
		case addition:
			result += operandInt

		case multiplication:
			result *= operandInt
		}
	}

	grandTotal := result

	for _, result := range results {
		grandTotal += result
	}

	fmt.Println("grand total:", grandTotal)
}

func resetResult(operation operation) int {
	switch operation {
	case addition:
		return 0

	case multiplication:
		return 1
	}

	panic("unreachable")
}

func isDividerCol(worksheet worksheet, col int) bool {
	for row := range rows {
		if worksheet[row][col] != ' ' {
			return false
		}
	}

	return true
}

func getInput() (worksheet worksheet, operations []operation) {
	scanner := bufio.NewScanner(os.Stdin)

	worksheet = getWorksheet(scanner)
	operations = getOperations(scanner)

	return
}

func getWorksheet(scanner *bufio.Scanner) (worksheet worksheet) {
	for range rows {
		scanner.Scan()

		var line []rune

		for _, rune := range scanner.Text() {
			line = append(line, rune)
		}

		worksheet = append(worksheet, line)
	}

	return
}

func getOperations(scanner *bufio.Scanner) (operations []operation) {
	scanner.Scan()

	for operation := range strings.SplitSeq(scanner.Text(), " ") {
		switch strings.TrimSpace(operation) {
		case "+":
			operations = append(operations, addition)

		case "*":
			operations = append(operations, multiplication)
		}
	}

	return
}
