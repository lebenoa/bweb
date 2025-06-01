# Be Website

A simple CLI tool to serve a directory as static web server

## Building
```nushell
cargo build --release
```

## Usage

```pwsh
Usage: bweb.exe [OPTIONS] <DIR>

Arguments:
  <DIR>  The directory to serve

Options:
  -a, --address <ADDRESS>  Address to bind to [default: 127.0.0.1:8998]
  -h, --help               Print help
```

## Example

```pwsh
bweb -a 0.0.0.0:1234 ./dist
```

`http://127.0.0.1:1234` will read `index.html` file in directory `dist`  
`http://127.0.0.1:1234/test` will read `test.html` file in directory `dist`
