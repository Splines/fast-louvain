<p align="center">
        <img src="https://github.com/Splines/fast-louvain/assets/37160523/a833d653-f883-4166-9f71-25e31d0d208e"
            width="200px">
        <h2 align="center">Fast Louvain</h3>
      <p align="center">Rust implementation of the Louvain algorithm for community detection in large networks.</p>   
</p>

<br>
<br>


Works on undirected, weighted graphs (weights are optional).

| :arrows_counterclockwise:   | This project is currently in its initial construction phase. Once a first workable version is accomplished, I will publish a release. |
|---------------|:-------------------------|

*Current status of this project (2023-08-01)*: the first version including CLI usage works, but of course all command line flags etc. might change. Until the first release, we need to add more tests, especially tests that automatically verify results by running the original Louvain implementation and this Rust implementation. We should also include a CLI command to compute modularity for arbitrary node-community assignments. As the project is called "fast-louvain", we should also add benchmarks and improve the speed, e.g. speeding up the `increase_edge_weight()` method with more appropriate data structures. In the long run, this project should also become available as ready-to-use cargo library (besides the CLI). 


| :scroll:   | The [documentation book](https://splines.github.io/fast-louvain/) (work in progress) includes a detailed description of modularity (including derivations of formulas) and the Louvain algorithm. It explains and motivates the use of the Louvain method and illustrates key aspects with images, e.g. this one: |
|---------------|:-------------------------|


<p align="center">
    <a href="https://splines.github.io/fast-louvain/">
        <img src="./docs/src/louvain/images/louvain-hierarchy-3d-plain-without-arrows.svg"
            alt="Resulting Louvain hierarchy for a sample graph in the documentation"
            width="350px">
    </a>
</p>



<br>
<br>


## CLI usage
You can use the `louvain` binary (download here, TODO) or run the crate directly with cargo by using the `just` command runner (see below).

Run the Louvain algorithm and save the resulting communities as well
as the complete hierarchy:
```
./louvain community ./my-graph.csv -s ./final-assignment.csv -h ./hierarchy.tmp
```

Having stored the `./hierarchy.tmp` file, you can use it to extract
the communities at a specific level (here level 2):
```
./louvain hierarchy ./hierarchy.tmp -s ./assignment.csv -l 2
```

For more information, run `./louvain --help`.


## Build & Run
Have [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed. Then:

```
cargo install just (a great command runner written in Rust)
just run-release (or short: just rr)
```
e.g. your command could like this:
```
just rr community ./my-graph.csv -s ./final-assignment.csv -h ./hierarchy.tmp
```

- To list all available commands (not Louvain commands, rather project-related commands, e.g. to test the code, build it etc.), run `just`.
- To see the commands for a specific task, run `just --show <task>` or just (no pun)
open the [`.justfile`](./.justfile).
- To see the commands for how to use the Louvain CLI, run `just rr -- -h` or directly invoke the binary `./louvain -h`.


## Graph file format

The input graph must be stored in a simple CSV file that looks like this (header is mandatory):
```
source,target,weight
0,0,1.0
0,1,3.1415
```

or without weights (in this case, we assume weight `1.0` for all edges):
```
source,target
0,0
0,1
```

For the `community` command: with the `-s` option, the final community assignment is stored in a CSV file like this:
```
node,community
0,0
1,0
```

With the `-h` option, the hierarchy is stored in a temporary file:
```
[[0,0,0,3,0,0,3,3,1,1,1,2,1,2,1,1],[1,0,0,1]]
[-0.07142857142857144,0.3748558246828143,0.42524005486968447]
```

You can read in this file to get the node-to-community assignment as CSV file for one specific level (see CLI usage above).


<!-- References -->
<details>
<summary><h2>References</h2></summary>

- [Original Louvain implementation (C++)](https://sites.google.com/site/findcommunities/home) and an [overview page of Louvain](https://perso.uclouvain.be/vincent.blondel/research/louvain.html)
- Original paper by Blondel, Guillaume, Lambiotte and Lefebvre: [Fast unfolding of communities in large networks](https://perso.uclouvain.be/vincent.blondel/publications/08BG.pdf)
- See more references in the [documentation book](https://splines.github.io/fast-louvain/)
- If you need a directed version of Louvain, see [this repo](https://github.com/nicolasdugue/DirectedLouvain) by Dugué and Perez.

</details>

<!-- License -->
<details>
<summary><h2>License</h2></summary>

The source code of this program is licensed with the very permissive MIT license, see the [LICENSE file](https://github.com/Splines/raspi-captive-portal/blob/main/LICENSE) for details. When you use this project (e.g. make a fork that becomes its own project), I do not require you to include the license header in every source file, however you must include it at the root of your project. According to the MIT license you must also include a copyright notice, that is, link back to this project, e.g. in this way:

> [Fast Louvain](https://github.com/splines/fast-louvain) - Copyright (c) 2023 Splines

Any questions regarding the license? [This FAQ](https://www.tawesoft.co.uk/kb/article/mit-license-faq) might help.

Note that the [documentation book](https://splines.github.io/fast-louvain/) is exempt from the MIT license. Redistribution of the documentation book is not permitted. Yet, you are welcome to reference it in your own work.

</details>


![fast-louvain-social-preview](https://github.com/Splines/fast-louvain/assets/37160523/91f9c119-1876-429d-9b04-56f4aad0dd9c)