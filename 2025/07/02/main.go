package main

import (
	"bufio"
	"fmt"
	"os"
)

type block int

const (
	emptySpace block = iota
	splitter
	start
)

type manifold [][]block

func main() {
	manifold := getInput()
	startCol := findStart(&manifold)

	timelines := make([]int, len(manifold[0]))
	timelines[startCol] = 1

	for _, row := range manifold {
		for col, block := range row {
			if block != splitter {
				continue
			}

			if col > 0 {
				timelines[col-1] += timelines[col]
			}

			if col < len(manifold[0])-1 {
				timelines[col+1] += timelines[col]
			}

			timelines[col] = 0
		}
	}

	totalTimelines := 0

	for _, timelines := range timelines {
		totalTimelines += timelines
	}

	fmt.Println("timelines:", totalTimelines)
}

func findStart(manifold *manifold) int {
	for col := range (*manifold)[0] {
		if (*manifold)[0][col] == start {
			return col
		}
	}

	panic("unreachable")
}

func getInput() (manifold manifold) {
	scanner := bufio.NewScanner(os.Stdin)

	for scanner.Scan() {
		text := scanner.Text()

		if text == "" {
			break
		}

		var line []block

		for _, rune := range text {
			switch rune {
			case 'S':
				line = append(line, start)

			case '.':
				line = append(line, emptySpace)

			case '^':
				line = append(line, splitter)
			}
		}

		manifold = append(manifold, line)
	}

	return
}
