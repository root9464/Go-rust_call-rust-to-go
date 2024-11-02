package main

import (
	"fmt"
	"os/exec"
)

func main() {
	output, err := exec.Command("../rust/target/release/rust-code", "--command", "ping").Output()
	if err != nil {
		fmt.Println(err.Error())
	}
	fmt.Println(string(output)) // will print "Hello, John!"
}
