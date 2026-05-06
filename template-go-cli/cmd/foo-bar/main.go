package main

import (
	"fmt"

	"FOO_GIT_REPO_PATH/pkg/api"
)

func main() {
	myVar, err := api.SayHello("pew pew")
	if err != nil {
		fmt.Println("Error: " + err.Error())
	}
	fmt.Println(myVar)
}
