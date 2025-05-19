import type { Animal } from "./Animal";
import { Dog } from "./Dog";

class Main {
  public static main() {
    const shiba: Animal = new Dog({ name: "Shabi", age: 10 });
    shiba.show();
  }
}

Main.main();
