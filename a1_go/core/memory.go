package core

type Allocator interface {
	Alloc(size Size, id Id)
	Dealloc(id Id)
	Compact()
}

type Memory struct {
	size        Size
	free_blocks BlockList
	used_blocks BlockList
}

func (m *Memory) New(size Size) {
	m.size = size
	m.free_blocks.New()
	m.used_blocks.New()
	m.free_blocks.Add(Block{Free, size, 0, Add(Address(0), size)})
}

func (m *Memory) Alloc(size Size, id Id, fetch_block func(Id, Size, BlockList) int) {
	block_index := fetch_block(id, size, m.free_blocks)
	block := m.free_blocks.Get(block_index)
	m.free_blocks.Remove(block_index)
	if block.size > size {
		m.free_blocks.Add(Block{Free, Sub(block.size, size), Add(block.start_addr, size), block.end_addr})
	}
	m.used_blocks.Add(Block{id, size, block.start_addr, Add(block.start_addr, size)})
}

func (m *Memory) Dealloc(id Id) {
	for i := 0; i < m.used_blocks.Len(); i++ {
		block := m.used_blocks.Get(i)
		if block.id == id {
			m.used_blocks.Remove(i)
			m.free_blocks.Add(block)
			return
		}
	}
}

func (m *Memory) Compact() {
	for i := 0; i < m.free_blocks.Len(); i++ {
		block := m.free_blocks.Get(i)
		for j := 0; j < m.free_blocks.Len(); j++ {
			if i != j {
				other_block := m.free_blocks.Get(j)
				if block.end_addr == other_block.start_addr {
					block.size = Add(block.size, other_block.size)
					block.end_addr = other_block.end_addr
					m.free_blocks.Remove(j)
					i--
					break
				}
			}
		}
	}
}
