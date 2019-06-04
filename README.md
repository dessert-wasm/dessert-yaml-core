Dessert YAML
============

This library is the base API for YAML modules, written in Rust for WebAssembly.

For simplicity's sake, this module has been written using [serde yaml], but at the cost of flexibility. Some feature might be missing for now, like passing parsing options when loading/dumping.

[serde yaml]: https://github.com/dtolnay/serde-yaml

## Summary
* [Installation](#installation)
* [Performance](#Performance)
* [API](#api)
* [Building](#building)


## Installation
```sh
npm install dessert-yaml-core
```

## Performance

Big file (6.85MB)

|           | dessert-yaml | js-yaml |  yaml-js |
|:---------:|:------------:|:-------:|:--------:|
|    load   |    1904ms    |  836ms  |  55461ms |
|    dump   |     528ms    |  1193ms | 124931ms |

Small file (360 B)

|           | dessert-yaml | js-yaml | yaml-js |
|:---------:|:------------:|:-------:|:-------:|
|    load   |      1ms     |   11ms  |   33ms  |
|    dump   |      1ms     |   8ms   |   37ms  | 


## API
This section lists methods exported from Wasm to Javascript.

> Note:  
Althought this library is usable as-is, it should be used as a dependency for a connector, such as [dessert-js-yaml].

[dessert-js-yaml]: https://github.com/dessert-wasm/dessert-js-yaml

``` javascript
yaml = require('dessert-yaml-core');
fs   = require('fs');

// Get document, or throw exception on error
try {
  let doc = yaml.load(fs.readFileSync('/home/ixti/example.yml', 'utf8'));
  console.log(doc);

  // Print the parsed object as a YAML stream
  let yaml_stream = yaml.dump(doc);
  console.log(yaml_stream);
} catch (e) {
  console.log(e);
}
```

### load(string, [, options])
Most simple way of parsing a document. Parses string as single YAML document. Returns a JavaScript object or throws YAMLException on error. options aren't used as for now.


### loadSafe(string, [, options])
Same as load()


### loadAll(string, [, options])
Same as safeLoad(), but understands multi-document sources. Applies iterator to each document if specified, or returns array of documents.
Currently same as load() because serde_yaml doesn't support multi document yet, and data structure from yaml-rust aren't serializable.


### dump(object, [, options])
Serializes object as a YAML document


### safeDump(objects, [, options])
Same as dump()


## Building
The package can be built using [wasm-pack]

[wasm-pack]: https://rustwasm.github.io/wasm-pack/installer/

```sh
wasm-pack build --target browser
```

Testing:
```sh
wasm-pack test --headless --[firefox|chrome|safari]
```
