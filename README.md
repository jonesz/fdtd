FDTD electromagnetic simulator; referenced from
[Schneider's book](https://eecs.wsu.edu/~schneidj/ufdtd/ufdtd.pdf).

No guarantees anything is correct (though 1D seems to be); more of a project
to play with Futhark.

The general design is to allow the programmer to define closures to be run
after the magnetic/fields are updated. One could place their tfsf/boundary
logic within those functions. Examples within `/src/bin`.
