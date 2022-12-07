package main

import (
	"fmt"

	"github.com/ingmarrr/goa1/core"
	"github.com/ingmarrr/goa1/rw"
)

func main() {
	data := rw.ReadFile("test.txt")
	size, cmds := rw.ToCmdList(data)
	ops := map[string]func(size core.Size, blocks core.BlockList) int{
		"FirstFit": core.FirstFit,
		"BestFit":  core.BestFit,
		"WorstFit": core.WorstFit,
	}

	dat := ""
	for key, op := range ops {
		mem := core.NewMemory(core.Size(size))
		for _, cmd := range cmds.Cmds {
			mem.Execute(cmd, op)
		}
		dat += key + ":\n" + mem.String() + "\n"
		fmt.Println(dat)
	}
	rw.WriteFile("test_out.txt", dat)

}
