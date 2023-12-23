#include <stdlib.h>
#include <stdio.h>
#include <sys/time.h>
#include <unistd.h>
#include <sys/sysctl.h>
#include <sys/types.h>
#include <time.h>

#define SLEEP_SEC 3
#define NUM_MULS 100000000
#define NUM_MALLOCS 100000
#define MALLOC_SIZE 1000

struct profile_times {
  int pid;
  int cpu_cores;
  float user_time;
  float system_time;
  float real_time;
};

void profile_start(struct profile_times *t) {
  int ncpu;
  size_t len = sizeof(ncpu);

  // Get the number of CPU cores
  if (sysctlbyname("hw.ncpu", &ncpu, &len, NULL, 0) == -1) {
    perror("sysctl");
  }

  printf("Number of CPU cores: %d\n", ncpu);

  t->pid = getpid();
  t->cpu_cores = ncpu;
  t->user_time = clock();
  t->system_time = clock();
  t->real_time = clock();
}

void profile_log(struct profile_times *t, clock_t *end, struct timeval *start_real, struct timeval *end_real, struct rusage *usage) {
  double real_time = (end_real->tv_sec - start_real->tv_sec) + (end_real->tv_usec - start_real->tv_usec) / 1000000.0;
  double user_time = (usage->ru_utime.tv_sec + usage->ru_utime.tv_usec) / 1000000.0;
  double sys_time = (usage->ru_stime.tv_sec + usage->ru_stime.tv_usec) / 1000000.0;

  printf("[pid %d, cpu %d] real: %.3fs user: %.3fs sys: %.3fs \n", t->pid, t->cpu_cores, real_time, user_time, sys_time);
}

int main(int argc, char *argv[]) {
  struct profile_times t;
  clock_t end = clock();
  struct rusage usage;
  struct timeval start_real, end_real;

  // profile doing a bunch of floating point muls
  float x = 1.0;

  profile_start(&t);

  gettimeofday(&start_real, NULL);

  for (int i = 0; i < NUM_MULS; i++) {
    x *= 1.1;
  }

  end = clock();
  gettimeofday(&end_real, NULL);
  getrusage(RUSAGE_SELF, &usage);

  printf("[pid %d, cpu %d] 100000000 fmuls\n", t.pid, t.cpu_cores);
  profile_log(&t, &end, &start_real, &end_real, &usage);

  // profile doing a bunch of mallocs
  profile_start(&t);


  void *p;

  for (int i = 0; i < NUM_MALLOCS; i++) {
    p = malloc(MALLOC_SIZE);
  }

  end = clock();
  gettimeofday(&end_real, NULL);
  getrusage(RUSAGE_SELF, &usage);

  printf("[pid %d, cpu %d] 100000 mallocs of size 1000\n", t.pid, t.cpu_cores);
  profile_log(&t, &end, &start_real, &end_real, &usage);

  profile_start(&t);

  printf("Sleeping for %d seconds...\n", SLEEP_SEC);
  sleep(SLEEP_SEC);
  end = clock();
  gettimeofday(&end_real, NULL);
  getrusage(RUSAGE_SELF, &usage);

  printf("[pid %d, cpu %d] 100000 mallocs of size 1000\n", t.pid, t.cpu_cores);
  profile_log(&t, &end, &start_real, &end_real, &usage);
}
