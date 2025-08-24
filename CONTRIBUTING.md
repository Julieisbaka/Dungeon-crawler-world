# Contributing to Dungeon Crawler World

Thank you for your interest in contributing to Dungeon Crawler World! This
document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [How to Contribute](#how-to-contribute)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Schema Guidelines](#schema-guidelines)
- [Issue Reporting](#issue-reporting)

## Code of Conduct

By participating in this project, you agree to maintain a respectful and
inclusive environment for everyone. Please be considerate of others and
communicate professionally. To view the full Code of Conduct please read the [Code of Conduct](CODE_OF_CONDUCT.md) which is adapted from the Mozilla code of conduct and contributor covenant.

## Getting Started

1. Setup [Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)
2. Install and setup [Rust](https://www.rust-lang.org/tools/install)
3. Clone or fork the repository
4. `cd` into the project directory or open the terminal in the project directory; ensure to go into the root of the project
5. Run `cargo build` to install dependencies and build the project
6. Make any changes and run `cargo build` to ensure everything works. After running the `cargo build` command a new directory called `target` will be created in this directory you can find a executable file. You can also use `cargo run` to run the project.
7. Stage the changes and create a pull request

## How to Contribute

There are many ways to contribute to Dungeon Crawler World:

- **Code contributions** - Implement new features or fix bugs
- **Documentation** - Improve or create documentation
- **Bug reports** - Report issues you encounter
- **Feature requests** - Suggest new features or improvements
- **Game content** - Contribute new items, abilities, or game world elements
- **Playtesting** - Test gameplay and provide feedback

## Pull Request Process

1. Ensure your code adheres to the project's coding standards
2. Update the `README.md` with details of changes if applicable
3. Update the Schema files if you're adding new data structures
4. Your PR should contain a clear description of the changes and their purpose
5. The PR must pass all automated tests
6. A project maintainer will review your PR and may request changes
7. Once approved, your PR will be merged

## Coding Standards

- Use consistent indentation (spaces or tabs as specified in the project)
- Follow naming conventions:
  - PascalCase for classes and methods
  - camelCase for variables and properties
  - snake_case for JSON properties
- Write clear, descriptive comments
- Keep methods focused on a single responsibility
- Write unit tests for new functionality when applicable

## Schema Guidelines

- Follow the JSON Schema [draft-07 standard](https://json-schema.org/specification-links.html#draft-07)
- Add clear descriptions for each property
- Include appropriate validation rules (type, min/max values, etc.)
- Document all changes especially breaking changes

## Issue Reporting

When reporting issues, please include:

1. A clear, descriptive title
2. A detailed description of the issue
3. Steps to reproduce the problem
4. Expected behavior
5. Actual behavior
6. Screenshots if applicable
7. Your environment

---

Thank you for contributing to Dungeon Crawler World! Your efforts help make this project better for everyone.
