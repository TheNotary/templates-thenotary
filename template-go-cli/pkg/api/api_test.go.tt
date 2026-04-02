package api

import (
	"regexp"
	"testing"
)

func TestSomeStructSaysHello(t *testing.T) {
	otherMsg := "other message"
	want := regexp.MustCompile(`\b` + otherMsg + `\b`)

	myStruct := &SomeStruct{}
	myStruct.Msg = "this is the msg"

	output, err := myStruct.Hello(otherMsg)

	// Ok, this is such horrible testing, conser
	// Ginkgo asap https://onsi.github.io/ginkgo/
	if !want.MatchString(output) || err != nil {
		t.Errorf("myStruct#Hello returned an error")
		t.Errorf(`Hello("other message") = %q, %v, want match for %#q, nil`, output, err, want)
	}
}

func TestSomeStructBannsTheWordBanned(t *testing.T) {
	msg := "banned"
	want := regexp.MustCompile(`\b` + msg + `\b`)

	output, err := SayHello(msg)

	// Ok, this is such horrible testing, conser
	// Ginkgo asap https://onsi.github.io/ginkgo/
	if output != "" || err == nil {
		t.Errorf("myStruct#Hello didn't return an error")
		t.Errorf(`Hello("banned") = %q, %v, want match for %#q, nil`, output, err, want)
	}
}
