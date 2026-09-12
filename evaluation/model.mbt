///|
pub(all) enum Outcome {
  Success
  BackendFailure
  RelayError
  ProviderError
  HarnessError
  Timeout
  Censored
  Unknown
} derive(Debug, Eq)

///|
/// A pair is comparable only when both arms reached a complete observable end.
pub fn comparable(left : Outcome, right : Outcome) -> Bool {
  left == Outcome::Success && right == Outcome::Success
}

///|
pub(all) struct PairResult {
  pair_id : String
  task_id : String
  shell : Outcome
  transparent : Outcome
}

///|
pub fn PairResult::is_comparable(self : PairResult) -> Bool {
  comparable(self.shell, self.transparent)
}

///|
/// Keep planned, attempted, and complete counts separate in every report.
pub fn count_comparable(results : Array[PairResult]) -> Int {
  results.filter(r => r.is_comparable()).length()
}
