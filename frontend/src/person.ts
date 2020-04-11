export function what(): string {
    return "Rust"
}

export class Person {
    _name: string;
    age: number;
    work: Work;
    constructor(name: string) {
        this._name = name;
        this.age = 5;
        this.work = {
            company: 'Proemion',
            salary: 123456
        };
    }
    get name(): string {
        return this._name;
    }
    set name(n: string) {
        this._name = n;
    }
    talk() {
        return `My name is ${this.name} and my age is ${this.age}`;
    }
}


export interface Work {
    salary: number;
    company: string;
}