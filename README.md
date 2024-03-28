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

## Supported shells

Here you can find a list of currently supported shells.

- Shell (`sh`)
- Bash (`bash`)
- Z shell (`zsh`)
- Fish (`fish`)
- Nushell (`nu`)
- PowerShell (`powershell`, `pwsh`)
- Cmd (`cmd`)

Are you missing a shell? Feel free to [create an issue](https://github.com/zekroTJA/whattheshell/issues/new) or create a pull request with an implementation.

## Credits

This implementation is very much inspired and influenced by the shell infer implementation in [Schniz/fnm](https://github.com/Schniz/fnm) (see [fnm/src/shell](https://github.com/Schniz/fnm/tree/master/src/shell)).