package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
)

func sumList(inputList []interface{}) float64 {
	total := 0.0
	for _, v := range inputList {
		switch vType := v.(type) {
		case float64:
			total += v.(float64)
		case []interface{}:
			total += sumList(v.([]interface{}))
		case map[string]interface{}:
			total += sumMap(v.(map[string]interface{}))
		case string: // Don't care.
		default:
			panic(fmt.Sprintf("Unexpected type %T with value \"%v\"\n", vType, v))
		}
	}
	return total
}

func sumMap(inputMap map[string]interface{}) float64 {
	total := 0.0
	for k, v := range inputMap {
		if k == "red" {
			continue
		}
		switch vType := v.(type) {
		case float64:
			total += v.(float64)
		case []interface{}:
			total += sumList(v.([]interface{}))
		case map[string]interface{}:
			total += sumMap(v.(map[string]interface{}))
		case string:
			if v.(string) == "red" {
				return 0
			}
		default:
			panic(fmt.Sprintf("Unexpected type %T with value \"%v\"\n", vType, v))

		}
	}
	return total
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func main() {
	file, e := ioutil.ReadFile("input.json")
	check(e)
	var dat map[string]interface{}
	err := json.Unmarshal(file, &dat)
	check(err)
	log.Printf("%v", sumMap(dat))
}
