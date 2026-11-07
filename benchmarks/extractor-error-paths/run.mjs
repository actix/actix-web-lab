import {spawnSync} from 'node:child_process';
import {appendFileSync,writeFileSync} from 'node:fs';
const output = process.argv[2];
writeFileSync(output, 'round,variant,case,count,ns\n');
for(let round=0;round<8;round++) {
 for(const kind of ['query','json','form']) for(const mode of ['success','early','late']) {
  for(const variant of round%2 ? ['after','before'] : ['before','after']) {
   const r=spawnSync(`/tmp/extractor-${variant}`, [`${kind}-${mode}`],{encoding:'utf8'});
   if(r.status!==0) throw Error(r.stderr);
   appendFileSync(output, `${round},${variant},${r.stdout.trim()}\n`);
  }
 }
 console.log(`round ${round+1}/8`);
}
