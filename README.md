# PyO3 Test

## Quick Start

**Create virtual environment:**

```sh
uv venv --python 3.13
```

That will create virtual environment in `.venv/` directory.

**Activate virtual environment:**

**Linux:**

```sh
source .venv/bin/activate
```

**Bash (Windows):**

```sh
source .venv/Scripts/activate
```

**PowerShell (Windows):**

```powershell
.venv\Scripts\activate.ps1
```

**Install dependencies, including dev group:**

```sh
# Sync dependencies, including dev group
uv sync --group dev
```

The `--group dev` option includes development group dependencies.

**To build shared library and install it in virtual environment:

```bash
maturin develop
```

**Test:**

```bash
python -c "from pyo3_test import hello; print(hello('world'))"
```

It should output "Hello, world!".

## Release Build

To build `.whl` and sdist `.tar.gz` in `dist/`:

```bash
uv build
```

**Output:**

Successfully built dist\pyo3_test-0.1.0.tar.gz
Successfully built dist\pyo3_test-0.1.0-cp312-cp312-win_amd64.whl

It will target the Python version and glibc detected in the virtual environment, by default.

To build for another Python 3.12:

```bash
uv build --python 3.12
```

> **Note:** That will build Python 3.12 version of wheel, despite that the Python version in the currently active virtual environment is different.

`uv build` with `maturin` backend is effectively same as `maturin build --release`, but outputs to `dist/` directory instead of `target/wheels/`. `uv build` uses `maturin` `--release` build automatically, which performs optimized build.

**To specify glibc version via --manylinux:**

The maturin `--manylinux` option specifies which glibc version to use, for example, `2.34`:

```bash
uv build --config-setting="build-args=--manylinux 2_34"
```

That runs maturin command with `--manylinux` option:

```bash
Running `maturin pep517 build-wheel -i C:\Users\username\AppData\Local\uv\cache\builds-v0\.tmpC1Tzfh\Scripts\python.exe --manylinux 2_34`
```

## Debug Build

If you need a debug build (non-release) while using uv, you cannot simply pass a flag to uv build. Instead, you must use one of these methods:

* Environment Variable: Export `MATURIN_PEP517_ARGS=--profile=dev` before running uv build.
* Config Settings: Use the PEP 517 config settings flag:

```bash
uv build --config-setting="build-args=--profile=dev"
```

**To pass multiple options to maturin, use space separated maturin options:**

```bash
uv build --config-setting="build-args=--manylinux 2_34 --profile=dev"
```

**Output:**
```bash
Running `maturin pep517 build-wheel -i C:\Users\username\AppData\Local\uv\cache\builds-v0\.tmprsgE6A\Scripts\python.exe --manylinux 2_34 --profile=dev`
```
