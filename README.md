Rudimentary implementation of MessagePack formatter for
[slog](https://github.com/slog-rs/slog/).

This repository is not actively maintained. I am uploading it here in
case I ever need to pick it up again. I wrote it in hopes of shrinking
my log size, but for the toolchain I'm using it doesn't provide enough
of a space savings to justify the added complexity of maintaining my
own formatter and adding a dependency on messagepack.

This is because, in order to keep size down, I still need to gzip the
results. Since the gzip ends up dominating time and space anyway, just
using JSON makes more sense.

The code here is based on
[slog-json](https://github.com/slog-rs/slog/), with minor
modifications to use the [rmp](htts://github.com/3Hren/msgpack-rust)
serializer.
