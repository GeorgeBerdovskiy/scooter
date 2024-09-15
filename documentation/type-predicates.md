# Type Predicates
Rust has a very powerful trait system, and some programmers are tempted to use traits as predicates. Unfortunately, because the Rust compiler doesn't treat traits as predicates, this often fails.

For example, let's say you wish to define a recursive predicate for a list of types. This "list" consists of a head type, which is _not_ a tuple, and a tail type, which _is_ a tuple. In other words, something like this -

```
(A, ())
(A, (B, ()))
(A, (B, (C, ())))
...
```

The unit type indicates the end of the list. Suppose you wanted to define a predicate called `Contains<A>`, which is satisfied if the type `A` exists in the list. My first attempt involved traits, and looked like this.

```rs
trait Contains<A> {}

impl<A> Contains<A> for (A, ()) {}

impl<A, H, T: Contains<A>> Contains<A> for (H, T) {}
```

However, anyone familiar with Rust will quickly realize this is not allowed due to **conflicting implementations.** Sometimes you can get around this by using associated types, but it's pretty tricky.

One of my goals with Scooter is to implement a Prolog-esque **type predicate** system compatible with Rust's trait system. That might look something like this.

```
predicate Contains<A> {
    satisfied by (A, _);
    satisfied by (B, C) when
        C: Contains<A>;
}
```

The predicate `Contains<A>` is satisfied by...

* Any pair with `A` as the first element, and
* Any pair where the first element isn't `A` but the second element `C` satisfies `Contains<A>`
