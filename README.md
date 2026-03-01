# PyO3 Test

## Development Commands

### Install Dependencies

```sh
    pixi install -e py314
```

Substitute `py314` with the actual virtual environment.

### Build for Development

```sh
pixi run -e py314 develop
```

This installs package in the virtual environment.

### Build Wheel for Release Distribution

```sh
pixi run -e py314 build
```

This does not install package in the virtual environment.
