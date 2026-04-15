# LXD simple client

A simple `unixSocket`-based client for `LXD` REST APIs. Educational project 
to:
- get an idea of what LXC does (and what would be needed to build an
  alternative client);
- implement, using `unix socket`, an interface to a known Linux daemon.

Only `read` (`GET`) APIs for now.

## How to use it

Follow [LXD documetation](https://documentation.ubuntu.com/lxd/latest/getting_started/) to 
install and initialize LXD. Check LXC to understand what you can to with `Linux Containers`.

To run this project, just run:
    
```shell
  cargo run 
```

You should get the list of `images` and `clusters` you have locally.
