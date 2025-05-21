#include "cond_blocking_queue.h"
#include "bounded_buffer.h"
#include "utils.h"
#include <errno.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>

// Initialise the blocking queue structure above.
blocking_queue_t *cond_blocking_queue_init(int length) {
  blocking_queue_t *b;
  b = (blocking_queue_t *)malloc(sizeof(blocking_queue_t));
  b->buffer = bounded_buffer_init(length);
  // Initialize the synchronization components
  return b;
}

// Extract an element from buffer. If the attempted operation is
// not possible immedidately, the method call blocks until it is.
void *cond_blocking_queue_get(blocking_queue_t *b) {
  void *d;

  // Enter mutual exclusion

  // Wait until there is a full slot to get data from the unprotected
  // bounded buffer (bounded_buffer_get).

  // Signal or broadcast that an empty slot is available in the
  // unprotected bounded buffer (if needed)

  d = bounded_buffer_get(b->buffer);
  if (d == NULL)
    mtxprintf(pb_debug, "get (B) - data=NULL\n");
  else
    mtxprintf(pb_debug, "get (B) - data=%d\n", *(int *)d);

  // Leave mutual exclusion

  return d;
}

// Insert an element into buffer. If the attempted operation is
// not possible immedidately, the method call blocks until it is.
void cond_blocking_queue_put(blocking_queue_t *b, void *d) {

  // Enter mutual exclusion

  // Wait until there is an empty slot to put data in the unprotected
  // bounded buffer (bounded_buffer_put).

  // Signal or broadcast that a full slot is available in the
  // unprotected bounded buffer (if needed)

  bounded_buffer_put(b->buffer, d);
  if (d == NULL)
    mtxprintf(pb_debug, "put (B) - data=NULL\n");
  else
    mtxprintf(pb_debug, "put (B) - data=%d\n", *(int *)d);

  // Leave mutual exclusion
}

// Extract an element from buffer. If the attempted operation is not
// possible immedidately, return NULL. Otherwise, return the element.
void *cond_blocking_queue_remove(blocking_queue_t *b) {
  void *d;

  // Enter mutual exclusion

  // Signal or broadcast that an empty slot is available in the
  // unprotected bounded buffer (if needed)

  d = bounded_buffer_get(b->buffer);
  if (d == NULL)
    mtxprintf(pb_debug, "remove (I)) - data=NULL\n");
  else
    mtxprintf(pb_debug, "remove (I)) - data=%d\n", *(int *)d);

  // Leave mutual exclusion
  return d;
}

// Insert an element into buffer. If the attempted operation is
// not possible immedidately, return 0. Otherwise, return 1.
int cond_blocking_queue_add(blocking_queue_t *b, void *d) {
  int done;

  // Enter mutual exclusion

  // Signal or broadcast that a full slot is available in the
  // unprotected bounded buffer (if needed)

  done = bounded_buffer_put(b->buffer, d);
  if (!done)
    d = NULL;

  if (d == NULL)
    mtxprintf(pb_debug, "add (I)) - data=NULL\n");
  else
    mtxprintf(pb_debug, "add (I)) - data=%d\n", *(int *)d);

  // Leave mutual exclusion
  return done;
}

// Extract an element from buffer. If the attempted operation is not
// possible immedidately, the method call blocks until it is, but
// waits no longer than the given timeout. Return the element if
// successful. Otherwise, return NULL.
void *cond_blocking_queue_poll(blocking_queue_t *b, struct timespec *abstime) {
  void *d = NULL;
  int rc = 0;

  // Enter mutual exclusion

  // Wait until there is a full slot to get data from the unprotected
  // bounded buffer  (bounded_buffer_get) but waits no longer than
  // the given timeout.

  // Signal or broadcast that an empty slot is available in the
  // unprotected bounded buffer (if needed)

  d = bounded_buffer_get(b->buffer);
  if (d == NULL)
    mtxprintf(pb_debug, "poll (T) - data=NULL\n");
  else
    mtxprintf(pb_debug, "poll (T) - data=%d\n", *(int *)d);

  // Leave mutual exclusion

  return d;
}

// Insert an element into buffer. If the attempted operation is not
// possible immedidately, the method call blocks until it is, but
// waits no longer than the given timeout. Return 0 if not
// successful. Otherwise, return 1.
int cond_blocking_queue_offer(blocking_queue_t *b, void *d,
                              struct timespec *abstime) {
  int rc = 0;
  int done = 0;

  // Enter mutual exclusion

  // Wait until there is an empty slot to put data in the unprotected
  // bounded buffer (bounded_buffer_put) but waits no longer than
  // the given timeout.

  // Signal or broadcast that a full slot is available in the
  // unprotected bounded buffer (if needed)

  done = bounded_buffer_put(b->buffer, d);
  if (!done)
    d = NULL;

  if (d == NULL)
    mtxprintf(pb_debug, "offer (T) - data=NULL\n");
  else
    mtxprintf(pb_debug, "offer (T) - data=%d\n", *(int *)d);

  // Leave mutual exclusion
  return done;
}
