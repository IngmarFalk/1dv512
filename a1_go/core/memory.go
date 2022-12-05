package core

import (
	"fmt"
	"sort"
)

type Allocator interface {
	Alloc(size Size, id Id)
	Dealloc(id Id)
	Compact()
}

type Memory struct {
	Size       Size
	FreeBlocks BlockList
	UsedBlocks BlockList
}

func NewMemory(size Size) Memory {
	freeBlocks := BlockList{}
	freeBlocks.Add(NewFree(Size(size), Address(0)))
	return Memory{size, freeBlocks, BlockList{}}
}

func (m *Memory) Alloc(size Size, id Id, fetch_block func(Size, BlockList) int) {
	block_index := fetch_block(size, m.FreeBlocks)
	block := m.FreeBlocks.Get(block_index)
	m.FreeBlocks.Remove(block_index)
	if block.Size > size {
		m.FreeBlocks.Add(Block{Free, Sub(block.Size, size), Add(block.Start_addr, size), block.End_addr})
	}
	m.UsedBlocks.Add(Block{id, size, block.Start_addr, Add(block.Start_addr, size) - 1})
}

func (m *Memory) Dealloc(id Id) {
	for i := 0; i < m.UsedBlocks.Len(); i++ {
		block := m.UsedBlocks.Get(i)
		if block.Id == id {
			m.UsedBlocks.Remove(i)
			newFree := block.AsFree()
			for j := 0; j < m.FreeBlocks.Len(); j++ {
				free_block := m.FreeBlocks.Get(j)
				if free_block.CanMerge(newFree) {
					m.FreeBlocks.Remove(j)
					block = block.Merge(free_block)
					break
				}
			}
			m.FreeBlocks.Add(block.AsFree())
			return
		}
	}
}

func (m *Memory) Compact() {
	for i := 0; i < m.FreeBlocks.Len(); i++ {
		block := m.FreeBlocks.Get(i)
		for j := 0; j < m.FreeBlocks.Len(); j++ {
			if i != j {
				other_block := m.FreeBlocks.Get(j)
				if block.End_addr == other_block.Start_addr {
					block.Size = Add(block.Size, other_block.Size)
					block.End_addr = other_block.End_addr
					m.FreeBlocks.Remove(j)
					i--
					break
				}
			}
		}
	}
}

func (m *Memory) Execute(cmd Cmd, fetch_block func(Size, BlockList) int) {
	switch cmd.Op {
	case Alloc:
		m.Alloc(Size(cmd.Size), Id(cmd.BlockID), fetch_block)
	case Dealloc:
		m.Dealloc(Id(cmd.BlockID))
	case Compact:
		m.Compact()
	}
}

func (m *Memory) String() string {
	var str string
	str += "Allocated Blocks:\n"
	for _, block := range m.UsedBlocks.blocks {
		str += block.String()
	}
	str += "Free Blocks:\n"
	for _, block := range m.FreeBlocks.blocks {
		str += block.String()
	}
	str += "Fragmentation:\n" + fmt.Sprintf("%.16f", m.Fragmentation()) + "\n"
	return str
}

func (m *Memory) All() []Block {
	used := m.UsedBlocks.blocks
	free := m.FreeBlocks.blocks

	sort.Slice(used, func(i, j int) bool {
		return used[i].Start_addr < used[j].Start_addr
	})
	sort.Slice(free, func(i, j int) bool {
		return free[i].Start_addr < free[j].Start_addr
	})

	all := append(used, free...)
	return all
}

func (m *Memory) AsByteArray() []byte {
	bytes := make([]byte, m.Size)
	all := m.All()

	for _, block := range all {
		for i := range Range(block.Start_addr, block.End_addr) {
			if block.Id == Free {
				bytes[i] = 0
			} else {
				bytes[i] = 1
			}
		}
	}
	return bytes
}

func (m *Memory) Fragmentation() float64 {
	free := m.FreeBlocks.blocks
	var largest_i Size
	for i, b := range free {
		if i == 0 || b.Size > largest_i {
			largest_i = Size(i)
		}
	}
	largest := free[largest_i]
	free_memory := m.CalcFreeMemory()
	return 1 - (float64(largest.Size) / float64(free_memory))
}

func (m *Memory) CalcFreeMemory() Size {
	free_memory := Size(0)
	for i := 0; i < m.FreeBlocks.Len(); i++ {
		block := m.FreeBlocks.Get(i)
		free_memory = Add(free_memory, block.Size)
	}
	return free_memory
}

func Range[I Address | int](start I, end I) []int {
	var rnge []int
	for i := start; i < end; i++ {
		rnge = append(rnge, int(i))
	}
	return rnge
}
