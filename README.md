# description

Convert configuration file format to each other.

Valid input/output file format:

|  Input   | Output  |
|  ----  | ----  |
| yaml | yaml |
| json  | json |
| toml | toml |

# compile

rust version 1.56 or higher is needed.

compile command:
```bash
cargo build
```

# usage

1. choose what file type as input. like `json`
2. choose what file type you want to convert to. like `toml`
3. then you get convert mode `json2toml`
4. usage: `file_converter --mode <MODE> <FILE>`

# todo

- [ ] add xml support
- [ ] add ini support
