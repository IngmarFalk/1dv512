
# Assignment 2

## Task 1

Create 2 threads where
  - T_a prints `A`
  - T_b prints `B`

Synchronize the threads (using semaphores) so that `AB` is printed 10 times

## Task 2

Create 4 threads where
  - T_a => `A`
  - T_b => `B`
  - T_c => `C`
  - T_d => `D`

Synchronize the threads (using semaphores) so that `ABACDC` is printed 5 times

## Task 3

Create a thread safe message queue (fifo)

Handle a single receiver && multiple senders

Note:
  - you are only allowed to use primitive arrays, not ArrayList/Vec/... => must use circular queue
  - max internal buffer size is 5
  - must use counting semaphores for managing internal buffer
