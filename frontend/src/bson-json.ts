import { memory } from 'wasm-rust-js/wasm_rust_js_bg.d';
import {EJSON, serialize, deserialize} from 'bson'
import axios from 'axios';
export class BsonJson {
    constructor() {
        // // Serialize a document
        const doc = { long: 100 };
        console.log(doc);
        const data = serialize(doc);
        console.log('data:', data);

        // // De serialize it again
        // const doc_2 = deserialize(data);
        // console.log('doc_2:', doc_2);

        this.bsonSerde();
    }

    public async bsonSerde(): Promise<void> {
        let resp =  await this.getSampleData();
        console.log(Array.isArray(resp.data.data))
        const data = serialize(resp.data.data.map(x => ({...x, deii: ['yes', {a: 1,}, false, 11, undefined, null]})));
        console.log('data:', data);
         // De serialize it again
         const doc_2 = deserialize(data, {promoteValues: true});
         console.log('doc_2:', Object.keys(doc_2), Array.isArray(doc_2));

         console.log(EJSON.serialize(data))
         console.log('hmm', EJSON.deserialize(resp.data))
        //  console.log(new Uint8Array(memory.buffer))
    }

    public getSampleData() {
        return axios.get('https://reqres.in/api/users');
    }
}