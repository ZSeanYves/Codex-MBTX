#include <errno.h>
#include <signal.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

static volatile sig_atomic_t requested_signal;
static volatile sig_atomic_t child_pid;
static volatile sig_atomic_t group_delivery;
static struct sigaction previous[3];
static const int signals[3] = {SIGINT, SIGTERM, SIGHUP};

static void receive_signal(int sig) {
  int saved_errno = errno;
  requested_signal = sig;
  if (!group_delivery && child_pid > 0) (void)kill(child_pid, sig);
  errno = saved_errno;
}

int mbtx_cancellation_start(void) {
  requested_signal = 0;
  child_pid = 0;
  const char *scope = getenv("MBTX_CANCEL_SCOPE");
  group_delivery = scope && strcmp(scope, "caller-process-group") == 0;
  /* This is a launcher control, not part of the child's environment. */
  if (group_delivery) unsetenv("MBTX_CANCEL_SCOPE");
  struct sigaction action;
  memset(&action, 0, sizeof(action));
  action.sa_handler = receive_signal;
  sigemptyset(&action.sa_mask);
  action.sa_flags = SA_RESTART;
  for (int i = 0; i < 3; i++) {
    if (sigaction(signals[i], &action, &previous[i]) != 0) {
      int error = errno;
      while (i-- > 0) sigaction(signals[i], &previous[i], NULL);
      return error;
    }
  }
  return 0;
}

void mbtx_cancellation_child(int pid) {
  child_pid = pid;
  if (requested_signal && !group_delivery) (void)kill(pid, requested_signal);
}

int mbtx_cancellation_signal(void) { return requested_signal; }

void mbtx_cancellation_kill(int pid) { (void)kill(pid, SIGKILL); }

void mbtx_cancellation_finish(void) {
  child_pid = 0;
  for (int i = 0; i < 3; i++) sigaction(signals[i], &previous[i], NULL);
}
