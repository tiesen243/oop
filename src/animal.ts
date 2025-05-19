export abstract class Animal {
  protected abstract _name: string;
  protected abstract _age: number;

  protected abstract show(): void;
  protected abstract sound(): void;
  protected toString(): string {
    return `Animal[name: ${this._name}, age: ${this._age}]`;
  }
}
