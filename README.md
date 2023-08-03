# thor-rs

A super fast cli tool to keep your `node_modules` healthy and fresh.

## Features

- [ ] List tons of installed packages in 1s
- [ ] Fix potentially problematic packages
- [ ] Support workspace mode

## Install

```shell
$ npm i -g thor-rs
```

## Commands

### `thor list`

List all the installed `node_modules`.

### `thor outdated`

List or update outdated packages based on lock-files or installed `node_modules`. 

### `thor audit`

List or fix vulnerable packages based on lock-files or installed `node_modules`. 

