\# string-utils



\*\*A lightweight Rust library for efficient string manipulation and path handling.\*\*



\[!\[Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)

\[!\[License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)



\## Overview



`string-utils` is a high-performance utility library that provides a collection of functions for common string operations, path normalization, and case conversion. It focuses on safety and speed, making it suitable for both CLI tools and server-side applications.



\## Features



\-  \*\*Fast string trimming\*\* (`trim`, `trim\_start`, `trim\_end`)

\-  \*\*Path normalization\*\* (converts `\\` to `/` across platforms)

\-  \*\*Case conversion\*\* (`to\_camel\_case`, `to\_snake\_case`, `to\_kebab\_case`)

\-  \*\*Zero dependencies\*\* – minimal footprint



\## Installation



Add this to your `Cargo.toml`:



```toml

