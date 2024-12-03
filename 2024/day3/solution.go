package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func main() {
	content, err := os.ReadFile("input.txt")
	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	// Sanitize the content until no more "don't()" or "do()" are found
	for true {
		result := sanitizeOnce(content)
		if result == nil {
			break
		}
		content = result
	}

	re := regexp.MustCompile(`mul\((\d{0,3}),(\d{0,3})\)`)
	matches := re.FindAllStringSubmatch(string(content), -1)

	total := 0

	for _, match := range matches {
		m1, _ := strconv.Atoi(match[1])
		m2, _ := strconv.Atoi(match[2])
		total += m1 * m2
	}

	fmt.Println(total)
}

func sanitizeOnce(content []byte) []byte {
	re := regexp.MustCompile(`don't\(\)`)
	// Index of first "don't()"
	dontIndex := re.FindIndex(content)

	// If no match, return nil
	if dontIndex == nil {
		return nil
	}

	re = regexp.MustCompile(`do\(\)`)
	// Relative index of next "do()"
	doIndex := re.FindIndex(content[dontIndex[1]:])

	// If no match, remove the rest of the content
	if doIndex == nil {
		return content[dontIndex[1]:]
	}

	startIndex := dontIndex[0]
	endIndex := dontIndex[1] + doIndex[1]

	// Remove the content between the two
	return append(content[:startIndex], content[endIndex:]...)
}
