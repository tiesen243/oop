import { Animal } from "@/animal";

export class Cat extends Animal {
  protected _name: string;
  protected _age: number;

  constructor(parameters: { name: string; age: number }) {
    super();
    this._name = parameters.name;
    this._age = parameters.age;
  }

  public show(): string {
    console.log(`Hi, i'm ${this._name} and i'm ${this._age} yo!`);
    return "a";
  }

  public sound(): void {
    console.log("Meow!");
  }

  public toString(): string {
    return `Cat[name: ${this._name}, age: ${this._age}]`;
  }
}
