package main

import (
	"crypto/md5"
	"encoding/hex"
	"io"
	"log"
	"strconv"
	"strings"
)

func main() {
	input := "ckczppom"
	for i := 0; ; i++ {
		s := input + strconv.Itoa(i)
		h := md5.New()
		io.WriteString(h, s)
		hashString := hex.EncodeToString(h.Sum(nil))
		if strings.HasPrefix(hashString, "00000") {
			log.Println(i)
			break
		}
	}
}
