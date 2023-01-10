# printrn

A macro to replace `println!()`, that will replace all `\n` with `\r\n`.

This is useful when working with raw terminals, such as via `crossterm` or `tui`. In raw terminal mode, `\n` will only move the current line one down, and keep printing from there, like so:

```
APPLE
     BOOK
         CHERRY
```

Which is where `\r` (short for carriage Return) comes in. By replacing all `\n` with `\r\n`, you can get the more expected behavior:

```
APPLE
BOOK
CHERRY
```