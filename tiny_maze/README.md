﻿# tiny-maze

Alice found herself very tiny and wandering around Wonderland.  Even
the grass around her seemed like a maze.

![alice tiny](/images/alicetiny.gif)

This is a tiny maze solver.

A maze is represented by a matrix (in Rust, a Vector of enums)

```
[[S 0 1]
 [1 0 1]
 [1 0 E]]
```

- _S_ : start of the maze
- _E_ : end of the maze
- _1_ : This is a wall that you cannot pass through
- _0_ : A free space that you can move through.

The goal is the get to the end of the maze.  A solved maze will have a
_:x_ in the start, the path, and the end of the maze, like this.

```
[[x x 1]
 [1 x 1]
 [1 x x]]
```


## Instructions

- Clone or fork this repo
- `cd tiny_maze`
- Run the tests with `cargo test`
- Make the tests pass!

## Solutions

Once you have your kata solution, you are welcome to submit a link to your repo to share here in this section with others.

If you haven't solved your kata yet - Don't Peek!

## License

Copyright © 2015 Stephen Bastians

Copyright © 2014 Carin Meier

Distributed under the Eclipse Public License either version 1.0 or (at
your option) any later version.

