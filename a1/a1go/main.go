package main

import (
	"fmt"

	"github.com/ingmarrr/goa1/core"
)

func main() {
	data := core.ReadFile("test.in")
	size, cmds := core.ToCmdList(data)
	ops := map[string]func(size core.Size, blocks core.BlockList) (int, error){
		"FirstFit": core.FirstFit,
		"BestFit":  core.BestFit,
		"WorstFit": core.WorstFit,
	}

	dat := ""
	for key, fn := range ops {
		mem := core.NewMemory(core.Size(size))
		for _, cmd := range cmds.Cmds {
			mem.Execute(cmd, fn)
		}
		dat += key + ":\n" + mem.String() + "\n"
		fmt.Println("")
	}
	core.WriteFile("test.out", dat)

}
