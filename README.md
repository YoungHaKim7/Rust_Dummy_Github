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

- final Result

```

$ cargo r pr create --title Hello

   Compiling Rust_Dummy_Github v0.1.0 (D:\Rust_Dummy_Github)
warning: crate `Rust_Dummy_Github` should have a snake case name
warning: `Rust_Dummy_Github` (bin "Rust_Dummy_Github") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 1.02s
     Running `target\debug\Rust_Dummy_Github.exe pr create --title Hello`
    
Pr with title Hello is created and the fraft status is: false



// short command ver.

$ cargo r pr create -t Hello -d

Pr with title Hello is created and
 the draft status is: true

```
