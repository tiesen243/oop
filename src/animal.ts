export abstract class Animal {
  protected abstract _name: string;
  protected abstract _age: number;

  public abstract show(): void;
  public abstract sound(): void;
  public toString(): string {
    return `Animal[name=${this._name}, age=${this._age}]`;
  }
}
