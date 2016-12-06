package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func isNiceString(input string) bool {
	vowels := "aeiou"
	badStrings := []string{"ab", "cd", "pq", "xy"}
	for _, bs := range badStrings {
		if strings.Count(input, bs) >= 1 {
			return false
		}
	}
	vowelCount := 0
	for _, v := range vowels {
		vowelCount += strings.Count(input, string(v))
	}
	if vowelCount < 3 {
		return false
	}
	hasDoubleLetter := false
	for i, letter := range input {
		if i == len(input)-1 {
			break
		}
		if letter == rune(input[i+1]) {
			hasDoubleLetter = true
		}
	}
	if !hasDoubleLetter {
		return false
	}
	return true
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func main() {
	f, err := os.Open("input.txt")
	check(err)
	defer f.Close()
	scanner := bufio.NewScanner(f)
	numNiceStrings := 0
	for scanner.Scan() {
		s := scanner.Text()
		if isNiceString(s) {
			numNiceStrings++
		}
	}
	fmt.Println(numNiceStrings)

}
