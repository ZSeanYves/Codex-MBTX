///|
pub struct Case {
  id : String
  workload : String
  prompt : String
  inputs : Map[String, String]
  expected : Json
  editable : String?
  background : Bool
} derive(ToJson)

///|
pub fn suite() -> Array[Case] {
  [
    {
      id: "json_totals",
      workload: "script",
      prompt: "Read orders.json. Sum cents by customer for paid orders only. Write result.json as an object mapping customer names to total cents, including paid zero amounts. Preserve every input file.",
      inputs: {
        "orders.json": "[{\"customer\":\"Ada\",\"cents\":1250,\"status\":\"paid\"},{\"customer\":\"Lin\",\"cents\":400,\"status\":\"void\"},{\"customer\":\"Ada\",\"cents\":-50,\"status\":\"paid\"},{\"customer\":\"Lin\",\"cents\":0,\"status\":\"paid\"},{\"customer\":\"Sam\",\"cents\":275,\"status\":\"paid\"}]",
      },
      expected: { "Ada": 1200, "Lin": 0, "Sam": 275 },
      editable: None,
      background: false,
    },
    {
      id: "jsonl_recovery",
      workload: "script",
      prompt: "Read records.jsonl line by line. Skip blank lines. Valid records are JSON objects with integer id and nonnegative integer count; extra keys are allowed. Write result.json with valid_ids in input order, total_count, and rejected_lines (1-based physical line numbers). Preserve every input file.",
      inputs: {
        "records.jsonl": "{\"id\":4,\"count\":3}\n\nnot json\n{\"id\":7,\"count\":-1}\n{\"id\":9,\"count\":0,\"x\":true}\n{\"id\":10,\"count\":2.5}\n{\"id\":11,\"count\":8}\n",
      },
      expected: {
        "valid_ids": [4, 9, 11],
        "total_count": 11,
        "rejected_lines": [3, 4, 6],
      },
      editable: None,
      background: false,
    },
    {
      id: "path_inventory",
      workload: "script",
      prompt: "Read manifest.json listing relative file paths. For each listed UTF-8 file, count nonempty lines (do not trim spaces; split on LF), and report its exact text. Write result.json as an array in manifest order with path, nonempty_lines, text. Do not execute any filenames or modify input files.",
      inputs: {
        "manifest.json": "[\"data/alpha space.txt\",\"data/quote'$.txt\",\"data/nested/utf8.txt\"]",
        "data/alpha space.txt": "first\n\nlast\n",
        "data/quote'$.txt": "literal $HOME; `echo unsafe`\n",
        "data/nested/utf8.txt": "\u4f60\u597d\n\u4e16\u754c\n",
      },
      expected: [
        {
          "path": "data/alpha space.txt",
          "nonempty_lines": 2,
          "text": "first\n\nlast\n",
        },
        {
          "path": "data/quote'$.txt",
          "nonempty_lines": 1,
          "text": "literal $HOME; `echo unsafe`\n",
        },
        {
          "path": "data/nested/utf8.txt",
          "nonempty_lines": 2,
          "text": "\u4f60\u597d\n\u4e16\u754c\n",
        },
      ],
      editable: None,
      background: false,
    },
    {
      id: "literal_argv",
      workload: "script",
      prompt: "Run the existing capture.mbtx using moon run from the workspace directory, passing exactly three arguments: alpha beta, quote'\"$, and $(touch SHOULD_NOT_EXIST). The script must create result.json. Do not modify capture.mbtx or manually write result.json. Preserve literal arguments without shell expansion.",
      inputs: {
        "capture.mbtx": "import {\n  \"moonbitlang/async\",\n  \"moonbitlang/core/env\",\n  \"moonbitlang/async/fs\",\n}\nasync fn main {\n  let args = @env.args()\n  @fs.write_file(\"result.json\", args[1:].to_owned().to_json().stringify())\n}\n",
      },
      expected: ["alpha beta", "quote'\"$", "$(touch SHOULD_NOT_EXIST)"],
      editable: None,
      background: false,
    },
    {
      id: "repair_moon",
      workload: "script",
      prompt: "Repair only the function sum_positive in broken.mbtx so it sums strictly positive integers. Run the repaired file with moon run to produce result.json. Preserve the main function and do not manually create result.json.",
      inputs: {
        "broken.mbtx": "import { \"moonbitlang/async\", \"moonbitlang/async/fs\" }\nfn sum_positive(xs : Array[Int]) -> Int {\n  let mut total = 0\n  for x in xs { if x >= 0 { total = x } }\n  total\n}\nasync fn main {\n  let values = [sum_positive([3,-8,2,0,5]), sum_positive([-1,-2]), sum_positive([]), sum_positive([9,1])]\n  @fs.write_file(\"result.json\", values.to_json().stringify())\n}\n",
      },
      expected: [10, 0, 0, 10],
      editable: Some("broken.mbtx"),
      background: false,
    },
    {
      id: "background_job",
      workload: "script",
      prompt: "Start worker.mbtx (moon run worker.mbtx) as a background process and poll it until completion using the available command execution tools. Verify result.json. Do not modify worker.mbtx or write result.json yourself.",
      inputs: {
        "worker.mbtx": "import {\n  \"moonbitlang/async\",\n  \"moonbitlang/async/fs\",\n}\nasync fn main {\n  println(\"started\")\n  @async.sleep(3500)\n  let result : Json = { \"state\": \"complete\", \"steps\": 3 }\n  @fs.write_file(\"result.json\", result.stringify())\n  println(\"complete\")\n}\n",
      },
      expected: { "state": "complete", "steps": 3 },
      editable: None,
      background: true,
    },
    {
      id: "host_literal_argv",
      workload: "process",
      prompt: "Run the existing capture_args.py with python3 from the workspace. Pass exactly these three arguments: alpha beta, quote'\"$, and $(touch SHOULD_NOT_EXIST). The helper must create result.json. Do not modify capture_args.py or manually write result.json. Preserve literal arguments without shell expansion.",
      inputs: {
        "capture_args.py": "import json,sys\nwith open('result.json','w',encoding='utf-8') as f:\n json.dump(sys.argv[1:],f,ensure_ascii=False)\n",
      },
      expected: ["alpha beta", "quote'\"$", "$(touch SHOULD_NOT_EXIST)"],
      editable: None,
      background: false,
    },
    {
      id: "host_stdin_stream",
      workload: "process",
      prompt: "Run read_stdin.py with python3 and feed it exactly this UTF-8 input through the command's stdin: first line, a blank line, 你好, and last, each separated by LF and with a final LF. The helper must create result.json. Do not modify read_stdin.py or manually write result.json.",
      inputs: {
        "read_stdin.py": "import json,sys\ntext=sys.stdin.read()\nwith open('result.json','w',encoding='utf-8') as f:\n json.dump({'lines':text.splitlines(),'bytes':len(text.encode('utf-8'))},f,ensure_ascii=False)\n",
      },
      expected: { "lines": ["first line", "", "你好", "last"], "bytes": 24 },
      editable: None,
      background: false,
    },
    {
      id: "host_path_inventory",
      workload: "process",
      prompt: "Run inspect_paths.py with python3. It reads manifest.json and must create result.json describing every listed UTF-8 file in manifest order. Do not execute any filename, modify input files, modify inspect_paths.py, or manually write result.json.",
      inputs: {
        "manifest.json": "[\"data/alpha space.txt\",\"data/quote'$.txt\",\"data/nested/utf8.txt\"]",
        "data/alpha space.txt": "first\n\nlast\n",
        "data/quote'$.txt": "literal $HOME; `echo unsafe`\n",
        "data/nested/utf8.txt": "你好\n世界\n",
        "inspect_paths.py": "import json\nwith open('manifest.json',encoding='utf-8') as f: paths=json.load(f)\nout=[]\nfor path in paths:\n with open(path,encoding='utf-8') as f: text=f.read()\n out.append({'path':path,'nonempty_lines':sum(1 for line in text.split('\\n') if line != ''),'text':text})\nwith open('result.json','w',encoding='utf-8') as f: json.dump(out,f,ensure_ascii=False)\n",
      },
      expected: [
        {
          "path": "data/alpha space.txt",
          "nonempty_lines": 2,
          "text": "first\n\nlast\n",
        },
        {
          "path": "data/quote'$.txt",
          "nonempty_lines": 1,
          "text": "literal $HOME; `echo unsafe`\n",
        },
        {
          "path": "data/nested/utf8.txt",
          "nonempty_lines": 2,
          "text": "你好\n世界\n",
        },
      ],
      editable: None,
      background: false,
    },
    {
      id: "host_background",
      workload: "process",
      prompt: "Start worker.py with python3 as a background process and poll it with the available command execution tools until it finishes. It should create result.json after a short delay. Do not modify worker.py or manually write result.json.",
      inputs: {
        "worker.py": "import json,time\nprint('started',flush=True)\ntime.sleep(1.5)\nwith open('result.json','w',encoding='utf-8') as f: json.dump({'state':'complete','steps':3},f)\nprint('complete',flush=True)\n",
      },
      expected: { "state": "complete", "steps": 3 },
      editable: None,
      background: true,
    },
    {
      id: "host_large_output",
      workload: "process",
      prompt: "Run emit_output.py with python3 and wait for it to finish. It intentionally writes a bounded amount of stdout before creating result.json. Verify the result file and do not modify emit_output.py or manually write result.json.",
      inputs: {
        "emit_output.py": "import json,sys\nsys.stdout.write('x'*16384)\nsys.stdout.flush()\nwith open('result.json','w',encoding='utf-8') as f: json.dump({'bytes':16384},f)\n",
      },
      expected: { "bytes": 16384 },
      editable: None,
      background: false,
    },
    {
      id: "host_exit_recovery",
      workload: "process",
      prompt: "Run retry.py with python3 once without arguments and observe its nonzero exit and stderr. Then rerun it with the required --confirm argument. The successful run must create result.json. Do not modify retry.py or manually write result.json.",
      inputs: {
        "retry.py": "import json,sys\nif '--confirm' not in sys.argv[1:]:\n print('confirmation required',file=sys.stderr)\n sys.exit(7)\nwith open('result.json','w',encoding='utf-8') as f: json.dump({'status':'ok'},f)\n",
      },
      expected: { "status": "ok" },
      editable: None,
      background: false,
    },
  ]
}

