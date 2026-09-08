///|
/// Execute exactly one run request and drain its events without reading stdin.
/// This entry point lets a host approve the full request before spawning us.
pub async fn serve_request(line : String, output : &@io.Writer) -> Unit {
  let frame = try {
    if @utf8.encode(line).length() > 1024 * 1024 {
      raise @runtime.InvalidRequest("request exceeds 1 MiB")
    }
    @protocol.decode_request(line) |> ignore
    @json.parse(line).stringify() + "\n"
  } catch {
    error => {
      output.write(
        @protocol.encode_event(
          Failed(
            id=None,
            job_id=None,
            kind="invalid_request",
            message=error.to_string(),
          ),
        ) +
        "\n",
      )
      return
    }
  }
  let input = @io.MemoryReader(writer => writer.write(frame))
  defer input.close()
  serve(input, output)
}
