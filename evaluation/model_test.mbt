///|
test "relay errors remain distinct from backend failures" {
  assert_false(comparable(Outcome::RelayError, Outcome::Success))
  assert_false(comparable(Outcome::BackendFailure, Outcome::Success))
  assert_true(comparable(Outcome::Success, Outcome::Success))
}
