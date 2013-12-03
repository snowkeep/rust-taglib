rust-taglib
===========

Rust interface to the taglib C library.

I think the API I'm using is Rust-ic.  There is an example in [example/example.rs](example/example.rs).
API suggestions and comments on my Rust code are welcome.

rust-taglib is under the [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0).

tag\_c.h was generated with
[https://github.com/crabtw/rust-bindgen](rust-bindgen) (and edited to work)
from /usr/include/taglib/tag\_c.h, which is under the
[http://www.gnu.org/licenses/lgpl.html](GNU Lesser General Public License)
(LGPL) and [http://www.mozilla.org/MPL/MPL-1.1.html](Mozilla Public License
(MPL).



`make`

For documentation:


`make doc`

I tried to set up rustpkg, but could not get it to work.  YMMV:


`rustpkg install github.com/snowkeep/rust-taglib`
