package rw

import (
	"os"
	"strconv"

	"github.com/ingmarrr/goa1/core"
)

func WriteFile(path string, data string) {
	err := os.WriteFile(path, []byte(data), 0644)
	check(err)
}

func CmdListToString(cmds core.CmdList) string {
	var data string
	for _, cmd := range cmds.Cmds {
		data += CmdToString(cmd) + "\n"
	}
	return data
}

func CmdToString(cmd core.Cmd) string {
	if cmd.Op == core.Alloc {
		return "A;" + strconv.Itoa(int(cmd.BlockID)) + ";" + strconv.Itoa(int(cmd.Size))
	} else if cmd.Op == core.Dealloc {
		return "D;" + strconv.Itoa(int(cmd.BlockID))
	} else {
		return "C"
	}
}
