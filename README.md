# September Review

I had a brief detour into Swift, which was fun, but I realized wasn't practical for me (I'll probably blog about it). But, it's been over a month since I played with Rust, and forgot a lot of it. This is going over the chunk of *The Rust Programming Language* I did to this point, to remember what I learned. 

This will probably incorporate snippets of code I previously used.

* `if` statements: what I expect initially
* Mutable variables:
    * I can do `let x=5` then `let x=6`--effectively, a new variable. I can't do `x=7`--`x` is *immutable*.
    * If I do `let mut x=6`, then, it's *mutable*--I can subsequently do `x=7`.