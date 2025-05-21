#ifndef BLOCKING_QUEUE_H
#define BLOCKING_QUEUE_H
#include "bounded_buffer.h"
#include <pthread.h>
#include <semaphore.h>
#include <stdbool.h>
#include <stdlib.h>

#define BLOCKING 0
#define NONBLOCKING 1
#define TIMEDOUT 2
extern bool is_sem_impl; // Use the semaphore implementation or not
extern int pb_debug;

// Protected buffer structure used for both implemantations.
typedef struct {
  bool is_sem_impl;
  bounded_buffer_t *buffer;
} blocking_queue_t;

// Initialise the protected buffer structure above. sem_impl specifies
// whether the implementation is a semaphore based implementation.
blocking_queue_t *blocking_queue_init(bool is_sem_impl, int length);

// Extract an element from buffer. If the attempted operation is
// not possible immedidately, the method call blocks until it is.
void *blocking_queue_get(blocking_queue_t *b);

// Insert an element into buffer. If the attempted operation is
// not possible immedidately, the method call blocks until it is.
void blocking_queue_put(blocking_queue_t *b, void *d);

// Extract an element from buffer. If the attempted operation is not
// possible immedidately, return NULL. Otherwise, return the element.
void *blocking_queue_remove(blocking_queue_t *b);

// Insert an element into buffer. If the attempted operation is
// not possible immedidately, return 0. Otherwise, return 1.
int blocking_queue_add(blocking_queue_t *b, void *d);

// Extract an element from buffer. If the attempted operation is not
// possible immedidately, the method call blocks until it is, but
// waits no longer than the given timeout. Return the element if
// successful. Otherwise, return NULL.
void *blocking_queue_poll(blocking_queue_t *b, struct timespec *abstime);

// Insert an element into buffer. If the attempted operation is not
// possible immedidately, the method call blocks until it is, but
// waits no longer than the given timeout. Return 0 if not
// successful. Otherwise, return 1.
int blocking_queue_offer(blocking_queue_t *b, void *d,
                         struct timespec *abstime);

int blocking_queue_size(blocking_queue_t *b);
#endif
