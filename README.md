A Rust implementation of Conway's Game of Life. 3 'Games of Life' are played simultaneously, each represented by a primary colour. 
If living cells overlap, their colours are combined.
NBMETHOD = 0 has no neighbour interations between different colours
NBMETHOD = 1 counts other colours in the same location as neigbours

RULE = 0 default GoL rules: stay alive = 2-3 neighbours, spawn life = 3 neighbours, everything else = death
RULE = 1 GoL rules+1: stay alive = 3-4 neighbours, spawn life = 4 neighbours, everything else = death



To run:
```
cargo run
```
