package block;

import java.util.Optional;

public class Block {
  public Optional<Integer> id;
  public int size;
  public int startAddr;
  public int endAddr;

  public Block(Optional<Integer> id, int size, int startAddr, int endAddr) {
    this.id = id;
    this.size = size;
    this.startAddr = startAddr;
    this.endAddr = endAddr;
  }

  public boolean isEmpty() {
    return id.isEmpty();
  }
}



