# rim
`rim` is a simple text editor similar to `nano` that is implemented in Rust.

## Set up 
1. Clone the repo to your directory of choice and ensure that you have Rust installed
2. Run the following 

```
$ cargo build --release
$ ./target/release/rim [FILE_NAME]
```
> Note: You can (and should) move the executable to a better location.

## Controls
| Action                  | Modifier |  Key(s)    |
|-------------------------|----------|------------|
| Navigation              | -        | Arrow Keys |
| Cursor left             | Ctrl     | b          |
| Cursor right            | Ctrl     | f          |
| Cursor up               | Ctrl     | p          |
| Cursor down             | Ctrl     | n          |
| Cursor to start of line | Ctrl     | a          |
| Cursor to end of line   | Ctrl     | e          |
| Delete at cursor        | Ctrl     | d          |
| Kill to end of line     | Ctrl     | k          |
| Paste killed text       | Ctrl     | y          |
| Save                    | Ctrl     | s          |
| Exit                    | -        | Esc        |
| Exit                    | Ctrl     | x          |

## To be implemented
- Auto scroll when cursor nears border of terminal
- Change refresh to refresh to end of file (starting from previous line)
   * to limit flicker
- Delete to beginning from cursor
- Jump to beginning/end
- Message/command bar
- Find and replace 
- Controls help message
- Open scratch buffer using `rim`
- Exit without saving message
- Undo/redo
- Ctrl-l to center cursor if possible
- Fix slow paste on some terminals (move from iterative to jump)

