# docker-puzzles
[![Build Status](https://travis-ci.com/kobalazs/docker-puzzles.svg?branch=master)](https://travis-ci.com/kobalazs/docker-puzzles)
Docker Puzzles is a CLI tool for putting together Dockerfiles from pieces.

## Installation
For now, use Cargo to install Docker Puzzles:
```
$ cargo install docker-puzzles
```
(To install Cargo, follow instructions at <https://www.rust-lang.org/en-US/install.html>)

## Usage
Run `docker-puzzles` with a parameter of the parent directory of your `Puzzles.yml`
and `Puzzlefile`s. All `Puzzles.yml` files in the directory will be usef to generate
`Dockerfile`s next to each `Puzzlefile`, recursively.

## Examples

### Puzzles.yml
```
echos:
    RUN echo 'a' \
        && echo 'b'
```

### Puzzlefile
```
FROM ubuntu:trusty
PUZZLE echos
```

### Run Docker Puzzles
```
$ docker-puzzles path/to/my/project/directory
```

### Dockerfile generated
```
FROM ubuntu:trusty
RUN echo 'a' \
    && echo 'b'
```
