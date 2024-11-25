build geet

```
cargo build
export PATH=$PATH:$(pwd)/target/debug
```

create a test repo at ./test and cd into it

```
mkdir test
cd test
```

to init a new repo

```
cargo run -- init test
```

to add (stage) the file

```
cargo run -- add test.txt
```

to make a new commit

```
cargo run -- commit -m "first one"
```

to check status

```
cargo run -- status
```

to checkout a previous commit using its hash

```
cargo run -- checkout a145d0486463ceb2840f5c871608f142b713736f
```

to clean up the repository and try these commands again

```
cargo run -- cleanup
```

# todo

- right now we can make identical commits multiple times. Check if the tree_hash is the same, if it is don't create a new commit
- right now we can re-init a repository, disable that
- refactor cli parser error handling using ?
- refactor command handling code to be a group of helper functions, no more enums
- when we make a new commit, we only change HEAD to point to it. We need to make the current branch to point to it as well

# known bugs

- add/remove treats path with "./" prefix as unique from the same path without it
- status command treats empty files the same, because empty content gets hashed to the same value
