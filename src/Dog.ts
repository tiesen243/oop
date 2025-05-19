import type { Animal } from "@/Animal";

export class Dog implements Animal {
  private _name: string;
  private _age: number;

  constructor(parameters: { name: string; age: number }) {
    this._name = parameters.name;
    this._age = parameters.age;
  }

  get name(): string {
    return this._name;
  }

  set name(name: string) {
    this._name = name;
  }

  get age(): number {
    return this._age;
  }

  set age(age: number) {
    this._age = age;
  }

  public show(): void;
  public show(name: string): void;
  public show(name?: string): void {
    if (name) console.log(`Hi, i'm ${name} and i'm ${this.age} yo!`);
    else console.log(`Hi, i'm ${this.name} and i'm ${this.age} yo!`);
  }

  public bark(): void {
    console.log("Woof!");
  }
}
