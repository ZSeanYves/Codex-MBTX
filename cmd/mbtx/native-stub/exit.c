#include <signal.h>
#include <stdlib.h>

void mbtx_process_exit(int code) {
  if (code < 0) {
    int signal_number = -code;
    signal(signal_number, SIG_DFL);
    raise(signal_number);
    _Exit(128 + signal_number);
  }
  _Exit(code);
}
