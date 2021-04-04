This is a rust application which polls the lichess event stream endpoint
to detect when our lambda function should be invoked. In my personal case
I run this program on a dedicated raspberry pi zero instance as it is a
good balance of low downtime and cheap running costs. The rust binary is
running on "bare metal" i.e. outside of any docker container and needs to
be cross-compiled correctly. Some pain has been avoided in switching from
openssl to rustls but it can still be a bit painful without the right 
tools.

I recommend using the "cross" tool outlined here: 
https://github.com/rust-embedded/cross. I'm running ubuntu on my machine
and it works well, I was running into some issues on macos though.
You can install using

`cross build --release --target arm-unknown-linux-gnueabihf`

and then I scp the binary to the pi.

