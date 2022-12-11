package core

import (
	"os"
	"strconv"
	"strings"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func ReadFile(path string) string {
	dat, err := os.ReadFile(path)
	check(err)
	return string(dat)
}

func ToCmdList(data string) (int, CmdList) {
	cmds := CmdList{Cmds: make([]Cmd, 0)}
	var size int
	for i, line := range strings.Split(data, "\n") {
		if i == 0 {
			size, _ = strconv.Atoi(line)
		} else {
			cmds.Add(ToCmd(line))
		}
	}
	return size, cmds
}

func ToCmd(data string) Cmd {
	chars := strings.Split(data, ";")
	char := chars[0]
	if char == "A" {
		id, _ := strconv.Atoi(chars[1])
		size, _ := strconv.Atoi(chars[2])
		return NewAlloc(Id(id), Size(size))
	} else if char == "D" {
		id, _ := strconv.Atoi(chars[1])
		return NewDealloc(Id(id))
	} else if char == "C" {
		return NewCompact()
	} else {
		return NewOutput()
	}
}
