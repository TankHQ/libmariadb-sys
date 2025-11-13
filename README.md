# libmariadb-sys
Native bindings to the libmariadb library for Rust.

## License Gotcha
While this crate itself is licensed under Apache-2.0, it provides bindings to the MariaDB Connector/C library (libmariadb), which is licensed under the LGPL-2.1 license. When using this crate—particularly with the `bundled` feature, which compiles and statically links the MariaDB library—your application must comply with the LGPL terms. Key limitations include:

- If statically linking the LGPL-licensed library, you must ensure that users can replace it with a modified version. This often requires providing object files or using dynamic linking to allow relinking.
- You cannot incorporate the LGPL code into proprietary software in a way that prevents users from modifying and relinking the LGPL portions.
