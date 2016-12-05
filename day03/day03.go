package main

import (
	"bufio"
	"log"
	"os"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

type counter map[[2]int]int

func main() {
	f, err := os.Open("input.txt")
	check(err)
	defer f.Close()
	scanner := bufio.NewScanner(f)
	scanner.Scan()
	input := scanner.Text()
	currentPosition := [...]int{0, 0}
	houseVisitFrequencyMap := make(counter)
	houseVisitFrequencyMap[currentPosition]++
	for _, c := range input {
		if c == '^' {
			currentPosition[1]++
		} else if c == 'v' {
			currentPosition[1]--
		} else if c == '>' {
			currentPosition[0]++
		} else if c == '<' {
			currentPosition[0]--
		} else {
			log.Fatal("Unrecognized character: " + string(c))
		}
		houseVisitFrequencyMap[currentPosition]++
	}
	log.Printf("Answer #1=%d\n", len(houseVisitFrequencyMap))
}
