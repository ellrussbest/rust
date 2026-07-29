## Let’s Get Rusty

This project is organized into Rust lessons, data structures, algorithms, and coding challenges.

### Project Tracks

| Directory | Purpose |
| --- | --- |
| `lessons/` | Rust language concepts |
| `ds/` | Data structures and their implementations |
| `alg/` | Algorithms and efficient use of data structures |
| `challenges/` | Practice problems from platforms such as LeetCode |

### Run a Topic

Run a topic once:

```bash
./run <track> <number>
```

Run it in watch mode:

```bash
./run-dev <track> <number>
```

Examples:

```bash
./run lesson 1
./run ds 1
./run alg 1
```

### Run Tests

```bash
cargo test -p lesson_13
cargo test -p ds_1
cargo test -p alg_1
```

### Notes

- Valid executable tracks are `lesson`, `ds`, and `alg`.
- Each numbered directory has a short README describing its concepts.
- Challenge directories are organized by platform and challenge number.
