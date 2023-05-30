# termstatus

*termstatus* is a tool that lets you inform the user the progress of your program
by displaying the status. It looks similar to the status texts displayed by
[rustc](https://github.com/rust-lang/rust) or
[cargo](https://github.com/rust-lang/cargo).


## Installation

Simply use `$ cargo add termstatus` or add the following dependency to your
Cargo.toml:

```toml
[dependencies]
termstatus = "0.2"
```


## Usage

Create an `enum` whose variants are the different status labels and derive it
from `TermStatus`:

```rust
extern crate termstatus;
use termstatus::TermStatus;

#[allow(dead_code)]
#[derive(TermStatus)]
enum Status {
    Building,
    Built,
    Compiled,
    Compiling,
    #[style(red, bold)]
    Error,
    Finished,
    Running,
}
```

[`Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html) is automatically
implemented for that `enum` and can therefore be used with
[`println`](https://doc.rust-lang.org/std/macro.println.html) or
[`format`](https://doc.rust-lang.org/std/macro.format.html) for example:

```rust
println!("{} foo", Status::Building);
println!("{} foo", Status::Built);
println!("{} bar", Status::Compiling);
println!("{} bar", Status::Compiled);
println!("{} build of project", Status::Finished);
```

The result will look as follows:

```text
 Building foo
    Built foo
Compiling bar
 Compiled bar
 Finished build of project
```

## Planned Features

- use of `#[default_style(on_cyan, italic)]` on the derived `enum`
- support of bright colors


## TODO

- implement [planned features](#planned-features)
- include GIF demo in README
