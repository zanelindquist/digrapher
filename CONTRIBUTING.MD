# Contributing to Digrapher

First off, thank you for considering contributing to Digrapher! Whether you're fixing a bug, improving documentation, suggesting a feature, or refining the UI, contributions of all sizes are appreciated.

Digrapher is a lightweight graph theory and relation analysis application focused on clean visualization, usability, and mathematical tooling.

---

# Getting Started

## Prerequisites

Before contributing, make sure you have:

* Git
* Rust (latest stable recommended)
* `trunk`
* A modern browser

Helpful resources:

* [Rust Installation Guide](https://www.rust-lang.org/tools/install?utm_source=chatgpt.com)
* [Trunk Documentation](https://trunkrs.dev/?utm_source=chatgpt.com)
* [GitHub Forking Guide](https://docs.github.com/en/get-started/quickstart/fork-a-repo?utm_source=chatgpt.com)

---

# Development Setup

Clone the repository and install dependencies:

```bash
git clone https://github.com/zanelindquist/digrapher.git
cd digrapher
cargo install trunk
```

Run the development server:

```bash
trunk serve
```

This will start a local development server with hot reloading enabled.

---

# Our Development Process

## Development Workflow

Working on your first pull request? These resources are excellent places to start:

* [How to Contribute to Open Source on GitHub](https://opensource.guide/how-to-contribute/?utm_source=chatgpt.com)
* [GitHub Pull Request Documentation](https://docs.github.com/en/pull-requests?utm_source=chatgpt.com)

Typical contribution flow:

1. Fork the repository
2. Create a new branch from `main`
3. Make your changes
4. Test your changes locally
5. Commit using the conventional commit format
6. Open a pull request describing your changes

Example:

```bash
git checkout -b feat/treeDiagramLayouts
```

---

# Submitting Issues

Issues are welcome for:

* Bug reports
* Feature requests
* UI/UX improvements
* Documentation problems
* Performance concerns

When submitting an issue, please try to include:

## Bug Reports

* What happened
* What you expected to happen
* Steps to reproduce
* Screenshots (if applicable)
* Browser / OS information

## Feature Requests

* The problem your idea solves
* A proposed solution
* Possible alternatives
* Mockups or diagrams if relevant

Before opening a new issue, check existing issues to avoid duplicates.

---

# Pull Request Guidelines

When opening a pull request:

* Keep changes focused and reasonably scoped
* Explain *why* the change is needed
* Include screenshots for UI changes when possible
* Link related issues if applicable
* Ensure the project still builds successfully

Small, clean pull requests are generally easier to review and merge.

---

# Commit Message Convention

Digrapher follows the Conventional Commits specification for commit messages:

```text
feat: Add matrix export support
fix: Correct relation symmetry analysis
docs: Update installation instructions
refactor: Simplify graph rendering pipeline
test: Add parser unit tests
chore: Update dependencies
```

## Commit Types

* `feat:` → new features
* `fix:` → bug fixes
* `refactor:` → internal code improvements
* `docs:` → documentation changes
* `test:` → adding or updating tests
* `chore:` → tooling or maintenance changes

For breaking changes:

```text
feat: redesign graph serialization format

BREAKING CHANGE: old save files are no longer compatible
```

Learn more:

* [Conventional Commits Specification](https://www.conventionalcommits.org/en/v1.0.0/)

---

# Code Style

General guidelines:

* Prefer readable and maintainable code over clever code
* Keep components focused and modular
* Use descriptive variable and function names
* Maintain consistency with the existing codebase
* Avoid unnecessary dependencies

---

# Documentation Contributions

Documentation improvements are always welcome, including:

* README updates
* Setup clarification
* API explanations
* Better examples
* Screenshots and diagrams

Clear documentation makes the project more approachable for everyone.

---

# Questions

If you have questions, feel free to open an issue or discussion thread on GitHub.

Thanks again for contributing to Digrapher.
