# Rusty

Rusty is a to-do list manager CLI written in Rust.

## Installation


```bash
git clone https://github.com/hype08/rust-cli.git
```

## Usage

```rust
cargo run get // get your to-do list.
cargo run add "new task" // add a task to the list.
cargo run done `index` // mark task index as done.
cargo run remove `index` // remove task index.
```

## Future Improvements 
- [ ] Instead of taking user input from arguments, run a loop and ask the user for their command every iteration.
- [ ] Implement a command for changing the task description.
- [ ] Implement a custom sort command.

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.


## License
[MIT](https://choosealicense.com/licenses/mit/)