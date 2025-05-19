import { Animal } from "@/animal";

export class Dog extends Animal {
  protected _name: string;
  protected _age: number;

  constructor(name: string, age: number) {
    super();
    this._name = name;
    this._age = age;
  }

  public show(): void {
    console.log(`Hi, i'm ${this._name} and i'm ${this._age} yo!`);
  }

  public sound(): void {
    console.log("Woof!");
  }

  public toString(): string {
    return `Dog[name=${this._name}, age=${this._age}]`;
  }
}
