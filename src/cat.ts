import { Animal } from "@/animal";

export class Cat extends Animal {
  protected _name: string;
  protected _age: number;

  constructor(name: string, age: number) {
    super();
    this._name = name;
    this._age = age;
  }

  override show(): string {
    console.log(`Hi, i'm ${this._name} and i'm ${this._age} yo!`);
    return "a";
  }

  override sound(): void {
    console.log("Meow!");
  }

  override toString(): string {
    return `Cat[name=${this._name}, age=${this._age}]`;
  }
}