///|
pub fn correct_output(case : Case, text : String) -> Bool {
  try @json.parse(text) == case.expected catch {
    _ => false
  }
}

///|
pub fn preserved_input(case : Case, path : String, actual : String?) -> Bool {
  guard case.inputs.get(path) is Some(original) && actual is Some(actual) else {
    return false
  }
  if case.editable != Some(path) {
    return original == actual
  }
  let parts = original.split("async fn main {").to_array()
  guard parts.length() == 2 else { return false }
  actual.has_prefix(
    "import { \"moonbitlang/async\", \"moonbitlang/async/fs\" }",
  ) &&
  actual.has_suffix("async fn main {" + parts[1].to_owned())
}

///|
pub fn prompt(case : Case, backend : String) -> String {
  // Shell and transparent runs expose the same Codex tools to the model. The
  // backend difference is applied after command approval, so the prompt must
  // not teach the model a backend-specific scripting language.
  let _ = backend
  let routing = "Use exec_command and write_stdin for command and script execution. You may use apply_patch for editing."
  "\{routing}\nWork only inside the task workspace. Do not inspect other home directories, evaluation infrastructure or answer keys. Do not use the network, delegate to another agent, or ask for human input. Read MOONBIT.md if you need MoonBit syntax. Complete and verify the task before your final response.\n\n\{case.prompt}"
}
