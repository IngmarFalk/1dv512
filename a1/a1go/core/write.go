package core

import (
	"os"
	"strconv"
)

func WriteFile(path string, data string) {
	err := os.WriteFile(path, []byte(data), 0644)
	check(err)
}

func CmdListToString(cmds CmdList) string {
	var data string
	for _, cmd := range cmds.Cmds {
		data += CmdToString(cmd) + "\n"
	}
	return data
}

func CmdToString(cmd Cmd) string {
	if cmd.Op == Alloc {
		return "A;" + strconv.Itoa(int(cmd.BlockID)) + ";" + strconv.Itoa(int(cmd.Size))
	} else if cmd.Op == Dealloc {
		return "D;" + strconv.Itoa(int(cmd.BlockID))
	} else {
		return "C"
	}
}
