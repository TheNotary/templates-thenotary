package api

import "fmt"

type SomeStruct struct {
	Msg    string `json:"msg"`
	Random string `json:"random"`
}

func (h SomeStruct) Hello(otherString string) (string, error) {
	return fmt.Sprintf("Hello, %s %s", h.Msg, otherString), nil
}

func SayHello(msg string) (output string, err error) {
	if msg == "banned" {
		err = fmt.Errorf("error in SayHello: ")
		return
	}
	var pointerToSomeStruct = &SomeStruct{}
	pointerToSomeStruct.Msg = msg
	output, err = pointerToSomeStruct.Hello("other message")
	return
}
