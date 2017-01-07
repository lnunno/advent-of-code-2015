package main

import (
	"fmt"
	"log"
	"strings"
)

func incrementChar(char rune) rune {
	if char >= 'a' && char <= 'z' {
		if char == 'z' {
			char = 'a'
		} else {
			char++
		}
	} else {
		panic(fmt.Sprintf("Bad character '%c' must be between [a-z]", char))
	}
	return char
}

func incrementPassword(password string) string {
	passwordBytes := []rune(password)
	for i := len(passwordBytes) - 1; i >= 0; i-- {
		passwordBytes[i] = incrementChar(passwordBytes[i])
		if passwordBytes[i] == 'a' {
			// Means that this "digit" wrapped around.
			continue
		} else {
			break // Done incrementing
		}
	}
	return string(passwordBytes)
}

// Checks for the characters ['i','o','l'] which are not allowed in passwords.
func hasBadCharacters(password string) bool {
	return strings.ContainsAny(password, "iol")
}

// Returns true if 3 or more consecutive characters in the string are immediate successors (e.g. "abc" but not "abd")
func hasCharacterStraight(password string) bool {
	for i := 0; i <= len(password)-3; i++ {
		substr := password[i : i+3]
		if substr[0] == substr[1]-1 && substr[1] == substr[2]-1 {
			return true
		}
	}
	return false
}

// Returns true if there are two character pairs in the string like "aa", "bb", etc.
func hasTwoCharacterPairs(password string) bool {
	numPairs := 0
	for i := 0; i <= len(password)-2; i++ {
		substr := password[i : i+2]
		if substr[0] == substr[1] {
			numPairs++
			i++ // So we don't count an overlapping pair.
			if numPairs == 2 {
				return true
			}
		}
	}
	return false
}

func isValidPassword(password string) bool {
	return !hasBadCharacters(password) && hasCharacterStraight(password) && hasTwoCharacterPairs(password)
}

func nextValidPassword(password string) string {
	for {
		password = incrementPassword(password)
		if isValidPassword(password) {
			return password
		}
	}
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func main() {
	newPass := nextValidPassword("cqjxjnds")
	log.Printf("Part #1=%s", newPass)
	log.Printf("Part #2=%s", nextValidPassword(newPass))
}
