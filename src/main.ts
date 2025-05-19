import { Cat } from "@/cat";
import { Dog } from "@/dog";
import { Zoo } from "@/zoo";

(function main() {
  const zoo = new Zoo("Ac nai", []);
  zoo.add(new Dog("Goldenglow", 20));
  zoo.add(new Cat("Pepe", 19));

  console.log(zoo.toString());
  console.log("Animals in the zoo:");
  zoo.showAnimals();
})();
