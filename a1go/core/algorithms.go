package core

func FirstFit(size Size, blocks BlockList) int {
	for i := 0; i < blocks.Len(); i++ {
		block := blocks.Get(i)
		if block.Size >= size {
			return i
		}
	}
	return -1
}

func BestFit(size Size, blocks BlockList) int {
	best := -1
	for i := 0; i < blocks.Len(); i++ {
		block := blocks.Get(i)
		if block.Size >= size {
			if best == -1 || block.Size < blocks.Get(best).Size {
				best = i
			}
		}
	}
	return best
}

func WorstFit(size Size, blocks BlockList) int {
	best := -1
	for i := 0; i < blocks.Len(); i++ {
		block := blocks.Get(i)
		if block.Size >= size {
			if best == -1 || block.Size > blocks.Get(best).Size {
				best = i
			}
		}
	}
	return best
}
