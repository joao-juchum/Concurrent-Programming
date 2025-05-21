#define _GNU_SOURCE
#include "utils.h"

#include <errno.h>
#include <pthread.h>
#include <semaphore.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

pthread_key_t thread_info_key;

// Start time as a timespec
struct timespec start_time;
// These are internal synchronization objects. Do not use them.
pthread_mutex_t resync_mutex;
pthread_cond_t resync_condvar;

void utils_init() {
  pthread_mutex_init(&resync_mutex, NULL);
  pthread_cond_init(&resync_condvar, NULL);

  pthread_key_create(&thread_info_key, NULL);

  // Assign a name and an id (0) to the main thread
  int *id = (int *)malloc(sizeof(int));
  *id = 0;
  set_current_thread_id(id, "main");
}

// Task names per id
char **thread_names;
int thread_names_length = 0;

void resynchronize() {
  int *id = (int *)pthread_getspecific(thread_info_key);
  struct timeval tv_now;
  struct timespec ts_resync;

  gettimeofday(&tv_now, NULL);
  TIMEVAL_TO_TIMESPEC(&tv_now, &ts_resync);
  ts_resync.tv_nsec = (*id) * 10000000;
  ts_resync.tv_sec = tv_now.tv_sec + 1;
  pthread_mutex_lock(&resync_mutex);
  pthread_cond_timedwait(&resync_condvar, &resync_mutex, &ts_resync);
  pthread_mutex_unlock(&resync_mutex);
}

void set_current_thread_id(int *id, char *name) {
  pthread_mutex_lock(&resync_mutex);
  pthread_setspecific(thread_info_key, (void *)id);
  if (thread_names_length <= *id) {
    thread_names_length = *id + 1;
    thread_names =
        (char **)realloc(thread_names, thread_names_length * sizeof(char *));
  }
  asprintf(&thread_names[*id], "%s %02d", name, *id);
  pthread_mutex_unlock(&resync_mutex);
}

void mtxprintf(int debug, char *format, ...) {
  va_list arglist;
  if (!debug)
    return;

  pthread_mutex_lock(&resync_mutex);
  int *id = (int *)pthread_getspecific(thread_info_key);
  printf("%03ld [%s] ", relative_clock() / 1000, thread_names[*id]);
  va_start(arglist, format);
  vprintf(format, arglist);
  va_end(arglist);
  pthread_mutex_unlock(&resync_mutex);
}

// Add msec milliseconds to timespec ts (seconds, nanoseconds)
void add_millis_to_timespec(struct timespec *ts, long msec) {
  long nsec = (msec % (long)1E3) * 1E6;
  long sec = msec / 1E3;
  ts->tv_nsec = ts->tv_nsec + nsec;
  if (1E9 <= ts->tv_nsec) {
    ts->tv_nsec = ts->tv_nsec - 1E9;
    ts->tv_sec++;
  }
  ts->tv_sec = ts->tv_sec + sec;
}

/* Delay until an absolute time. */
void delay_until(struct timespec *absolute_time) {
  struct timeval tv_now;
  struct timespec ts_now;
  struct timespec relative_time;

  gettimeofday(&tv_now, NULL);
  TIMEVAL_TO_TIMESPEC(&tv_now, &ts_now);
  relative_time.tv_nsec = absolute_time->tv_nsec - ts_now.tv_nsec;
  relative_time.tv_sec = absolute_time->tv_sec - ts_now.tv_sec;
  if (relative_time.tv_nsec < 0) {
    relative_time.tv_nsec = 1E9 + relative_time.tv_nsec;
    relative_time.tv_sec--;
  }
  if (relative_time.tv_sec < 0)
    return;

  nanosleep(&relative_time, NULL);
}

// Compute time elapsed from start time
long relative_clock() {
  struct timeval tv_now;
  struct timespec ts_now;

  gettimeofday(&tv_now, NULL);
  TIMEVAL_TO_TIMESPEC(&tv_now, &ts_now);

  ts_now.tv_nsec = ts_now.tv_nsec - start_time.tv_nsec;
  ts_now.tv_sec = ts_now.tv_sec - start_time.tv_sec;
  if (ts_now.tv_nsec < 0) {
    ts_now.tv_sec = ts_now.tv_sec - 1;
    ts_now.tv_nsec = ts_now.tv_nsec + 1E9;
  }
  return (ts_now.tv_sec * 1E3) + (ts_now.tv_nsec / 1E6);
}

// Return the start time
struct timespec get_start_time() { return start_time; }

// Store current time as the start time
void set_start_time() {
  struct timeval tv_start_time; // start time as a timeval

  gettimeofday(&tv_start_time, NULL);
  TIMEVAL_TO_TIMESPEC(&tv_start_time, &start_time);
}

// Read string in file f and store it in s. If there is an error,
// provide filename and line number (file:line).
int get_string(FILE *f, char *s, char *file, int line) {
  char b[64];
  char *c;
  ;

  while (fgets(b, 64, f) != NULL) {
    c = strchr(b, '\n');
    *c = '\0';
    if (strcmp(s, b) == 0)
      return 1;
  }
  printf("getString failed to catch %s in %s:%d\n", s, file, line);
  exit(1);
  return 0;
}

// Read long in file f and store it in l. If there is an error,
// provide filename and line number (file:line).
int get_long(FILE *f, long *l, char *file, int line) {
  char b[64];
  char *c;

  if (fgets(b, 64, f) != NULL) {
    c = strchr(b, '\n');
    *c = '\0';
    *l = strtol(b, NULL, 10);
    if ((*l != 0) || (errno != EINVAL))
      return 1;
  }
  return 0;
}

#ifdef DARWIN
int pthread_mutex_timedlock(pthread_mutex_t *mutex,
                            const struct timespec *abs_timeout) {
  int rc;
  struct timeval tv_now;
  struct timespec ts_now;
  struct timespec ts_sleep;

  while ((rc = pthread_mutex_trylock(mutex)) == EBUSY) {
    // Poll every 1ms
    gettimeofday(&tv_now, NULL);
    TIMEVAL_TO_TIMESPEC(&tv_now, &ts_now);
    if ((ts_now.tv_sec > abs_timeout->tv_sec) ||
        ((ts_now.tv_sec == abs_timeout->tv_sec) &&
         (ts_now.tv_nsec > abs_timeout->tv_nsec)))
      return ETIMEDOUT;
    ts_sleep.tv_sec = 0;
    ts_sleep.tv_nsec = 1E6;
    nanosleep(&ts_sleep, NULL);
  }
  return rc;
}

int sem_timedwait(sem_t *restrict sem, const struct timespec *abs_timeout) {
  int rc;
  struct timeval tv_now;
  struct timespec ts_now;
  struct timespec ts_sleep;

  while ((rc = sem_trywait(sem)) != 0) {
    // Poll every 1ms
    gettimeofday(&tv_now, NULL);
    TIMEVAL_TO_TIMESPEC(&tv_now, &ts_now);
    if ((ts_now.tv_sec > abs_timeout->tv_sec) ||
        ((ts_now.tv_sec == abs_timeout->tv_sec) &&
         (ts_now.tv_nsec > abs_timeout->tv_nsec)))
      return -1;
    ts_sleep.tv_sec = 0;
    ts_sleep.tv_nsec = 1E8;
    nanosleep(&ts_sleep, NULL);
  }
  return rc;
}
#endif
