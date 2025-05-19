import { Cat } from "@/cat";
import { Dog } from "@/dog";

class Main {
  public static main() {
    const gg = new Dog({ name: "Goldenglow", age: 10 });
    gg.show();
    gg.sound();
    console.log(gg.toString());

    const pepe = new Cat({ name: "Pepe", age: 5 });
    pepe.show();
    pepe.sound();
    console.log(pepe.toString());
  }
}

Main.main();
