#define _GNU_SOURCE
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/time.h>
#include <unistd.h>

#include "blocking_queue.h"
#include "bounded_buffer.h"
#include "utils.h"

blocking_queue_t *blocking_queue;
pthread_t *tasks;

long sem_producers;   // Sem prod BLOCKING 0, NONBLOCKING 1, TIMEDOUT 2
long sem_consumers;   // Sem cons BLOCKING 0, NONBLOCKING 1, TIMEDOUT 2
long buffer_size;     // Size of the protected buffer
long n_values;        // Number of produced / consumed values
long n_consumers;     // Number of consumers
long n_producers;     // Number of producers
long consumer_period; // Period of consumer (millis)
long producer_period; // Period of producer (millis)

// Main consumer. Get consumer id as argument.
void *main_consumer(void *arg) {
  int i;
  int *id = (int *)arg;
  int *data;

  set_current_thread_id(id, "consumer");
  printf("start consumer %d\n", *id);

  // Get start time t0, the deadline will be t0 + T
  struct timespec deadline = get_start_time();

  for (i = 0; i < (n_values / n_consumers); i++) {
    // Behave as a periodic task. the current deadline corresponds to
    // the previous deadline + one period
    add_millis_to_timespec(&deadline, consumer_period);
    resynchronize();
    switch (sem_consumers) {
    case BLOCKING:
      data = (int *)blocking_queue_get(blocking_queue);
      break;
    case NONBLOCKING:
      data = (int *)blocking_queue_remove(blocking_queue);
      break;
    case TIMEDOUT:
      data = (int *)blocking_queue_poll(blocking_queue, &deadline);
      break;
    default:;
    }
    if (data != NULL)
      free(data);
    delay_until(&deadline);
  }
  pthread_exit(NULL);
  return NULL;
}

// Main producer. Get producer id as argument.
void *main_producer(void *arg) {
  int i;
  int *id = (int *)arg;
  int *data;
  long done;

  set_current_thread_id(id, "producer");
  printf("start producer %d\n", *id);

  // Get start time t0, the deadline will be t0 + T
  struct timespec deadline = get_start_time();

  for (i = 0; i < (n_values / n_producers); i++) {

    // Allocate data in order to produce and consume it
    data = (int *)malloc(sizeof(int));

    // Data is split in two parts : first the thread number and the
    // number of data produced.
    *data = *(int *)(arg) * 100 + i;

    // Behave as a periodic task. the current deadline corresponds to
    // the previous deadline + one period.
    add_millis_to_timespec(&deadline, producer_period);
    resynchronize();

    switch (sem_producers) {
    case BLOCKING:
      blocking_queue_put(blocking_queue, data);
      done = 1;
      break;

    case NONBLOCKING:
      done = blocking_queue_add(blocking_queue, data);
      break;

    case TIMEDOUT:
      done = blocking_queue_offer(blocking_queue, data, &deadline);
      break;
    default:;
    }
    if (!done)
      data = NULL;
    delay_until(&deadline);
  }
  pthread_exit(NULL);
  return NULL;
}

// Read scenario file
void read_file(char *filename);

int main(int argc, char *argv[]) {

  pb_debug =1;

  int i;
  int *data;

  if (argc != 2) {
    printf("Usage : %s <scenario file>\n", argv[0]);
    exit(1);
  }
  utils_init();
  read_file(argv[1]);

  blocking_queue = blocking_queue_init(is_sem_impl, buffer_size);


  set_start_time();

  // Create consumers and then producers. Pass the *value* of i
  // as parametre of the main procedure (main_consumer or main_producer).

  // Threads space
  tasks = malloc((n_consumers + n_producers) * sizeof(pthread_t));

  // Consumers thread
  for (i = 0; i < n_consumers; i++) {
    int *arg = malloc(sizeof(int));
    *arg = i;
    //printf("Consumers arg: %p \n", arg);
    pthread_create(&tasks[i], NULL, main_consumer, arg);
  }

  // Producers thread
  for (i = n_consumers; i < n_producers + n_consumers; i++) {
    int *arg = malloc(sizeof(int));
    *arg = i;
    //printf("Producers arg: %p \n", arg);
    pthread_create(&tasks[i], NULL, main_producer, arg);
  }

  // Wait for producers and consumers termination
  for (i = 0; i < n_consumers + n_producers; i++) {
    pthread_join(tasks[i], NULL);
  }
}

void read_file(char *filename) {
  FILE *file;

  file = fopen(filename, "r");
  if (file == NULL) {
    printf("no such file %s\n", filename);
    exit(1);
  }
  get_string(file, "#sem_impl", __FILE__, __LINE__);
  get_long(file, (long *)&is_sem_impl, __FILE__, __LINE__);
  printf("sem_impl = %ld\n", (long)is_sem_impl);

  get_string(file, "#sem_consumers", __FILE__, __LINE__);
  get_long(file, (long *)&sem_consumers, __FILE__, __LINE__);
  printf("sem_consumers = %ld\n", sem_consumers);

  get_string(file, "#sem_producers", __FILE__, __LINE__);
  get_long(file, (long *)&sem_producers, __FILE__, __LINE__);
  printf("sem_producers = %ld\n", sem_producers);

  get_string(file, "#buffer_size", __FILE__, __LINE__);
  get_long(file, (long *)&buffer_size, __FILE__, __LINE__);
  printf("buffer_size = %ld\n", buffer_size);

  get_string(file, "#n_values", __FILE__, __LINE__);
  get_long(file, (long *)&n_values, __FILE__, __LINE__);
  printf("n_values = %ld\n", n_values);

  get_string(file, "#n_consumers", __FILE__, __LINE__);
  get_long(file, (long *)&n_consumers, __FILE__, __LINE__);
  printf("n_consumers = %ld\n", n_consumers);

  get_string(file, "#n_producers", __FILE__, __LINE__);
  get_long(file, (long *)&n_producers, __FILE__, __LINE__);
  printf("n_producers = %ld\n", n_producers);

  get_string(file, "#consumer_period", __FILE__, __LINE__);
  get_long(file, (long *)&consumer_period, __FILE__, __LINE__);
  printf("consumer_period = %ld\n", consumer_period);

  get_string(file, "#producer_period", __FILE__, __LINE__);
  get_long(file, (long *)&producer_period, __FILE__, __LINE__);
  printf("producer_period = %ld\n", producer_period);
}
