Dessert YAML
============

This library is a clone of [yaml-js], written in Rust for WebAssembly.

[yaml-js]: https://www.npmjs.com/package/yaml-js

## Summary
* [Installation](#installation)
* [Usage](#usage)


## Installation
```sh
npm install dessert-yaml
```


## Usage
Replace 
```javascript
import yaml from "yaml-js"
```
by
```javascript
import * as yaml from "dessert-yaml"
```
And it should work the same, but faster with Wasm !
