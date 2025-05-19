# OOP

## UML

```mermaid
classdiagram
    animal <|-- dog
    animal <|-- cat
    zoo o-- animal

    class animal {
        -name: string
        -age: number
        +show() void
        +sound() void
    }

    class dog {
        +sound() void
    }

    class cat {
        +sound() void
    }

    class zoo {
        -name: string
        -animals: animal[]
        +showanimals() void
    }
```
