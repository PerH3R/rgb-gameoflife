A Rust implementation of Conway's Game of Life. 3 'Games of Life' are played simultaneously, each represented by a primary colour. 
If living cells overlap, their colours are combined.
NBMETHOD = 0 has no mechanical interations between colours
NBMETHOD = 1 counts other colours in the same location as neigbours (live, die and spawn rules unchanged)

To run:
```
cargo run
```
