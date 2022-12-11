package core

import "strconv"

type Size int
type Address int
type Id int

const (
	Free Id = -1
	Used
)

func Add[U Size | Address, V Size | Address](a U, b V) U {
	return U(int(a) + int(b))
}

func Sub[U Size | Address, V Size | Address](a U, b V) U {
	return U(int(a) - int(b))
}

type Block struct {
	Id        Id
	Size      Size
	StartAddr Address
	EndAddr   Address
}

func NewFree(size Size, start Address) Block {
	return Block{Free, size, start, Add(start, Address(size-1))}
}

func NewUsed(size Size, id Id, start Address) Block {
	return Block{id, size, start, Add(start, Address(size-1))}
}

func (b *Block) IsFree() bool {
	return b.Id == Free
}

func (b *Block) String() string {
	if b.Id == Free {
		return strconv.Itoa(int(b.StartAddr)) + ";" + strconv.Itoa(int(b.EndAddr)) + "\n"
	} else {
		return strconv.Itoa(int(b.Id)) + ";" + strconv.Itoa(int(b.StartAddr)) + ";" + strconv.Itoa(int(b.EndAddr)) + "\n"
	}
}

func (b *Block) AsFree() Block {
	return NewFree(b.Size, b.StartAddr)
}

func (b *Block) CanMerge(other Block) bool {
	return b.Id == Free && other.Id == Free && (other.StartAddr == b.EndAddr+1 || b.StartAddr == other.EndAddr+1)
}

func (b *Block) Merge(other Block) Block {
	if b.StartAddr < other.StartAddr {
		return NewFree(Add(b.Size, other.Size), b.StartAddr)
	} else {
		return NewFree(Add(b.Size, other.Size), other.StartAddr)
	}
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
		if block.Id == Free && block.Size >= size {
			return i, true
		}
	}
	return -1, false
}

func (b *BlockList) FindUsedBlock(addr Address) (int, bool) {
	for i, block := range b.blocks {
		if block.Id == Used && block.StartAddr <= addr && block.EndAddr > addr {
			return i, true
		}
	}
	return -1, false
}
