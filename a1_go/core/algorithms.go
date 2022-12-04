package core

func FirstFit(id Id, size Size, blocks BlockList) int {
	for i := 0; i < blocks.Len(); i++ {
		block := blocks.Get(i)
		if block.size >= size {
			return i
		}
	}
	return -1
}

func BestFit(id Id, size Size, blocks BlockList) int {
	best := -1
	for i := 0; i < blocks.Len(); i++ {
		block := blocks.Get(i)
		if block.size >= size {
			if best == -1 || block.size < blocks.Get(best).size {
				best = i
			}
		}
	}
	return best
}

func WorstFit(id Id, size Size, blocks BlockList) int {
	best := -1
	for i := 0; i < blocks.Len(); i++ {
		block := blocks.Get(i)
		if block.size >= size {
			if best == -1 || block.size > blocks.Get(best).size {
				best = i
			}
		}
	}
	return best
}
