<div align="center">
    <img src="./rsc/movement_logo.png" alt="Project Logo" width="200" height="200">

# M2

[![License](https://img.shields.io/badge/license-BSD-blue.svg)](https://opensource.org/license/bsd-3-clause/)
[![Tests](https://img.shields.io/badge/tests-Passing-brightgreen)](#)
[![Build Status](https://img.shields.io/badge/build-Passing-brightgreen)](#)
[![Coverage](https://img.shields.io/codecov/c/github/username/project.svg)](https://codecov.io/gh/username/project)
[![Windows](https://img.shields.io/badge/Windows-Download-blue)](https://github.com/movemntdev/m1/releases)
[![macOS](https://img.shields.io/badge/macOS-Download-blue)](https://github.com/movemntdev/m1/releases)
[![Linux](https://img.shields.io/badge/Linux-Download-blue)](https://github.com/movemntdev/m1/releases)

**L2s for Move VM.**

</div>


## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

---

## Introduction

The Move programming language poses numerous benefits to builders including direct interaction with digital assets through custom resource types, flexibility with transaction script declaration, on-chain verification, and bytecode safety privileges.

Movement M2 is an ecosystem of Layer 2 technologies for developing L2 smart contracts.

This repository contains the code and contributor documentation for M2. If you would like to learn how to use and develop for the platform, please visit [docs.movementlabs.xyx](docs.movementlabs.xyz).

## Features

Currently, M2 consists of...
- `sov-aptosvm`: a Sovereign Labs rollup of Aptos VM.
- `sov-movevm`: a Sovereign Labs rollup of Move VM.
- `sov-monovm`: a Sovereign Labs rollup combining both `sov-aptosvm` and `sov-movevm` namespaces.
  - Currently, the `sov-monovm` testnet is served at [testnet.sov-monovm.mvlabs.net](testnet.sov-monovm.mvlabs.net). As it is under heavy development, outages are common. Check our status page for know outages.

## Installation

See [docs.movementlabs.xyx](docs.movementlabs.xyz) for a more complete installation guide. We recommend working with our Docker containers or using our installer.

## Usage

Once you've installed our platform, the easiest way to get started developing is to use the CLI to test code locally and publish to our testnet.

```bash
# test
movement move test --sov-aptosvm

# compile and publish
movement move compile && movement move publish --sov-aptosvm
```

## Contributing

Please submit and review/comment on issues before contributing. Review [CONTRIBUTING.md](./CONTRIBUTING.md).

## License

This project is licensed under the BSD-3-Clause License - see the [LICENSE](LICENSE) file for details.

