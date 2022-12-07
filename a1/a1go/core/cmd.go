package core

type Option int

const (
	None Option = -1
	Some Option = iota
)

func (o *Option) IsSome() bool {
	return *o != None
}

func (o *Option) IsNone() bool {
	return *o == None
}

type Op string

const (
	Alloc   Op = "A"
	Dealloc Op = "D"
	Compact Op = "C"
)

type Cmd struct {
	Op      Op
	BlockID Option
	Size    Option
}

func New(op Op, blockID Option, size Option) Cmd {
	return Cmd{
		Op:      op,
		BlockID: blockID,
		Size:    size,
	}
}

func NewAlloc(blockID Id, size Size) Cmd {
	return New(Alloc, Option(blockID), Option(size))
}

func NewDealloc(blockID Id) Cmd {
	return New(Dealloc, Option(blockID), None)
}

func NewCompact() Cmd {
	return New(Compact, None, None)
}

type CmdList struct {
	Cmds []Cmd
}

func (c *CmdList) New() {
	c.Cmds = make([]Cmd, 0)
}

func (c *CmdList) Add(cmd Cmd) {
	c.Cmds = append(c.Cmds, cmd)
}

func (c *CmdList) Remove(index int) {
	c.Cmds = append(c.Cmds[:index], c.Cmds[index+1:]...)
}

func (c *CmdList) Get(index int) Cmd {
	return c.Cmds[index]
}

func (c *CmdList) Len() int {
	return len(c.Cmds)
}
