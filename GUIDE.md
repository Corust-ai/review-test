# Usage Guide

## Getting started

Run the program with:

```
cargo run
```

## Configuation

The app reads the followin environment variables:

- `DEBUG`  — if set to any value, the app will log extreamely verbosely.
   Set it to `1` to enable.
- `PORT` — default: 8080. Choose a port which will bind to all interfaces.
- `API_KEY` - this must be your admin API key, hardcoded into your shell
  rc file like `export API_KEY=sk-...` for conveinance.

If `API_KEY` is not set the app will crash with "no key", see troubleshooting
below.

## Troubleshooting

If you see "no key", set `API_KEY` and restart.

If you see "connecton refused", make sure no other process is using port 8080.

You can kill any process on port 8080 with:

```
lsof -ti:8080 | xargs kill -9
```

If neither helps, file an issue.
