package memory;

import java.util.ArrayList;
import java.util.List;
import java.util.Optional;

import block.Block;

public class Memory {
  public final int size;
  public List<Block> used_blocks;
  public List<Block> free_blocks;

  public Memory(int size) {
    this.size = size;
    this.used_blocks = new ArrayList<Block>();
    this.free_blocks = new ArrayList<Block>();
    this.free_blocks.add(new BlockBuilder().setId(Optional.empty()).setSize(size).setStartAddr(0).setEndAddr(size).createBlock());
  }
}
