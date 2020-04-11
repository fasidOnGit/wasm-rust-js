import * as wasm from "wasm-rust-js";
import {chars} from './charts-list';
export class ChartCounter {
  counters: wasm.Counter[] = [];
  constructor() {
    console.log(wasm);
    this.addCounter();
    let b = document.getElementById('add-counter');
    if (!b) throw new Error('Unable to find #add-counter');
    b.addEventListener('click', ev => this.addCounter());
  }
  addCounter() {
    let ctr = wasm.Counter.new(this.randomChar(), 0);
    this.counters.push(ctr);
    this.update();
  }
  update() {
    let container = document.getElementById('container');
    if (!container) throw new Error('Unable to find #container');
    while (container.hasChildNodes()) {
      if ((container.lastChild as any).id == 'add-counter') break;
      container.removeChild(container.lastChild!);
    }
    this.counters.forEach((counter, index) => {
      container!.appendChild(this.newCounter(counter.key(), counter.count(), ev => {
        counter.increment();
        this.update();
      }))
    })
  }
  randomChar() {
    console.log('randomChar');
    let idx = Math.floor(Math.random() * (chars.length - 1));
    console.log('index', idx);
    let ret = chars.splice(idx, 1)[0];
    console.log('char', ret);
    return ret;
  }
  newCounter(key, value, cb): HTMLDivElement {
    let container = document.createElement('div');
    container.setAttribute('class', 'counter');
    let title = document.createElement('h1');
    title.appendChild(document.createTextNode('Counter ' + key));
    container.appendChild(title);
    let plus = document.createElement('button');
    plus.setAttribute('class', 'plus-button');
    plus.appendChild(document.createTextNode('+'));
    plus.addEventListener('click', cb);
    container.appendChild(plus);
    return container;
  }
  newField(key, value) {
    let ret = document.createElement('div');
    ret.setAttribute('class', 'field');
    let name = document.createElement('span');
    name.setAttribute('class', 'name');
    name.appendChild(document.createTextNode(key));
    ret.appendChild(name);
    let val = document.createElement('span');
    val.setAttribute('class', 'value');
    val.appendChild(document.createTextNode(value));
    ret.appendChild(val);
    return ret;
  }
}
