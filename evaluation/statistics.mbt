///|
fn average(values : Array[Double]) -> Double {
  values.fold(init=0.0, (sum, value) => sum + value) /
  values.length().to_double()
}

///|
fn quantile(
  values : Array[Double],
  numerator : Int,
  denominator : Int,
) -> Double {
  let sorted = values.copy()
  sorted.sort()
  sorted[(sorted.length() - 1) * numerator / denominator]
}

///|
pub fn paired_statistics(rows : Array[Json]) -> Json {
  let shell : Array[Double] = []
  let transparent : Array[Double] = []
  for row in rows {
    if row is { "shell": Number(a, ..), "transparent": Number(b, ..), .. } &&
      a > 0.0 &&
      b > 0.0 {
      shell.push(a)
      transparent.push(b)
    }
  }
  let n = shell.length()
  if n == 0 {
    return {
      "pairs": 0,
      "ratio": Json::null(),
      "ratio_ci95": Json::null(),
      "delta": Json::null(),
      "interpretation": "insufficient_evidence",
      "ratio_ci_upper_at_most_0_90": false,
    }
  }
  let deltas = Array::makei(n, i => transparent[i] - shell[i])
  let ratios : Array[Double] = []
  let delta_means : Array[Double] = []
  let mut seed : UInt = 20260913U
  for _ in 0..<2000 {
    let mut a = 0.0
    let mut b = 0.0
    for _ in 0..<n {
      seed = seed ^ (seed << 13)
      seed = seed ^ (seed >> 17)
      seed = seed ^ (seed << 5)
      let index = (seed % n.reinterpret_as_uint()).reinterpret_as_int()
      a += shell[index]
      b += transparent[index]
    }
    ratios.push(b / a)
    delta_means.push((b - a) / n.to_double())
  }
  let upper = quantile(ratios, 975, 1000)
  let lower = quantile(ratios, 25, 1000)
  {
    "pairs": n.to_json(),
    "shell_mean": average(shell).to_json(),
    "transparent_mean": average(transparent).to_json(),
    "ratio": (average(transparent) / average(shell)).to_json(),
    "ratio_ci95": [lower.to_json(), upper.to_json()],
    "interpretation": (if n < 2 {
      "insufficient_evidence"
    } else if upper < 1.0 {
      "transparent_faster"
    } else if lower > 1.0 {
      "transparent_slower"
    } else {
      "no_clear_difference"
    }).to_json(),
    "delta": average(deltas).to_json(),
    "delta_median": quantile(deltas, 1, 2).to_json(),
    "delta_ci95": [
      quantile(delta_means, 25, 1000).to_json(),
      quantile(delta_means, 975, 1000).to_json(),
    ],
    "transparent_faster": deltas.filter(x => x < 0.0).length().to_json(),
    "ratio_ci_upper_at_most_0_90": (n >= 2 && upper <= 0.90).to_json(),
    "method": "paired bootstrap of ratio of means; percentile interval; 2000 resamples; xorshift32 seed 20260913",
  }
}
