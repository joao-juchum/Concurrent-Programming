# Answers of the TD2

To compile programs you can then can use the following command:

```bash

gcc -pedantic -Werror -Wall -std=c11 -pthread -O3 -g -o <you-program> <your-c11-code>

```

### 1 Simple thread
1. What is pthread_join() actually doing here?

### 2 Array with two producers
1. Is the program race-free? Explain your answer in detail.

2. Which kind of initerleavings are possible for this program? Which interleavings can you observe? What might be happening here?

### 3 Total store order
1. Will this program ever terminate? What do you observe when you run the program (maybe try running it several times)?

2. Could this be an artifact of the memory model of the computer architecture of your PC (x86)?

3. Call atomic_thread_fence() after the assignments to x and y for both threads. What happens now?

4. Is the program race-free? Explain your answer in detail.

5. What does this example demonstrate?

### 4 Implementing mutex
1. Which guarantees have to be provided by the memory model with regard to the two counters in order for the mutex algorithm to be correct?

2. Try to change your implementation such that the turn counter is not atomic, but only a normal variable. Is this safe? How many threads will execute the increment of turn?

3. What is the problem with this implementation of a mutex?