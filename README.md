# ri
`ri` is a simple text editor similar to `nano` that is implemented in Rust.

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
- fix cursor move issue on delete non empty last line
- Show '-' if text is cut off horizontally
- Page down/up & left/right
- Message/command bar
- Find and replace 
- Controls help message
- Open scratch buffer using `rim`
- Exit without saving message
- Undo/redo