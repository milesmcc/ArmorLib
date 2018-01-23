# Development Ideology
ArmorLib aims to achieve _stateless development_. Stateless development is software development that is so well documented that any other competent developer could quickly familiarize themselves with the codebase and pick up where the prior developer left off. That is to say, stateless development hides nothing in the developer; all processes are transparent.

Stateless development requires the software project to emphasize the following:
* **Excellent documentation** — files and non-trivial arguments must be explained. Function names should be self-explanatory.
* **High-level transparency** — familiarizing oneself with the high-level software architecture should not require reading the entire codebase. Instead, it should be clearly explained in [STRUCTURE.md](docs/STRUCTURE.md).
* **Feature branches** — work-in-progress features shouldn't be pushed to master. Each new feature should be developed along its own branch (the modular structure of ArmorLib minimizes merge conflicts), and each branch should respond to a specific repository issue.
