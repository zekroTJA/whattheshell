# whattheshell

This crate tries to provide a simple solution for a simple problem: "In what shell am I running?"

With `Shell::infer()`, the currently used shell is tried to be inferred by inspecting the given environment.

On unix-systems, the output of `ps -o ppid,comm {pid}` is inspected to get the process' parent process which is followed down the tree until a shell process was found. On non-unix system, the same algorithm is used but by the help of the crate [`sysinfo`](https://crates.io/crates/sysinfo).

## Example

```rust
use whattheshell::Shell;

fn main() {
    let shell = Shell::infer().unwrap();
    println!("{shell}"); // -> "zsh"
}
```

## Credits

This implementation is very much inspired and influenced by the shell infer implementation in [Schniz/fnm](https://github.com/Schniz/fnm) (see [fnm/src/shell](https://github.com/Schniz/fnm/tree/master/src/shell)).