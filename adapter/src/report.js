const report = JSON.parse(document.getElementById('report').textContent);
const pairs = report.pairs || [];
const arms = report.arms || [];
const byId = id => document.getElementById(id);
const pretty = value => JSON.stringify(value, null, 2);
const node = (tag, text) => {
  const element = document.createElement(tag);
  if (text != null) element.textContent = text;
  return element;
};
const shown = value => value == null ? 'unknown' : typeof value === 'number' ? Number(value.toPrecision(5)).toString() : String(value);
const summary = report.summary;
byId('totals').textContent = report.manifest.suite === 'launcher-startup'
  ? `Platform: ${shown(report.manifest.platform)} | Failed arms: ${summary.failed_arms}\nMeasured pairs per workload and observation: ${summary.formal_pairs_per_stratum}`
  : `Platform: ${shown(summary.platform)} | Strict pairs: ${summary.strict_comparable_pairs}/${shown(summary.target_pairs)} | ${summary.partial ? 'Partial' : 'Target reached'}\nITT arm statuses: ${pretty(summary.intention_to_treat.arm_statuses)}`;
byId('totals').textContent += `\nRun purpose: ${shown(summary.purpose)}`;
byId('limits').textContent = pretty(summary.limits || []);
for (const [id, label, values] of [
  ['task', 'All scenarios', pairs.map(p => p.task_id)],
  ['category', 'All categories', pairs.map(p => p.category)],
  ['platform', 'All platforms', pairs.map(p => p.platform)],
  ['failure', 'All arm statuses', arms.map(a => a.result.status)],
]) {
  for (const value of ['', ...new Set(values.filter(v => v != null))]) {
    const option = node('option', value || label);
    option.value = value;
    byId(id).append(option);
  }
}

function timing(arm) {
  const rows = arm.trace.filter(e => e.clock_domain === 'os-monotonic' && Number.isFinite(e.monotonic_ns));
  const start = arm.result.begin_ns ?? rows[0]?.monotonic_ns;
  const end = arm.result.end_ns ?? rows[rows.length - 1]?.monotonic_ns;
  return { rows, start, duration: start == null || end == null ? null : end - start };
}

function timeline(parent, arm, scale) {
  parent.append(node('h2', `${arm.result.backend} / ${arm.result.status}`));
  const canvas = node('canvas');
  canvas.width = 1000;
  canvas.height = 220;
  canvas.setAttribute('aria-label', `${arm.result.backend} observed events`);
  parent.append(canvas);
  requestAnimationFrame(() => {
  const width = Math.max(280, canvas.getBoundingClientRect().width);
  const pixelRatio = window.devicePixelRatio || 1;
  canvas.width = Math.ceil(width * pixelRatio);
  canvas.height = 220 * pixelRatio;
  const ctx = canvas.getContext('2d');
  ctx.scale(pixelRatio, pixelRatio);
  const left = width < 500 ? 142 : 175;
  const right = width - 5;
  const { rows, start } = timing(arm);
  if (!rows.length || start == null) { ctx.fillText('unknown', 10, 30); return; }
  const phases = ['external', 'tool', 'approval', 'sandbox_setup_spawn', 'spawn', 'launcher', 'child', 'io', report.manifest.suite === 'launcher-startup' ? 'startup' : 'codex'];
  ctx.font = width < 500 ? '11px system-ui' : '13px system-ui';
  phases.forEach((phase, i) => {
    ctx.fillStyle = '#555';
    ctx.fillText(phase, 4, 14 + i * 21);
    ctx.strokeStyle = '#ddd';
    ctx.beginPath(); ctx.moveTo(left, 10 + i * 21); ctx.lineTo(right, 10 + i * 21); ctx.stroke();
  });
  for (const event of rows) {
    const i = phases.indexOf(event.phase);
    if (i < 0 || event.monotonic_ns < start) continue;
    ctx.fillStyle = arm.result.backend === 'shell' ? '#167750' : '#c2444f';
    ctx.fillRect(left + (right - left - 3) * (event.monotonic_ns - start) / Math.max(1, scale), 4 + i * 21, 3, 14);
  }
  ctx.fillStyle = '#444';
  ctx.fillText('0 ms', left, 211);
  ctx.textAlign = 'right'; ctx.fillText(`${shown(scale / 1e6)} ms`, right, 211);
  });
  const { rows } = timing(arm);
  const detail = node('details');
  detail.append(node('summary', 'Observed events and durations'));
  detail.addEventListener('toggle', () => {
    if (detail.open && !detail.querySelector('pre')) detail.append(node('pre', pretty({ metrics: arm.metrics, events: rows })));
  });
  parent.append(detail);
}

function draw() {
  byId('pairs').replaceChildren();
  for (const pair of pairs) {
    if (['task', 'category', 'platform'].some(id => byId(id).value && byId(id).value !== pair[id === 'task' ? 'task_id' : id])) continue;
    if (byId('subset').value === 'strict' && !pair.strict_comparable) continue;
    if (byId('subset').value === 'failed' && pair.strict_comparable) continue;
    const members = ['shell', 'transparent'].map(key => arms[pair[key]]).filter(Boolean);
    if (byId('failure').value && !members.some(a => a.result.status === byId('failure').value)) continue;
    const section = node('section');
    const detail = node('details');
    detail.append(node('summary', `${pair.pair_id} | ${pair.strict_comparable ? 'strict comparable' : 'non-comparable'}`));
    detail.addEventListener('toggle', () => {
      if (!detail.open || detail.querySelector('.timeline')) return;
      const split = node('div'); split.className = 'timeline';
      const scale = Math.max(1, ...members.map(a => timing(a).duration || 0));
      for (const key of ['shell', 'transparent']) {
        const part = node('div');
        const arm = arms[pair[key]];
        if (!arm) { part.append(node('p', `${key}: censored`)); split.append(part); continue; }
        timeline(part, arm, scale);
        if (/^attempts\/[A-Za-z0-9_-]+$/.test(arm.result.artifact_dir)) {
          const link = node('a', 'Raw evidence');
          const base = /^(\.\.\/)*\.?$/.test(report.evidence_base || '.') ? report.evidence_base || '.' : '.';
          link.href = `${base}/${arm.result.artifact_dir}/result.json`;
          part.append(link);
        }
        const raw = node('details');
        raw.append(node('summary', 'Arguments, results and hashes'));
        raw.addEventListener('toggle', () => {
          if (raw.open && !raw.querySelector('pre')) raw.append(node('pre', pretty({ result: arm.result, facts: arm.facts, seal: arm.seal })));
        });
        part.append(raw); split.append(part);
      }
      detail.append(split, node('pre', pretty({ first_difference: pair.first_difference })));
    });
    section.append(detail); byId('pairs').append(section);
  }
  const first = byId('pairs').querySelector('details');
  if (first) first.open = true;
}

const table = node('table');
const head = node('tr');
for (const label of ['Stratum', 'Metric', 'Pairs', 'MBTX/Shell', '95% ratio interval', 'Delta (ms)', 'Evidence']) head.append(node('th', label));
table.append(head);
for (const row of report.statistics || []) {
  const tr = node('tr');
  const stats = row.statistics;
  for (const value of [row.stratum, row.metric, stats.pairs, stats.ratio, pretty(stats.ratio_ci95), stats.delta == null ? null : stats.delta / 1e6, row.confidence]) tr.append(node('td', shown(value)));
  table.append(tr);
}
byId('statistics').append(table);
for (const id of ['task', 'category', 'platform', 'failure', 'subset']) byId(id).onchange = draw;
draw();
