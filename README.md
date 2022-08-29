# AIR

AIR (**A**tomic **I**ntermediate **R**epresentation) is an experimental language spec for proving statements about atomic operations under the C++/Rust atomics model.

## Examples

### Store and Load

```air
data = 0;

// Thread 1
{
    [relaxed] data = 1;  // data.store(1, atomic::Ordering::Relaxed) in Rust
}

// Thread 2
{
    out = [relaxed] data; // out = data.load(atomic::Ordering::Relaxed) in Rust
}
```

Can represent the following memory orderings

```text
     Possible Execution 1       ┃       Possible Execution 2
                                ┃
Thread 1     data    Thread 2   ┃  Thread 1     data    Thread 2
╭───────╮   ┌────┐   ╭───────╮  ┃  ╭───────╮   ┌────┐   ╭───────╮
│ store ├─┐ │  0 ├───┤  load │  ┃  │ store ├─┐ │  0 │ ┌─┤  load │
╰───────╯ │ └────┘   ╰───────╯  ┃  ╰───────╯ │ └────┘ │ ╰───────╯
          └─┬────┐              ┃            └─┬────┐ │
            │  1 │              ┃              │  1 ├─┘
            └────┘              ┃              └────┘
```

> See <https://github.com/SabrinaJewson/rust-nomicon/blob/atomics/src/atomics/multithread.md> for more details about these diagrams

### RMW

```air
counter = 0;

// Thread 1
{
    [relaxed] data += 1;  // data.fetch_add(1, atomic::Ordering::Relaxed) in Rust
}

// Thread 2
{
    [relaxed] data += 1;  // data.fetch_add(1, atomic::Ordering::Relaxed) in Rust
}
```

Would represent these orderings

```text
  Thread 1     COUNTER     Thread 2     ┃     Thread 1     COUNTER     Thread 2
╭───────────╮   ┌───┐   ╭───────────╮   ┃   ╭───────────╮   ┌───┐   ╭───────────╮
│ fetch_add ├─┐ │ 0 │ ┌─┤ fetch_add │   ┃   │ fetch_add ├─┐ │ 0 │ ┌─┤ fetch_add │
╰───────────╯ │ └───┘ │ ╰───────────╯   ┃   ╰───────────╯ │ └───┘ │ ╰───────────╯
              └─┬───┐ │                 ┃                 │ ┌───┬─┘
                │ 1 │ │                 ┃                 │ │ 1 │
                └───┘ │                 ┃                 │ └───┘
                ┌───┬─┘                 ┃                 └─┬───┐
                │ 2 │                   ┃                   │ 2 │                   
                └───┘                   ┃                   └───┘                   
```
