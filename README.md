# Fast Louvain
A Rust implementation of the Louvain algorithm for community detection in large networks. Works on undirected, weighted graphs (weights are optional).

| :arrows_counterclockwise:   | This project is currently a work in progress. Once a first workable version is accomplished, I will publish a release. |
|---------------|:-------------------------|

| ::scroll::   | See the documentation [here](https://splines.github.io/fast-louvain/) (currently work in progress). |

## Build & Run
```
cargo run --bin louvain
```

## Test
Run all unit tests of every workspace.
```
cargo test --lib --locked --workspace
```