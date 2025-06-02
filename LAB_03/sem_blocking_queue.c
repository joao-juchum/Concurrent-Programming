#include "sem_blocking_queue.h"
#include "bounded_buffer.h"
#include "utils.h"
#include <errno.h>
#include <fcntl.h>
#include <pthread.h>
#include <semaphore.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/time.h>

#define EMPTY_SLOTS_NAME "/empty_slots"
#define FULL_SLOTS_NAME "/full_slots"

// Initialise the protected buffer structure above.
blocking_queue_t *sem_blocking_queue_init(int length) {
  blocking_queue_t *b;
  b = (blocking_queue_t *)malloc(sizeof(blocking_queue_t));
  b->buffer = bounded_buffer_init(length);
  // Initialize the synchronization attributes
  // Use these filenames as named semaphores
  sem_unlink(EMPTY_SLOTS_NAME);
  sem_unlink(FULL_SLOTS_NAME);
  // Open the semaphores using the filenames above
  b->empty_slots = sem_open(EMPTY_SLOTS_NAME, O_CREAT, 0600, length);
  b->full_slots  = sem_open(FULL_SLOTS_NAME,  O_CREAT, 0600, 0);
  pthread_mutex_init(&b->mutex, NULL);
  return b;
}

// Extract an element from buffer. If the attempted operation is
// not possible immedidately, the method call blocks until it is.
void *sem_blocking_queue_get(blocking_queue_t *b) {
  void *d;

  // Enforce synchronisation semantics using semaphores.
  sem_wait(b->full_slots);

  // Enter mutual exclusion.
  pthread_mutex_lock(&b->mutex);

  d = bounded_buffer_get(b->buffer);
  if (d == NULL)
    mtxprintf(pb_debug, "get (B) - data=NULL\n");
  else
    mtxprintf(pb_debug, "get (B) - data=%d\n", *(int *)d);

  // Leave mutual exclusion.
  pthread_mutex_unlock(&b->mutex);

  // Enforce synchronisation semantics using semaphores.
  sem_post(b->empty_slots);

  return d;
}

// Insert an element into buffer. If the attempted operation is
// not possible immedidately, the method call blocks until it is.
void sem_blocking_queue_put(blocking_queue_t *b, void *d) {

  // Enforce synchronisation semantics using semaphores.
  sem_wait(b->empty_slots);

  // Enter mutual exclusion.
  pthread_mutex_lock(&b->mutex);

  bounded_buffer_put(b->buffer, d);
  if (d == NULL)
    mtxprintf(pb_debug, "put (B) - data=NULL\n");
  else
    mtxprintf(pb_debug, "put (B) - data=%d\n", *(int *)d);

  // Leave mutual exclusion.
  pthread_mutex_unlock(&b->mutex);

  // Enforce synchronisation semantics using semaphores.
  sem_post(b->full_slots);
}

// Extract an element from buffer. If the attempted operation is not
// possible immedidately, return NULL. Otherwise, return the element.
void *sem_blocking_queue_remove(blocking_queue_t *b) {
  void *d = NULL;
  int rc = -1;

  // Enforce synchronisation semantics using semaphores.
  rc = sem_trywait(b->full_slots);

  if (rc != 0) {
    if (d == NULL)
      mtxprintf(pb_debug, "remove (I)) - data=NULL\n");
    else
      mtxprintf(pb_debug, "remove (I)) - data=%d\n", *(int *)d);
    return d;
  }

  // Enter mutual exclusion.
  pthread_mutex_lock(&b->mutex);

  d = bounded_buffer_get(b->buffer);
  if (d == NULL)
    mtxprintf(pb_debug, "remove (I)) - data=NULL\n");
  else
    mtxprintf(pb_debug, "remove (I)) - data=%d\n", *(int *)d);

  // Leave mutual exclusion.
  pthread_mutex_unlock(&b->mutex);

  // Enforce synchronisation semantics using semaphores.
  sem_post(b->empty_slots);

  return d;
}

// Insert an element into buffer. If the attempted operation is
// not possible immedidately, return 0. Otherwise, return 1.
int sem_blocking_queue_add(blocking_queue_t *b, void *d) {
  int rc = -1;

  // Enforce synchronisation semantics using semaphores.
  rc = sem_trywait(b->empty_slots);

  if (rc != 0) {
    d = NULL;
    if (d == NULL)
      mtxprintf(pb_debug, "add (I)) - data=NULL\n");
    else
      mtxprintf(pb_debug, "add (I)) - data=%d\n", *(int *)d);
    return 0;
  }

  // Enter mutual exclusion.
  pthread_mutex_lock(&b->mutex);

  bounded_buffer_put(b->buffer, d);
  if (d == NULL)
    mtxprintf(pb_debug, "add (I)) - data=NULL\n");
  else
    mtxprintf(pb_debug, "add (I)) - data=%d\n", *(int *)d);

  // Leave mutual exclusion.
  pthread_mutex_unlock(&b->mutex);

  // Enforce synchronisation semantics using semaphores.
  sem_post(b->full_slots);

  return 1;
}

// Extract an element from buffer. If the attempted operation is not
// possible immedidately, the method call blocks until it is, but
// waits no longer than the given timeout. Return the element if
// successful. Otherwise, return NULL.
void *sem_blocking_queue_poll(blocking_queue_t *b, struct timespec *abstime) {
  void *d = NULL;
  int rc = -1;

  // Enforce synchronisation semantics using semaphores.
  rc = sem_timedwait(b->full_slots, abstime);

  if (rc != 0) {
    if (d == NULL)
      mtxprintf(pb_debug, "poll (T) - data=NULL\n");
    else
      mtxprintf(pb_debug, "poll (T) - data=%d\n", *(int *)d);
    return d;
  }

  // Enter mutual exclusion.
  pthread_mutex_lock(&b->mutex);

  d = bounded_buffer_get(b->buffer);
  if (d == NULL)
    mtxprintf(pb_debug, "poll (T) - data=NULL\n");
  else
    mtxprintf(pb_debug, "poll (T) - data=%d\n", *(int *)d);

  // Leave mutual exclusion.
  pthread_mutex_unlock(&b->mutex);

  // Enforce synchronisation semantics using semaphores.
  sem_post(b->empty_slots);

  return d;
}

// Insert an element into buffer. If the attempted operation is not
// possible immedidately, the method call blocks until it is, but
// waits no longer than the given timeout. Return 0 if not
// successful. Otherwise, return 1.
int sem_blocking_queue_offer(blocking_queue_t *b, void *d,
                             struct timespec *abstime) {
  int rc;

  // Enforce synchronisation semantics using semaphores.
  rc = sem_timedwait(b->empty_slots, abstime);

  if (rc != 0) {
    d = NULL;
    if (d == NULL)
      mtxprintf(pb_debug, "offer (T) - data=NULL\n");
    else
      mtxprintf(pb_debug, "offer (T) - data=%d\n", *(int *)d);
    return 0;
  }

  // Enter mutual exclusion.
  pthread_mutex_lock(&b->mutex);

  bounded_buffer_put(b->buffer, d);
  if (d == NULL)
    mtxprintf(pb_debug, "offer (T) - data=NULL\n");
  else
    mtxprintf(pb_debug, "offer (T) - data=%d\n", *(int *)d);

  // Leave mutual exclusion.
  pthread_mutex_unlock(&b->mutex);

  // Enforce synchronisation semantics using semaphores.
  sem_post(b->full_slots);
  
  return 1;
}
