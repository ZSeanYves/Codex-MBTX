#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <unistd.h>
#include <string.h>

static unsigned long long sequence;

static void json_env(const char *name, char *buffer, size_t capacity) {
  const char *value = getenv(name);
  if (!value || !*value) {
    snprintf(buffer, capacity, "null");
    return;
  }
  size_t offset=0;
  buffer[offset++]='"';
  for (const unsigned char *p=(const unsigned char *)value;*p;p++) {
    if (offset+7>=capacity) {snprintf(buffer,capacity,"null");return;}
    if (*p<32 || *p=='"' || *p=='\\') {
      offset+=(size_t)snprintf(buffer+offset,capacity-offset,"\\u%04x",*p);
    } else {buffer[offset++]=(char)*p;}
  }
  buffer[offset++]='"';buffer[offset]=0;
}

void mbtx_launcher_trace(int event, int code, int child_pid) {
  const char *path = getenv("MBTX_LAUNCH_TRACE");
  if (!path || !*path) return;
  struct timespec ts;
  if (clock_gettime(CLOCK_MONOTONIC, &ts) != 0) return;
  const char *name = event == 0 ? "entry" : event == 1 ? "spawn_begin" : event == 2 ? "spawn_return" : event == 3 ? "wait_return" : event == 4 ? "cancellation_observed" : "kill_sent";
  char experiment_id[256], pair_id[256], attempt_id[256], task_id[256], backend[256];
  json_env("MBTX_EXPERIMENT_ID", experiment_id, sizeof(experiment_id));
  json_env("MBTX_PAIR_ID", pair_id, sizeof(pair_id));
  json_env("MBTX_ATTEMPT_ID", attempt_id, sizeof(attempt_id));
  json_env("MBTX_TASK_ID", task_id, sizeof(task_id));
  json_env("MBTX_BACKEND", backend, sizeof(backend));
  char line[2048];
  char wait_result[32]="null",exit_code[32]="null",signal_code[32]="null",child[32]="null";
  if (child_pid>0) snprintf(child,sizeof(child),"%d",child_pid);
  if (event==3) {
    snprintf(wait_result,sizeof(wait_result),"%d",code);
    if (code<0) snprintf(signal_code,sizeof(signal_code),"%d",-code);
    else snprintf(exit_code,sizeof(exit_code),"%d",code);
  }
  if (event==4 || event==5) snprintf(signal_code,sizeof(signal_code),"%d",code);
  int size = snprintf(line, sizeof(line),
    "{\"schema_version\":2,\"experiment_id\":%s,\"pair_id\":%s,\"attempt_id\":%s,\"task_id\":%s,\"backend\":%s,\"phase\":\"launcher\",\"event\":\"%s\",\"parent_id\":null,"
    "\"monotonic_ns\":%llu,\"clock_domain\":\"os-monotonic\",\"pid\":%ld,\"sequence\":%llu,"
    "\"ppid\":%ld,\"pgid\":%ld,\"child_pid\":%s,\"wait_result\":%s,\"exit_code\":%s,\"signal\":%s,\"stdout_bytes\":null,\"stderr_bytes\":null,\"status\":\"unknown\",\"failure_class\":null,\"confidence\":\"observed\"}\n",
    experiment_id, pair_id, attempt_id, task_id, backend, name,
    (unsigned long long)ts.tv_sec * 1000000000ULL + ts.tv_nsec,
    (long)getpid(), ++sequence, (long)getppid(), (long)getpgrp(), child,wait_result,exit_code,signal_code);
  int fd = open(path, O_WRONLY | O_CREAT | O_APPEND, 0600);
  if (fd < 0) return;
  if (size > 0 && size < (int)sizeof(line)) (void)write(fd, line, (size_t)size);
  close(fd);
}
