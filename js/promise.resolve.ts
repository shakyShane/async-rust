import {from, of} from "rxjs";

const p = Promise.resolve([1, 2, 3]);
p.then(function (v) {
  console.log('Promise.resolve')
  console.log(v); // 1
});

console.log('Observable.of')
const obs = of([1, 2, 3]);
obs.forEach(x => {
  console.log(x);
})
