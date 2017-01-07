package main

import "testing"
import "github.com/stretchr/testify/assert"

func TestIncrementChar(t *testing.T) {
	assert.Equal(t, 'a', incrementChar('z'))
	assert.Equal(t, 'b', incrementChar('a'))
}

func TestHasBadCharacters(t *testing.T) {
	assert.True(t, hasBadCharacters("foo"))
	assert.False(t, hasBadCharacters("abcd"))
	assert.True(t, hasBadCharacters("lucas"))
	assert.True(t, hasBadCharacters("ice"))
}

func TestHasCharacterStraight(t *testing.T) {
	assert.True(t, hasCharacterStraight("abc"))
	assert.True(t, hasCharacterStraight("abcd"))
	assert.False(t, hasCharacterStraight("lucas"))
	assert.False(t, hasCharacterStraight("ice"))
	assert.True(t, hasCharacterStraight("aasdfasbcxyz"))
	assert.False(t, hasCharacterStraight("aasdfasbcxrrryz"))
}

func TestHasTwoCharacterPairs(t *testing.T) {
	assert.False(t, hasTwoCharacterPairs("abc"))
	assert.False(t, hasTwoCharacterPairs("abcd"))
	assert.False(t, hasTwoCharacterPairs("lucas"))
	assert.False(t, hasTwoCharacterPairs("ice"))
	assert.True(t, hasTwoCharacterPairs("aabb"))
	assert.True(t, hasTwoCharacterPairs("aaabb"))
	assert.True(t, hasTwoCharacterPairs("aacbb"))
	assert.False(t, hasTwoCharacterPairs("aaa"))
	assert.False(t, hasTwoCharacterPairs("aaaba"))
	assert.True(t, hasTwoCharacterPairs("aasdfasbcxyzz"))
	assert.True(t, hasTwoCharacterPairs("aasdfasbcxrrryzz"))
}

func TestIsValidPassword(t *testing.T) {
	assert.False(t, isValidPassword("hijklmmn"))
	assert.False(t, isValidPassword("abbceffg"))
	assert.False(t, isValidPassword("abbcegjk"))
	assert.False(t, isValidPassword("abcdefgh"))
	assert.True(t, isValidPassword("abcdffaa"))
	assert.False(t, isValidPassword("ghijklmn"))
	assert.True(t, isValidPassword("ghjaabcc"))
}

func TestIncrementPassword(t *testing.T) {
	assert.Equal(t, "xy", incrementPassword("xx"))
	assert.Equal(t, "xz", incrementPassword("xy"))
	assert.Equal(t, "ya", incrementPassword("xz"))
	assert.Equal(t, "yb", incrementPassword("ya"))
}

func TestNextValidPassword(t *testing.T) {
	assert.Equal(t, "abcdffaa", nextValidPassword("abcdefgh"))
	assert.Equal(t, "ghjaabcc", nextValidPassword("ghijklmn"))
}
