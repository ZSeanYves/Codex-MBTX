///|
/// Serve a JSONL session. EOF drains accepted work; transport failure cancels
/// the whole task group and reaps its direct children before propagating.
pub async fn serve(input : &@io.Reader, output : &@io.Writer) -> Unit {
  let manager = @jobs.Manager::new()
  let requests : @async.Queue[@runtime.Request] = @async.Queue(
    kind=Blocking(16),
  )
  let events : @async.Queue[@runtime.Event] = @async.Queue(kind=Blocking(64))
  @async.with_task_group() <| transport => {
    // Only this task writes protocol bytes. Job timeouts can cancel enqueueing,
    // but cannot interrupt half of a JSON frame and corrupt the next event.
    transport.spawn_bg(() => {
      for ;; {
        let event = events.get() catch {
          @aqueue.QueueAlreadyClosed => break
          error => raise error
        }
        output.write(@protocol.encode_event(event) + "\n")
      }
    })
    @async.with_task_group() <| workers => {
      workers.spawn_bg(() => {
        for ;; {
          let request = requests.get() catch {
            @aqueue.QueueAlreadyClosed => break
            error => raise error
          }
          let job = manager.start(workers, request, event => events.put(event)) catch {
            @jobs.Busy => {
              events.put(
                Failed(
                  id=Some(request.id),
                  job_id=None,
                  kind="busy",
                  message="active job limit reached",
                ),
              )
              continue
            }
            error => raise error
          }
          if !request.background {
            job.wait()
          }
        }
      })
      read_frames(
        input,
        line => {
          let command = @protocol.decode_command(line) catch {
            @runtime.InvalidRequest(message) => {
              events.put(
                Failed(id=None, job_id=None, kind="invalid_request", message~),
              )
              return
            }
          }
          match command {
            Run(request) =>
              if !requests.try_put(request) {
                events.put(
                  Failed(
                    id=Some(request.id),
                    job_id=None,
                    kind="busy",
                    message="pending run queue is full",
                  ),
                )
              }
            JobOutput(id~, job_id~, cursor~) =>
              control_reply(manager, events, id, job_id, Some(cursor))
            JobStop(id~, job_id~) =>
              control_reply(manager, events, id, job_id, None)
          }
        },
        () => {
          events.put(
            Failed(
              id=None,
              job_id=None,
              kind="invalid_request",
              message="request exceeds 1 MiB or is not UTF-8",
            ),
          )
        },
      )
      requests.close()
    }
    events.close()
  }
}

///|
async fn control_reply(
  manager : @jobs.Manager,
  events : @async.Queue[@runtime.Event],
  id : String,
  job_id : String,
  cursor : Int?,
) -> Unit {
  let event = try {
    let job = manager.get(job_id)
    match cursor {
      Some(cursor) =>
        @runtime.Event::JobOutput(id~, job_id~, snapshot=job.output(cursor~))
      None => @runtime.Event::JobStop(id~, job_id~, state=job.stop())
    }
  } catch {
    @jobs.UnknownJob(_) =>
      Failed(
        id=Some(id),
        job_id=Some(job_id),
        kind="unknown_job",
        message="unknown or evicted job",
      )
    @jobs.InvalidCursor =>
      Failed(
        id=Some(id),
        job_id=Some(job_id),
        kind="invalid_cursor",
        message="cursor is beyond captured output",
      )
    error => raise error
  }
  events.put(event)
}
