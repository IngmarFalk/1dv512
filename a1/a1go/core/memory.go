package core

import (
	"fmt"
	"sort"
)

type Data struct {
	Op       Op
	BlockID  Id
	InstrNum int
	Param    int
}

func Default() Data {
	return Data{"", 0, 0, 0}
}

func (d *Data) String() string {
	return fmt.Sprintf("%s;%d;%d", d.Op, d.InstrNum, d.Param)
}

type Result struct {
	Data    Data
	isError bool
}

func Ok() Result {
	return Result{Default(), false}
}

func Err(data Data) Result {
	return Result{data, true}
}

type Memory struct {
	Size             Size
	FreeBlocks       BlockList
	UsedBlocks       BlockList
	Errors           []Data
	InstructionCount int
	OutputCount      int
}

func NewMemory(size Size) Memory {
	freeBlocks := BlockList{}
	freeBlocks.Add(NewFree(Size(size), Address(0)))
	return Memory{size, freeBlocks, BlockList{}, []Data{}, 0, 0}
}

func (m *Memory) Alloc(size Size, id Id, fn func(Size, BlockList) (int, error)) Result {
	block_index, err := fn(size, m.FreeBlocks)
	if err != nil {
		var largest Block
		for _, block := range m.FreeBlocks.blocks {
			if block.Size > largest.Size {
				largest = block
			}
		}
		return Err(Data{"A", id, m.InstructionCount, int(largest.Size)})
	}
	block := m.FreeBlocks.Get(block_index)
	m.FreeBlocks.Remove(block_index)
	if block.Size > size {
		m.FreeBlocks.Add(Block{Free, Sub(block.Size, size), Add(block.StartAddr, size), block.EndAddr})
	}
	m.UsedBlocks.Add(Block{id, size, block.StartAddr, Add(block.StartAddr, size) - 1})
	return Ok()
}

func (m *Memory) Dealloc(id Id) Result {
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
			return Ok()
		}
	}
	var reason int
	if m.didTryToAllocAlready(id) {
		reason = 1
	} else {
		reason = 0
	}
	return Err(Data{"D", id, m.InstructionCount, reason})
}

func (m *Memory) Compact() {
	for i := 0; i < m.FreeBlocks.Len(); i++ {
		block := m.FreeBlocks.Get(i)
		for j := 0; j < m.FreeBlocks.Len(); j++ {
			if i != j {
				other_block := m.FreeBlocks.Get(j)
				if block.EndAddr == other_block.StartAddr {
					block.Size = Add(block.Size, other_block.Size)
					block.EndAddr = other_block.EndAddr
					m.FreeBlocks.Remove(j)
					i--
					break
				}
			}
		}
	}
}

func (m *Memory) Output() string {
	out := "Size:\n" + fmt.Sprintf("%d", m.Size) + "\n"
	out += "Used Blocks:\n"
	sort.Slice(m.UsedBlocks.blocks, func(i, j int) bool {
		return m.UsedBlocks.blocks[i].Id < m.UsedBlocks.blocks[j].Id
	})
	for _, block := range m.UsedBlocks.blocks {
		out += block.String()
	}
	out += "Free Blocks:\n"
	sort.Slice(m.FreeBlocks.blocks, func(i, j int) bool {
		return m.FreeBlocks.blocks[i].StartAddr < m.FreeBlocks.blocks[j].StartAddr
	})
	for _, block := range m.FreeBlocks.blocks {
		out += block.String()
	}
	out += "Fragmentation:\n" + fmt.Sprintf("%.6f", m.Fragmentation()) + "\n"
	out += "Errors:\n"
	for _, err := range m.Errors {
		out += err.String()
	}
	if len(m.Errors) == 0 {
		out += "None"
	}
	fmt.Println(out)
	m.OutputCount++
	return out
}

func (m *Memory) Execute(cmd Cmd, fn func(Size, BlockList) (int, error)) {
	fmt.Println(cmd)
	switch cmd.Op {
	case Alloc:
		res := m.Alloc(Size(cmd.Size), Id(cmd.BlockID), fn)
		if res.isError {
			m.Errors = append(m.Errors, res.Data)
		}
	case Dealloc:
		res := m.Dealloc(Id(cmd.BlockID))
		if res.isError {
			m.Errors = append(m.Errors, res.Data)
		}
	case Compact:
		m.Compact()
	case Output:
		var prevOutput int
		prev := ""
		if m.OutputCount == 0 {
			prevOutput = 0
		} else {
			prevOutput = m.OutputCount - 1
		}
		for i := prevOutput; i < m.OutputCount; i++ {
			prev += ReadFile(fmt.Sprintf("output.out%d", prevOutput))
		}
		prev += m.Output()
		fmt.Println(prev)
		WriteFile(fmt.Sprintf("output.out%d", m.OutputCount), prev)
	}
}

func (m *Memory) didTryToAllocAlready(id Id) bool {
	for _, err := range m.Errors {
		if err.BlockID == id {
			return true
		}
	}
	return false
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
		return used[i].StartAddr < used[j].StartAddr
	})
	sort.Slice(free, func(i, j int) bool {
		return free[i].StartAddr < free[j].StartAddr
	})

	all := append(used, free...)
	return all
}

func (m *Memory) AsByteArray() []byte {
	bytes := make([]byte, m.Size)
	all := m.All()

	for _, block := range all {
		for i := range Range(block.StartAddr, block.EndAddr) {
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
