# Playground

This project contains several blank projects ready to write and execute. It is useful when you want to test some functions but don't want to set up an entire project. They also can be used as a starting template.

## Download a specific playground

To only download a subdirectory use these commands.

```sh
git clone --no-checkout --sparse --filter=blob:none --depth 1 https://github.com/otcova/playground.git
cd playground
git sparse-checkout init --cone
git sparse-checkout set c++ wasm # list of sub-folders to checkout
git checkout
```