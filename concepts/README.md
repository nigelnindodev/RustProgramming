### Variables & Mutability

- Of course, variables are immutable by default, and you can make them imutaley adding the `mut` keyword.

### Constants

- You can also declare constants, which are always immutable.

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

- The type of a constant MUST always be annotated.
- Constants can be declared in any scope, including the global scope.

### Shadowing

- You can declare a new variable with the same name as the previous variable as long as it's within an inner scope.
- Shadowing is different from marking a variable as `mut` because we'll get a compile time error by trying
to reasssign the variable without using the let keyword, this could happen accidentally.
