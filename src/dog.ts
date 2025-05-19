import { Animal } from "@/animal";

export interface Dog extends Animal {}

export class Dog implements Dog {
  private _name: string;
  private _age: number;

  constructor(parameters: { name: string; age: number }) {
    this._name = parameters.name;
    this._age = parameters.age;
  }

  public show(): void {
    console.log(`Hi, i'm ${this._name} and i'm ${this._age} yo!`);
  }

  public sound(): void {
    console.log("Woof!");
  }

  public toString(): string {
    return `Dog { name: ${this._name}, age: ${this._age} }`;
  }
}
