# mogura: A molecular graphic (toy) visualizer in Rust

[Try demo with WASM](https://mogura-rs.github.io/mogura/)

> [!WARNING]
> There are some limitation for WASM version.
> See [Feature](#feature)

## Quick Start
### Install
~~~bash
cargo install --git https://github.com/mogura-rs/mogura mogura
~~~

### Run mogura
~~~bash
# visualize PDB: 8GNG
mogura 8gng.pdb
# visualize MD simulation of chignolin
mogura init.gro input.mol.compact.xtc
~~~

## About the name

$$
\text{Molecular Graphic Visualizer} \rightarrow \text{Mole + graphic} \rightarrow \text{Mo + gra} \rightarrow \text{mogura}
$$

mogura also means "Mole".


## Gallery
Input files are available [here](https://github.com/mogura-rs/example-inputs)



## Feature
- Visualize PDB, GRO format
  - stick, stick&ball, ball, tube, line mode are supported
  - wasm does not suppert line (because WebGPU)
- Visualize MD simulation using XTC format
  - topology must be PDB or GRO format
  - WASM does not support this (because of groan_rs, this crate depends internally on libc)
- Atom selection language
  - `resname, resid, name, index, protein, water, backbone, sidechain, ion, all, (), and, or, not` are reserved words
  - support complex syntax
    - e.g. `protein and name C CA N`, `(resname TYR PRO) and protein or not water`
- Multiple selection panel
  - multiple atom selection are possible for a single structure file.
- Fetch PDB
  - dirty pdb file will cause parse error because of pdbtbx.


## Useful Reference

- bevy
  - https://github.com/bytestring-net/bevy_lunex
  - https://github.com/bevyengine/bevy
  -
- graphics
  - https://github.com/svenstaro/bvh
  - https://github.com/pannapudi/voidin
  - https://github.com/BLaZeKiLL/webray
  - https://github.com/servo/pathfinder
  - https://github.com/NotCamelCase/RasterizationInOneWeekend
  - https://github.com/RayTracing
- egui
  - https://github.com/emilk/egui
- wgpu
  - https://github.com/gfx-rs/wgpu
  - https://github.com/jack1232/wgpu-step-by-step
- pdb, gro, xtc
  - https://github.com/douweschulte/pdbtbx
  - https://github.com/Ladme/groan_rs
- nom
  - https://github.com/rust-bakery/nom
- other great visualizer
  - [PyMol](https://github.com/schrodinger/pymol-open-source)
  - [VMD](https://www.ks.uiuc.edu/Research/vmd/)
  - [ChimeraX](https://github.com/RBVI/ChimeraX)
  - [Cuemol](https://github.com/CueMol/cuemol2)
  - [ngl](https://github.com/nglviewer/ngl)
- other visualizer in Rust
  - [ferricyanide](https://github.com/frodofine/ferricyanide)
