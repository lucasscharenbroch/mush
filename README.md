# mush

![Mush Logo](logo.png)

A minimalist git clone

The basic plumbing is almost identical to that of `git`, modulo several incompatible differences (some of these are labeled with `//<` in the code).

## Subcommands
- `mush init`
- `mush hash-object`
- `mush cat-file`
- `mush update-index`
- `mush write-tree`
- `mush commit-tree`
- `mush config`

## To Do
- [X] set up cli argparse
- [X] init
- [X] hash-object
- [X] cat-file
- [X] update-index
- [X] write-tree
- [X] commit-tree
- [X] objects (creation, hashing, compression)
    - [X] blobs
    - [X] trees
    - [X] commits
- [ ] refs
    - [ ] heads
    - [ ] tags
    - [ ] update-ref (update the reflog, set contents of heads/refs/...)
- [X] index
- [ ] status / add
- [ ] log
- [ ] commit
- [ ] checkout / switch
- [ ] diff
- [ ] merge
- [ ] (maybe)
    - [ ] remotes
    - [ ] blame
