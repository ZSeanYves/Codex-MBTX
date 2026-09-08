///|
pub(all) enum Command {
  Run(Request)
  JobOutput(id~ : String, job_id~ : String, cursor~ : Int)
  JobStop(id~ : String, job_id~ : String)
} derive(Debug)
