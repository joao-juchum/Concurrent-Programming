#ifndef BOUNDED_BUFFER_H
#define BOUNDED_BUFFER_H
typedef struct {
  int first, last, size, max_size;
  void ** buffer;
} bounded_buffer_t;

// Allocate and initialize the bounded buffer structure
bounded_buffer_t * bounded_buffer_init(int size);

// Remove an element from bounded buffer. When empty, return NULL.
void * bounded_buffer_get(bounded_buffer_t * b);

// Append an element into bounded buffer. When full, return 0.
int bounded_buffer_put(bounded_buffer_t * b, void * d);

int bounded_buffer_size(bounded_buffer_t * b);
#endif
