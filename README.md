# Fast Louvain
A Rust implementation of the Louvain algorithm for community detection in large networks. Works on undirected, weighted graphs (weights are optional).

| :arrows_counterclockwise:   | This project is currently a work in progress. Once a first workable version is accomplished, I will publish a release. |
|---------------|:-------------------------|

| :scroll:   | See the documentation [here](https://splines.github.io/fast-louvain/) (currently work in progress). |
|---------------|:-------------------------|

## Build & Run
```
cargo run --bin louvain
```

## Test
Run all unit tests of every workspace.
```
cargo test --lib --locked --workspace
```


<!-- References -->
<details>
<summary><h2>References</h2></summary>

- TODO
</details>

<!-- License -->
<details>
<summary><h2>License</h2></summary>

The source code of this program is licensed with the very permissive MIT license, see the [LICENSE file](https://github.com/Splines/raspi-captive-portal/blob/main/LICENSE) for details. When you use this project (e.g. make a fork that becomes its own project), I do not require you to include the license header in every source file, however you must include it at the root of your project. According to the MIT license you must also include a copyright notice, that is, link back to this project, e.g. in this way:

> [Fast Louvain](https://github.com/Splines/raspi-captive-portal) - Copyright (c) 2023 Splines

Any questions regarding the license? [This FAQ](https://www.tawesoft.co.uk/kb/article/mit-license-faq) might help.

Note that the [documentation book](https://splines.github.io/fast-louvain/) is exempt from the MIT license. Redistribution of the documentation book is not permitted. Yet, you are welcome to reference it in your own work.

</details>

