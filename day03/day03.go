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

func move(c rune, currentPosition *[2]int, houseVisitFrequencyMap counter) {
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
	houseVisitFrequencyMap[*currentPosition]++
}

func part1(input string) {
	currentPosition := [...]int{0, 0}
	houseVisitFrequencyMap := make(counter)
	houseVisitFrequencyMap[currentPosition]++
	for _, c := range input {
		move(c, &currentPosition, houseVisitFrequencyMap)
	}
	log.Printf("Answer #1=%d\n", len(houseVisitFrequencyMap))
}

func part2(input string) {
	santa := [...]int{0, 0}
	robot := [...]int{0, 0}
	houseVisitFrequencyMap := make(counter)
	houseVisitFrequencyMap[santa]++
	for i, c := range input {
		if (i % 2) == 0 {
			move(c, &santa, houseVisitFrequencyMap)
		} else {
			move(c, &robot, houseVisitFrequencyMap)
		}
	}
	log.Printf("Answer #2=%d\n", len(houseVisitFrequencyMap))
}

func main() {
	f, err := os.Open("input.txt")
	check(err)
	defer f.Close()
	scanner := bufio.NewScanner(f)
	scanner.Scan()
	input := scanner.Text()
	part1(input)
	part2(input)
}
