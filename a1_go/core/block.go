package core

type Size int
type Address int
type Id int

const (
	Used Id = iota
	Free
)

func Add[U Size | Address, V Size | Address](a U, b V) U {
	return U(int(a) + int(b))
}

func Sub[U Size | Address, V Size | Address](a U, b V) U {
	return U(int(a) - int(b))
}

type Data struct {
	size       Size
	start_addr Address
	end_addr   Address
}

func (d *Data) New(size Size, start_addr Address) {
	d.size = size
	d.start_addr = start_addr
	d.end_addr = Add(start_addr, size)
}

type Block struct {
	id   Id
	data Data
}

type BlockList struct {
	blocks []Block
}

func (b *BlockList) New() {
	b.blocks = make([]Block, 0)
}

func (b *BlockList) Add(block Block) {
	b.blocks = append(b.blocks, block)
}

func (b *BlockList) Remove(index int) {
	b.blocks = append(b.blocks[:index], b.blocks[index+1:]...)
}

func (b *BlockList) Get(index int) Block {
	return b.blocks[index]
}

func (b *BlockList) Len() int {
	return len(b.blocks)
}

func (b *BlockList) FindFreeBlock(size Size) (int, bool) {
	for i, block := range b.blocks {
		if block.id == Free && block.data.size >= size {
			return i, true
		}
	}
	return -1, false
}

func (b *BlockList) FindUsedBlock(addr Address) (int, bool) {
	for i, block := range b.blocks {
		if block.id == Used && block.data.start_addr <= addr && block.data.end_addr > addr {
			return i, true
		}
	}
	return -1, false
}
