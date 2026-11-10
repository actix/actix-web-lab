import {readFileSync, writeFileSync} from 'node:fs';
const rows = readFileSync(new URL('./raw.csv', import.meta.url), 'utf8').trim().split('\n').slice(1).map(line => {
  const [round, variant, name, count, ns] = line.split(',');
  return {round: +round, variant, name, ns: +ns / +count};
});
let seed = 3924;
const random = () => ((seed = (Math.imul(seed, 1664525) + 1013904223) >>> 0) / 2 ** 32);
const mean = values => values.reduce((a, b) => a + b, 0) / values.length;
const summary = [...new Set(rows.map(row => row.name))].map(name => {
  const before = rows.filter(row => row.name === name && row.variant === 'before');
  const after = rows.filter(row => row.name === name && row.variant === 'after');
  if(before.length !== 8 || after.length !== 8) throw Error('Expected eight complete pairs');
  const logs = before.map(row => Math.log(row.ns / after.find(other => other.round === row.round).ns));
  const gain = logs => 100 * (Math.exp(mean(logs)) - 1);
  const bootstrap = Array.from({length: 10000}, () => gain(logs.map(() => logs[Math.floor(random() * logs.length)]))).sort((a,b) => a-b);
  return {name, before_ns: mean(before.map(row=>row.ns)), after_ns: mean(after.map(row=>row.ns)), speed_gain_percent: gain(logs), low: bootstrap[250], high: bootstrap[9750]};
});
writeFileSync(new URL('./summary.json', import.meta.url), JSON.stringify(summary, null, 2)+'\n');
console.table(summary);
