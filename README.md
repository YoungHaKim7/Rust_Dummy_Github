# Rust_Dummy_Github



# Source
- Create a Dummy GitHub CLI in Rust! | Ghamza
  - https://youtu.be/pyeUkQg8z9A?si=-WUDmoorTjs75eQ7

# Result


```
Usage: Rust_Dummy_Github.exe <COMMAND>

Commands:
  pr    Manage pull requests
  auth  Login or Logout
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help


// pr 입력시
$ target\debug\Rust_Dummy_Github.exe pr

Manage pull requests

Usage: Rust_Dummy_Github.exe pr <COMMAND>

Commands:
  create  Create a pull request
  list    List pull request
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help 


$ cargo r pr create
  Compiling Rust_Dummy_Github v0.1.0 (D:\Rust_Dummy_Github)
  Running `target\debug\Rust_Dummy_Github.exe pr create`
      
Pr Created 

```
