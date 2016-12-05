package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func surfaceAreaFromString(s string) int {
	dimensions := strings.Split(s, "x")
	l, w, h := parseInt(dimensions[0]), parseInt(dimensions[1]), parseInt(dimensions[2])
	area := surfaceArea(l, w, h)
	return area
}

func surfaceArea(l int, w int, h int) int {
	areas := []int{l * w, w * h, h * l}
	sort.Ints(areas)
	return 2*areas[0] + 2*areas[1] + 2*areas[2] + areas[0]
}

func ribbonLengthFromString(s string) int {
	dimensions := strings.Split(s, "x")
	l, w, h := parseInt(dimensions[0]), parseInt(dimensions[1]), parseInt(dimensions[2])
	rLength := ribbonLength(l, w, h)
	return rLength
}

func ribbonLength(l int, w int, h int) int {
	sides := []int{l, w, h}
	sort.Ints(sides)
	r := 2*sides[0] + 2*sides[1]
	ribbonBowLength := l * w * h
	return r + ribbonBowLength
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func parseInt(s string) int {
	i, e := strconv.Atoi(s)
	check(e)
	return i
}

func main() {
	f, err := os.Open("input.txt")
	check(err)
	defer f.Close()
	scanner := bufio.NewScanner(f)
	sum := 0
	ribbonLengthSum := 0
	for scanner.Scan() {
		s := scanner.Text()
		sum += surfaceAreaFromString(s)
		ribbonLengthSum += ribbonLengthFromString(s)
	}
	fmt.Printf("Answer #1=%d\n", sum)
	fmt.Printf("Answer #2=%d\n", ribbonLengthSum)
}
