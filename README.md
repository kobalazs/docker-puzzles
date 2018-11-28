# docker-puzzles
Docker Puzzles is a CLI tool for putting together Dockerfiles from pieces.

## Installation
For now, use Cargo to install Docker Puzzles: `cargo install docker-puzzles`

## Usage
Run `docker-puzzles` with a parameter of the parent directory of your `Puzzles.yml`
and `Puzzlefile`s. All `Puzzles.yml` files in the directory will be usef to generate
`Dockerfile`s next to each `Puzzlefile`.

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
`$ docker-puzzles path/to/my/project/directory`

### Dockerfile generated
```
FROM ubuntu:trusty
RUN echo 'a' \
    && echo 'b'
```
