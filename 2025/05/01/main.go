package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Window struct {
	start int
	end   int
}

func main() {
	windows, ids := getInput()

	freshCount := 0

	for _, id := range ids {
		for _, window := range windows {
			if id >= window.start && id <= window.end {
				freshCount++
				break
			}
		}
	}

	fmt.Println("fresh count:", freshCount)
}

func getInput() ([]Window, []int) {
	var windows []Window
	var ids []int

	scanner := bufio.NewScanner(os.Stdin)

	expectsWindows := true

	for scanner.Scan() {
		text := scanner.Text()

		if expectsWindows {
			if text == "" {
				expectsWindows = false
				continue
			}

			window := strings.Split(text, "-")

			start, err := strconv.Atoi(window[0])
			if err != nil {
				return make([]Window, 0), make([]int, 0)
			}

			end, err := strconv.Atoi(window[1])
			if err != nil {
				return make([]Window, 0), make([]int, 0)
			}

			windows = append(windows, Window{start, end})

		} else {
			if text == "" {
				break
			}

			id, err := strconv.Atoi(text)
			if err != nil {
				return make([]Window, 0), make([]int, 0)
			}

			ids = append(ids, id)
		}
	}

	return windows, ids
}
