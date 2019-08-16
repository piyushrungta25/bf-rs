# bf-rs
Optimizing Brainf*** interpreter written in Rust

- Unlimited number of memory cells are available
- Memory cell increments and decrements wrap around

## Usage

You need Rust and Cargo installed to build this. Clone this repository and build.

```bash
$ cargo build --release
```

Run any program

```bash
$ target/release/bfrs samples/mandelbrot.bf
```

or simply

```bash
$ cargo run --release samples/mandelbrot.bf
```

### Interactive Mode

You can also run the interpretor in interactive mode.

- `dump` command will dump the current memory state in a `hd` like format
- `reset` command will reset the internal memory state


```bash
$ cargo run --release
>>> dump
-----------
Memory Dump
-----------
00000000  00                                                |.               |
00000010

>>> [-][Following will print capital 'A' followed by a new line character]
>>> ++++++++[>++++++++<-]>+.>++++++++++.
A

>>> dump
-----------
Memory Dump
-----------
00000000  00 41 0a                                          |.A.             |
00000010

>>> [-][use 'reset' command to reset memory state]
>>> reset

>>> dump
-----------
Memory Dump
-----------
00000000  00                                                |.               |
00000010
>>> 
```

## Optimizations

The interpretor makes some optimizations inspired by the excellent [bfc](https://github.com/Wilfred/bfc#optimisations) project.

1. Combine successive additions and subtractions
   i.e. `+++-` is done in a single step as `increment 2`

2. Combine successive memory cell pointer increment and decrement
   i.e. `>>><` is done in a single step as `shift right twice`

3. The biggest performance gain over the naive implementation was caching the loop start and end index so that input file is scanned only once.


