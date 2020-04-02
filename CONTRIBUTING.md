# Contributing to Dessert

First of all, thank you for taking the time to contribute! :smiley:

The following is a set of guidelines for contributing to Dessert. These are mostly guidelines, not rules. Use your best judgment, and feel free to propose changes to this document in a pull request.

## Reporting Bugs (Issues)

Before submitting one, please check for existing issues. When you create an issue, please provide the following information.
* Use a clear and descriptive title.
* Describe the bug reproduce it, as detailed as possible.
* Provide an example to demonstrate the bug.
* Explain the behavior you observed and what behavior you expected to see instead and why.


## Suggesting Enhancements

Before creating suggestion, plese check the issue list if there is already a request.

Create an issue following these guidelines:
* Use a clear and descriptive title.
* If your feature request is related to a problem, describe it.
* Describe the solution you would like.
* Describe the alternatives you have considered.


## Notes

Our modules are based on a core implemented with WebAssembly. We are currently using Rust with [wasm-pack] and [wasm-bindgen] to write our core modules. Every modules who are a *core* end with `-core`, otherwise it is a *connector*, depending on a *core*.

[wasm-pack]: https://github.com/rustwasm/wasm-pack
[wasm-bindgen]: https://github.com/rustwasm/wasm-bindgen