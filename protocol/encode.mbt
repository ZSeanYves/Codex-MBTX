///|
/// Encode one event without the newline; all text goes through the JSON encoder.
pub fn encode_event(event : @runtime.Event) -> String {
  let fields : Map[String, Json] = Map([])
  match event {
    Started(id~, job_id~) => {
      fields["event"] = "started".to_json()
      fields["id"] = id.to_json()
      fields["job_id"] = job_id.to_json()
    }
    Output(id~, job_id~, chunk~) => {
      fields["event"] = stream_name(chunk.stream).to_json()
      fields["id"] = id.to_json()
      fields["job_id"] = job_id.to_json()
      fields["seq"] = chunk.seq.to_json()
      fields["data"] = chunk.data.to_json()
    }
    Completed(id~, job_id~, exit_code~, duration_ms~) => {
      fields["event"] = "completed".to_json()
      fields["id"] = id.to_json()
      fields["job_id"] = job_id.to_json()
      fields["exit_code"] = exit_code.to_json()
      fields["duration_ms"] = milliseconds(duration_ms)
    }
    Stopped(id~, job_id~, duration_ms~) => {
      fields["event"] = "stopped".to_json()
      fields["id"] = id.to_json()
      fields["job_id"] = job_id.to_json()
      fields["duration_ms"] = milliseconds(duration_ms)
    }
    Failed(id~, job_id~, kind~, message~) => {
      fields["event"] = "failed".to_json()
      fields["id"] = nullable(id)
      fields["job_id"] = nullable(job_id)
      fields["error"] = failure_json({ kind, message, })
    }
    JobOutput(id~, job_id~, snapshot~) => {
      fields["event"] = "job_output".to_json()
      fields["id"] = id.to_json()
      fields["job_id"] = job_id.to_json()
      fields["state"] = state_name(snapshot.state).to_json()
      fields["next_cursor"] = snapshot.next_cursor.to_json()
      fields["chunks"] = Json::array(snapshot.chunks.map(chunk_json))
      fields["exit_code"] = nullable(snapshot.exit_code)
      fields["duration_ms"] = match snapshot.duration_ms {
        Some(ms) => milliseconds(ms)
        None => Json::null()
      }
      fields["error"] = match snapshot.error {
        Some(error) => failure_json(error)
        None => Json::null()
      }
    }
    JobStop(id~, job_id~, state~) => {
      fields["event"] = "job_stop".to_json()
      fields["id"] = id.to_json()
      fields["job_id"] = job_id.to_json()
      fields["state"] = state_name(state).to_json()
    }
  }
  Json::object(fields).stringify()
}

///|
fn milliseconds(value : Int64) -> Json {
  Json::number(value.to_double(), repr=value.to_string())
}

///|
fn[T : ToJson] nullable(value : T?) -> Json {
  match value {
    Some(value) => value.to_json()
    None => Json::null()
  }
}

///|
fn failure_json(error : @runtime.JobFailure) -> Json {
  Json::object({
    "kind": error.kind.to_json(),
    "message": error.message.to_json(),
  })
}

///|
fn chunk_json(chunk : @runtime.Chunk) -> Json {
  Json::object({
    "seq": chunk.seq.to_json(),
    "stream": stream_name(chunk.stream).to_json(),
    "data": chunk.data.to_json(),
  })
}

///|
fn stream_name(stream : @runtime.Stream) -> String {
  match stream {
    Stdout => "stdout"
    Stderr => "stderr"
  }
}

///|
fn state_name(state : @runtime.State) -> String {
  match state {
    Running => "running"
    Stopping => "stopping"
    Completed => "completed"
    Failed => "failed"
    Stopped => "stopped"
  }
}
