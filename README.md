<div align="center">
  <picture>
    <img src="documentation/images/ScooterLight.png" style="width: 250px; max-width: 100%"/>
  </picture>
</div>

## What is Scooter?
Scooter is a toy programming language built for the purpose of **learning compiler design and implementation**. Reading books and papers will provide you with the required _theory_, but that theory is only useful when _applied_ to a real programming language.

Unfortunately, real programming languages are complicated. Naturally, so are their compilers. It certainly doesn't help that many compilers are worked on by large teams of contributors over the course of years (or, in the case of older languages, decades).

That's where Scooter comes in. Its source code doesn't feature many optimizations or extravagant abstractions, so it should be approachable to newer programmers. On the other hand, this means we ignore many best practices. If you feel something could be written better, you're probably right!

## About Scooter
Scooter's syntax is inspired by Rust. In fact, most Scooter code is valid Rust code. However, Scooter is exceedingly simple and doesn't do much. As of 09/14/2024, it only supports `i32` values and simple arithmetic operations.

## Example
Here is a basic Scooter program.

```
fn foo() -> i32 {
    let a: i32 = 5 + 5;
    let b: i32 = a + 5 * 7;
    return b + a;
}
```

### Intermediate Representation
Scooter source code is lowered to a three address code called **Wheel IR**. Future languages in this family will also use this intermediate representation. Note that there is no optimization!

```
L0: t0 = 5
    t1 = 5
    t2 = t0 + t1
    x0 = t2
    t3 = x0
    t4 = 5
    t5 = 7
    t6 = t4 * t5
    t7 = t3 + t6
    x1 = t7
    t8 = x1
    t9 = x0
    t10 = t8 + t9
    ret t10
```
