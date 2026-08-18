# Notes for developers

## Install dependencies

- Install Rust with the help of `rustup` (https://rustup.rs/)
- Install Pythons dependencies:
  ```shell
  uv venv
  uv pip install .[dev] 
  ```

## Build Rust library

### Debug version

```shell
uv run maturin develop
```

### Optimized version

```shell
uv run maturin develop --release --strip
```

## Run tests

```shell
uv run pytest
```

## Build release wheels and sdist

```shell
uv run maturin build --release --strip
```
