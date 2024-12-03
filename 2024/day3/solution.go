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
