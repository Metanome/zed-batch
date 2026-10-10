## Zed Windows Batch Extension

![Zed download chart](https://zedbadge.dev/chart/windows-batch.svg)

Run `.bat` and `.cmd` files without leaving Zed. Every batch file offers two tasks:

| Task | What it does |
| --- | --- |
| `Run <file>` | Runs the file in the terminal |
| `Run <file> as Administrator` | Runs the file elevated, asking for UAC only when needed |

Start one from the play button on the file's first line, or from `task: spawn` (`ctrl-shift-p`). The file is saved first, runs from its own directory, and an elevated run echoes its output into the task's terminal.

To run with a key, bind `editor::SpawnNearestTask` in your `keymap.json`:

```json
{
  "context": "Workspace",
  "bindings": {
    "f5": "editor::SpawnNearestTask"
  }
}
```

## Author
![Zed author badge](https://zedbadge.dev/author/pleahmacaka.svg)  

## License

[MIT](./LICENSE)
