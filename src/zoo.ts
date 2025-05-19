import type { Animal } from "@/animal";

export class Zoo {
  private _name: string;
  private _animals: Animal[];

  constructor(name: string, animals: Animal[]) {
    this._name = name;
    this._animals = animals;
  }

  public add(animal: Animal): void {
    this._animals.push(animal);
  }

  public showAnimals(): void {
    this._animals.map((animal) => console.log(animal.toString()));
  }

  public toString(): string {
    return `Zoo[name=${this._name}, animals=${this._animals.length}]`;
  }
}
