# Computer V1

This little project is a 2 degrees equation solver.
I did it to start learning rust.
- can solve eqaution from 0 to 2 degrees.
- give the complexe results of second degreee equation with delta < 0
- can solve "human write" expressions


## Run it
- make && ./computor "__\_\_your_equation\_\___"

## Run tests
- make tests

#### No externe mathematical library.
only use:
f64.sqrt() and f64.powi() which is are processor call instructions (llvm, not a math library)
- https://doc.rust-lang.org/src/std/f64.rs.html#339-345
- https://doc.rust-lang.org/src/std/f64.rs.html#302-304
- https://doc.rust-lang.org/core/intrinsics/index.html
- https://doc.rust-lang.org/std/intrinsics/index.html
