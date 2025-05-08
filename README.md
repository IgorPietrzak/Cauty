# Cauty - Coloured Nauty.

An algorithm that counts the orbits of graph colourings by using Nauty [https://pallini.di.uniroma1.it/](https://pallini.di.uniroma1.it/) to compute canonical labellings.

## What are Colourings?

This algorithm is concerned with binary colourings. We represent colourings as vectors. Since we are considering binary colourings the vectors will only consist of 1s and 0s. For example `[0,1,0]` tells us that 
vertex 0  has colour 0, vertex 1 has colour 1 and vertex 2 has colour 0. When a vertex has colour 1 we say it is infected.

## What are Orbits?

This algorithm is counting orbit representatives of binary colourings of graphs. Let `G` be a graph. We say a colouring `A` is in the orbit of `B` if there is a `g` in the Automorphism group `Aut(G)`
of `G` such that `g.A = B`. The algorithm collects 1 element from each orbit. This element is known as the orbit representative.

## Technical Overview

The Nauty package available at [https://pallini.di.uniroma1.it/](https://pallini.di.uniroma1.it/) is written in C. Cauty is written in Rust using foreign function interfaces (FFI) to embed the Nauty C code. An example build for an x86_64 system is included but Nauty will need to be recompiled for other CPU architectures. To setup Nauty to work with Cauty run:

```bash
cd nauty2_8_9
make clean
./configure --enable-wordsize=64
make CFLAGS="-g -Wall"
ar rcs libnauty.a nauty.o nautil.o naugraph.o naurng.o schreier.o naututil.o
```

### Notes
- Ensure `build.rs` points to the correct location of the compiled Nauty library.
- Keep the compiled `libnauty.a` in the main crate directory, as demonstrated in this repository.


