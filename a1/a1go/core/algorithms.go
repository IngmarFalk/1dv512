package core

import "errors"

func FirstFit(size Size, blocks BlockList) (int, error) {
	for i := 0; i < blocks.Len(); i++ {
		block := blocks.Get(i)
		if block.Size >= size {
			return i, nil
		}
	}
	return -1, errors.New("no block large enough")
}

func BestFit(size Size, blocks BlockList) (int, error) {
	best := -1
	for i := 0; i < blocks.Len(); i++ {
		block := blocks.Get(i)
		if block.Size >= size {
			if best == -1 || block.Size < blocks.Get(best).Size {
				best = i
			}
		}
	}
	if best == -1 {
		return -1, errors.New("no block large enough")
	}
	return best, nil
}

func WorstFit(size Size, blocks BlockList) (int, error) {
	best := -1
	for i := 0; i < blocks.Len(); i++ {
		block := blocks.Get(i)
		if block.Size >= size {
			if best == -1 || block.Size > blocks.Get(best).Size {
				best = i
			}
		}
	}
	if best == -1 {
		return -1, errors.New("no block large enough")
	}
	return best, nil
}
