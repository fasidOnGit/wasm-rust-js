import { BsonJson } from './bson-json';
import * as person from "./person";
import {ChartCounter} from './chart-counter';
import * as wasm from 'wasm-rust-js';
import {Snow} from './snow';
export class WasmWeb {
  
  private aString = 'A String';
  private aMethod(): string {
    return this.aString;
  }

  public async startWasm(): Promise<void> {
    const aClosure = () => {
      return this.aMethod();
    };

    // const wasm = await import("wasm-rust-js");
    new ChartCounter();
    console.log('hemem')
    wasm.greet();
    wasm.web_sys_dom();
    wasm.web_sys_closures();
    wasm.performance();
    wasm.performance_now_greet();
    let what = wasm.take_a_closure(this.aMethod.bind(this));
    console.log('asasasas', what);
    // console.log(await wasm.fetch_gh_branch());
    // Skipping all of canvas.. revisit if needed!
    // Also skipping WebGL, WEBAudio, WebSockets!
    // And everything after that...
    //TODO MVC sounds interesting.. but need to check it out later
    interface Person {
      name: string;
      id: number;
      parent_id: string,
      some: {
        name?: string
      }
    }
    const arr: Person[] = [{
      name: 'kader',
      id: 1,
      parent_id: '2',
      some: {
      }
    }];
    console.log('Are you there? ', wasm.mighty_fn(arr));
    console.log(
      wasm.try_undefined({
        a: undefined,
        b: 1.11
      })
    );
    console.log(wasm.data_filter({
      offset: 0,
      limit: 10,
      filter: 'wh',
      sort_property: 'name',
      sort_dir: 'asc',
      students: [{
        name: 'whet',
        class: 'kg',
        subject: {name: 'English', key: 'English'}
      },{
        name: 'asadawh',
        class: 'kg',
        subject: {name: 'English', key: 'English'}
      },{
        name: 'what',
        class: 'lkg',
        subject: {name: 'Tamil', key: 'Tamil'}
      }]
    }))


    //bson
    new BsonJson();
    new Snow();
  }
}
