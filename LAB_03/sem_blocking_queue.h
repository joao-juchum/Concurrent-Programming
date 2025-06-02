#ifndef SEM_BLOCKING_QUEUE_H
#define SEM_BLOCKING_QUEUE_H
#include "blocking_queue.h"
#include "bounded_buffer.h"
#include <pthread.h>
#include <stdlib.h>

// Initialise the blocking queue structure above.
blocking_queue_t *sem_blocking_queue_init(int length);

// Extract an element from queue. If the attempted operation is
// not possible immedidately, the method call blocks until it is.
void *sem_blocking_queue_get(blocking_queue_t *b);

// Insert an element into queue. If the attempted operation is
// not possible immedidately, the method call blocks until it is.
void sem_blocking_queue_put(blocking_queue_t *b, void *d);

// Extract an element from queue. If the attempted operation is not
// possible immedidately, return NULL. Otherwise, return the element.
void *sem_blocking_queue_remove(blocking_queue_t *b);

// Insert an element into queue. If the attempted operation is
// not possible immedidately, return 0. Otherwise, return 1.
int sem_blocking_queue_add(blocking_queue_t *b, void *d);

// Extract an element from queue. If the attempted operation is not
// possible immedidately, the method call blocks until it is, but
// waits no longer than the given timeout. Return the element if
// successful. Otherwise, return NULL.
void *sem_blocking_queue_poll(blocking_queue_t *b, struct timespec *abstime);

// Insert an element into queue. If the attempted operation is not
// possible immedidately, the method call blocks until it is, but
// waits no longer than the given timeout. Return 0 if not
// successful. Otherwise, return 1.
int sem_blocking_queue_offer(blocking_queue_t *b, void *d,
                             struct timespec *abstime);
#endif
