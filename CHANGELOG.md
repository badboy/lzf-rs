### 1.0.0 (2022-11-06)

This is the same as v0.3.2

This crate is now DEPRECATED.
The API now is suboptimal and not as fast as I hoped.

### 0.3.2 (2022-11-06)

DO NOT USE <= v0.3.1.
It's broken, wrong and unsound.

* Remove wrong usage of `mem::unitialized` and replace it with a heap-allocated `Vec`.
  This might be slower than the initial version, but at least not broken.

### 0.3.1 (2015-09-27)

* Correct output length calculation

### 0.3.0 (2015-09-16)

* Replace C code with Rust code

### 0.2.3 (2015-04-16)

* Fix tests to work on beta

### 0.2.2 (2015-04-16)

* Make it work with Rust 1.0 Beta

### 0.2.1 (2015-02-15)

* Add documentation

### 0.2.0 (2015-02-15)

* Return unknown error as i32

### 0.1.4 (2015-02-15)

* Adopt features and Debug trait

### 0.1.3 (2015-01-16)

* Implement Display for LzfError to allow for unwrap of the result
* Use Show to display an error

### 0.1.2 (2015-01-16)

* Updated to work with Rust 1.0

### 0.1.1 (2015-01-16)

* Derive copy as suggested by latest nightly

### 0.1.0 (2014-11-16)

* Basic functionality
