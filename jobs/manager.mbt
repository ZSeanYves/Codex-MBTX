///|
pub(all) suberror JobError {
  UnknownJob(String)
  Busy
  InvalidCursor
  InvalidLimits
} derive(Debug, Eq)

///|
/// Bounded, session-local job registry. Completed jobs are evicted in admission
/// order when space is needed. Active jobs are never evicted.
pub struct Manager {
  priv jobs : Array[Job]
  priv mut next_id : Int64
  priv max_active : Int
  priv max_retained : Int
}

///|
pub fn Manager::new(
  max_active? : Int = 4,
  max_retained? : Int = 32,
) -> Manager raise JobError {
  if max_active < 1 ||
    max_active > 32 ||
    max_retained < max_active ||
    max_retained > 128 {
    raise InvalidLimits
  }
  { jobs: [], next_id: 1L, max_active, max_retained, }
}

///|
/// Admit and start a job inside the caller's structured task group.
/// The returned job is live; waiting is optional for background execution.
pub async fn Manager::start(
  self : Manager,
  group : @async.TaskGroup[Unit],
  request : @runtime.Request,
  emit : async (@runtime.Event) -> Unit,
) -> Job {
  let active = self.jobs.iter().filter(job => !job.settled).count()
  if active >= self.max_active {
    raise Busy
  }
  if self.jobs.length() == self.max_retained {
    for index, job in self.jobs {
      if job.settled {
        self.jobs.remove(index) |> ignore
        break
      }
    }
  }
  let job = Job::{
    id: "job-\{self.next_id}",
    request_id: request.id,
    control: @process.Control::new(),
    state: Running,
    chunks: [],
    exit_code: None,
    duration_ms: None,
    error: None,
    settled: false,
    changed: @cond_var.Cond(),
    output_lock: @async.Mutex(),
  }
  self.next_id += 1L
  self.jobs.push(job)
  emit(Started(id=request.id, job_id=job.id))
  group.spawn_bg(() => job.execute(request, emit))
  job
}

///|
pub fn Manager::get(self : Manager, id : String) -> Job raise JobError {
  for job in self.jobs {
    if job.id == id {
      return job
    }
  }
  raise UnknownJob(id)
}
