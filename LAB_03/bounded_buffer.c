#include <stdlib.h>
#include "bounded_buffer.h"

bounded_buffer_t * bounded_buffer_init(int max_size) {
  bounded_buffer_t * b =
    (bounded_buffer_t *)malloc(sizeof(bounded_buffer_t));
  b->first = 0;
  b->last  = -1;
  b->size = 0;
  b->max_size = max_size;
  b->buffer = (void *)malloc(max_size*sizeof(void *));
  return b;
}

void * bounded_buffer_get(bounded_buffer_t * b){
  void * d;
  if (b->size == 0) return NULL;
  d = b->buffer[b->first];
  b->first = (b-> first + 1) % b->max_size;
  b->size--;
  return d;
}

int bounded_buffer_put(bounded_buffer_t * b, void * d){
  if (b->size == b->max_size) return 0;
  b->last = (b->last + 1) % b->max_size;
  b->buffer[b->last] = d;
  b->size++;
  return 1;
}

int bounded_buffer_size(bounded_buffer_t * b) {
  return b->size;
}
   
