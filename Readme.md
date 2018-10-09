# fpa Filter Pairwise Alignment

[![Build Status](https://travis-ci.org/natir/fpa.svg?branch=master)](https://travis-ci.org/natir/fpa)

Filter output of all-against-all read mapping, you filter or select:

- internal match
- containment
- dovetails
- self matcing
- read name match against regex

All this filter can be revert.

For internal match, containment, dovetails definition go read [algorithm 5 in minimap article](https://academic.oup.com/bioinformatics/article/32/14/2103/1742895/Minimap-and-miniasm-fast-mapping-and-de-novo)

## Rationale

Long Read mapping tools provides all match they found in read dataset, for many usage some of match aren't usfull, this programme provide some filter to remove it. 
This soft can be replace by a simple awk, bash, python, ~perl~, {your favorite language}.

## Requirements

- [Rust](https://www.rust-lang.org/) in stable channel
- libgz
- libbzip2
- liblzma

## Instalation

### With cargo

If you have a rust environment setup you can run :

```
cargo install fpa_lr
```

### With conda

fpa is avaible in [bioconda channel](https://bioconda.github.io/)

if bioconda channel is setup you can run :

```
conda install fpa
```

### From source

```
git clone https://github.com/natir/fpa.git
cd fpa
git checkout v0.1

cargo build
cargo test
cargo install
```

