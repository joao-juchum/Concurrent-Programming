#include "bounded_buffer.h"
#include "cond_blocking_queue.h"
#include "sem_blocking_queue.h"

bool is_sem_impl; // Use the semaphore implementation or not
int pb_debug = 0;

// Initialise the blocking queue structure above. is_sem_impl specifies
// whether the implementation is a semaphore based implementation.
blocking_queue_t *blocking_queue_init(bool is_sem_impl, int length) {
  blocking_queue_t *b;
  if (is_sem_impl)
    b = sem_blocking_queue_init(length);
  else
    b = cond_blocking_queue_init(length);
  b->is_sem_impl = is_sem_impl;
  return b;
}

// Extract an element from queue. If the attempted operation is
// not possible immedidately, the method call blocks until it is.
void *blocking_queue_get(blocking_queue_t *b) {
  if (b->is_sem_impl)
    return sem_blocking_queue_get(b);
  else
    return cond_blocking_queue_get(b);
}

// Insert an element into queue. If the attempted operation is
// not possible immedidately, the method call blocks until it is.
void blocking_queue_put(blocking_queue_t *b, void *d) {
  if (b->is_sem_impl)
    sem_blocking_queue_put(b, d);
  else
    cond_blocking_queue_put(b, d);
}

// Extract an element from queue. If the attempted operation is not
// possible immedidately, return NULL. Otherwise, return the element.
void *blocking_queue_remove(blocking_queue_t *b) {
  if (b->is_sem_impl)
    return sem_blocking_queue_remove(b);
  else
    return cond_blocking_queue_remove(b);
}

// Insert an element into queue. If the attempted operation is
// not possible immedidately, return 0. Otherwise, return 1.
int blocking_queue_add(blocking_queue_t *b, void *d) {
  if (b->is_sem_impl)
    return sem_blocking_queue_add(b, d);
  else
    return cond_blocking_queue_add(b, d);
}

// Extract an element from queue. If the attempted operation is not
// possible immedidately, the method call blocks until it is, but
// waits no longer than the given timeout. Return the element if
// successful. Otherwise, return NULL.
void *blocking_queue_poll(blocking_queue_t *b, struct timespec *abstime) {
  if (b->is_sem_impl)
    return sem_blocking_queue_poll(b, abstime);
  else
    return cond_blocking_queue_poll(b, abstime);
}

// Insert an element into queue. If the attempted operation is not
// possible immedidately, the method call blocks until it is, but
// waits no longer than the given timeout. Return 0 if not
// successful. Otherwise, return 1.
int blocking_queue_offer(blocking_queue_t *b, void *d,
                         struct timespec *abstime) {
  if (b->is_sem_impl)
    return sem_blocking_queue_offer(b, d, abstime);
  else
    return cond_blocking_queue_offer(b, d, abstime);
}

int blocking_queue_size(blocking_queue_t *b) {
  return bounded_buffer_size(b->buffer);
}
