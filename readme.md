create a test repo at ./test and cd into it

```
cargo run -- init test
```

```
cargo run -- commit -m "first one"
```

# todo

- right now we can make identical commits multiple times. Check if the tree_hash is the same, if it is don't create a new commit
- right now we can re-init a repository, disable that
- refactor cli parser error handling using ?
- refactor command handling code to be a group of helper functions, no more enums
- when we make a new commit, we only change HEAD to point to it. We need to make the current branch to point to it as well
