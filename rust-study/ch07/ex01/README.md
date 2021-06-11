# 7 - 1

이장에서는 cargo를 이용해 새로운 package, crate를 만드는 방법에대해 설명한다

crate는 binary또는 lib입니다.
package 는 기능 세트를 제공하는 하나 이상의 상자. 여기에서는 Cargo.toml 이 포함 돼 있습니다.

package는 0 or 1개 의 lib상자를 포함해야하며, 더 이상 포함하지 않아야 한다.

```sh
$ cargo new my-projec

$ ls my-project

Cargo.toml
src

$ ls my-project/src
main.rs
```

project를 생성하는 방버에는

```sh
$ cargo new [target] [--lib,--bin(default)]
```

```sh
$ mkdir [target]
$ cd [target]
$ cargo init [--bin(default),--lib]
```
