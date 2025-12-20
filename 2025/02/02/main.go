package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Range struct {
	start int
	end   int
}

func main() {
	var ranges []Range

	scanner := bufio.NewScanner(os.Stdin)

	scanner.Scan()

	input := scanner.Text()

	for substring := range strings.SplitSeq(input, ",") {
		parts := strings.Split(substring, "-")

		start, err := strconv.Atoi(parts[0])
		if err != nil {
			return
		}

		end, err := strconv.Atoi(parts[1])
		if err != nil {
			return
		}

		ranges = append(ranges, Range{start, end})
	}

	invalidSum := 0

	for _, subrange := range ranges {
		for id := subrange.start; id <= subrange.end; id++ {
			if isIDInvalid(id) {
				invalidSum += id
			}
		}
	}

	fmt.Println("Sum", invalidSum)
}

func isIDInvalid(id int) bool {
	idStr := strconv.Itoa(id)

	for prefixLength := 1; prefixLength <= len(idStr)/2; prefixLength++ {
		if len(idStr)%prefixLength != 0 {
			continue
		}

		pattern := idStr[:prefixLength]

		if strings.Repeat(pattern, len(idStr)/prefixLength) == idStr {
			return true
		}
	}

	return false
}
