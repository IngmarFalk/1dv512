package core

type Option int

const (
	Some Option = iota
	None
)

type Op string

const (
	Alloc   Op = "A"
	Dealloc Op = "D"
	Compact Op = "C"
)

type Cmd struct {
	op       Op
	block_id Option
	size     Option
}

func (c *Cmd) New(op Op, block_id Option, size Option) {
	c.op = op
	c.block_id = block_id
	c.size = size
}

type CmdList struct {
	cmds []Cmd
}

func (c *CmdList) New() {
	c.cmds = make([]Cmd, 0)
}

func (c *CmdList) Add(cmd Cmd) {
	c.cmds = append(c.cmds, cmd)
}

func (c *CmdList) Remove(index int) {
	c.cmds = append(c.cmds[:index], c.cmds[index+1:]...)
}

func (c *CmdList) Get(index int) Cmd {
	return c.cmds[index]
}

func (c *CmdList) Len() int {
	return len(c.cmds)
}
