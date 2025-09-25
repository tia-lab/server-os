# Getting started - nftables firewall backend

## <a name="preparingyourhost"></a> Preparing your host

* [ ] Have at least kernel 3.18

    The core of nftables is developed as part of the kernel.
    nftables has first been available in kernel 3.13, but support for one of the core features we use (masquerading) has only landed with kernel 3.18.

    You can check your kernel-version by executing `uname -r`:

    ```console
    $ uname -r
    4.15.0-58-generic
    ```

    Should your kernel-version be lower than 3.18, you have to check the documentation for your distribution on how to update it.

    (Having a recent 4.x-kernel is preferrable to profit from potential optimizations and bug-fixes.)

* [ ] Configure Docker daemon

    When you want DFW to manage your firewall, it is essential that you disable the iptables-features integrated in the Docker daemon.
    Probably the easiest way to do this is to modify (or create) the file `/etc/docker/daemon.json` and add the following contents:

    ```json
    {
        "iptables": false
    }
    ```

    Be sure to restart your Docker daemon afterwards.
    (You might also have to remove any rules the Docker-daemon might have already created in iptables.
    The easiest way to do this is to reboot your host.)

* [ ] Install nftables userspace utility `nft` *(optional when running DFW in Docker, but still recommended)*

    Installing the userspace utility for nftables, `nft`, is strictly necessary if you run the DFW-binary directly on your host, and overall recommended no matter how you run DFW.
    How to install `nft` depends on your distribution/package manager, but one of the following should work:

    * `apt install nftables` (for Debian, Ubuntu, ...)
    * `yum install nftables` (for RHEL, CentOS, ...)
    * `pacman -S nftables` (for Arch, Manjaro, ...)
    * `zypper install nftables` (for SLES, OpenSuse, ...)

* [ ] Migrate any custom iptables-rules you have to nftables *(if you haven't managed any iptables rules or are already using nftables, you can skip this step)*

    A description on how to migrate your iptables-rules [can be found in the nftables wiki][nftableswiki-movingfromiptables].
    The rough outline is as follows:

    1. Export your iptables rules (`iptables-save`).
    2. Translate them into nftables rules (`iptables-restore-translate`).
    3. Import them into nftables (`nft -f`).

    One point to add to this: most distributions include the file `/etc/nftables.conf` as part of their nftables userspace package, which will be automatically loaded on system-boot through the systemd-service `nftables`.
    This is an easy way to add your pre-existing (or new) rules to the default nftables-chains and have them be loaded whenever you boot.

    [nftableswiki-movingfromiptables]: https://wiki.nftables.org/wiki-nftables/index.php/Moving_from_iptables_to_nftables

* [ ] Prepare the default nftables-configuration (or the configuration you imported in the previous step)

    As mentioned in the previous step, most distributions include the file `/etc/nftables.conf` as their default nftables configuration when you install the `nft` utility.
    This file usually contains a very barebones ruleset, including the `input`, `forward` and `output` chains, all configured to accept all traffic (there might be no explicit policy, which equals `accept`).

    One suggestion here is to change the `input` and `forward` policies to `drop` packets that are not explicitly allowed, ensuring that nobody can access resources that you don't want to be public.
    **BE CAREFUL:** do not apply this change before you also added any rules you might require to e.g. access SSH on your host.
    The following is a simple example for what this configuration file can look like:

    ```shell
    #!/usr/sbin/nft -f

    flush ruleset

    table inet filter {
        chain input {
            type filter hook input priority 0; policy drop;
  
            # Ensure local traffic is accepted still
            iif lo accept
            # Allow established connections (e.g. responses to outgoing traffic)
            ct state { established, related } accept
            # Allow incoming SSH connections
            tcp dport 22 accept
        }
        chain forward {
            type filter hook forward priority 0; policy drop;
        }
        chain output {
            type filter hook output priority 0; policy accept;
        }
    }
    ```

    This configuration specifically adds a rule to accept incoming connections on port 22.

    The `/etc/nftables.conf` configuration file can be loaded through various ways, here are some examples (all executed as `root` or through `sudo`):

    * ```systemctl reload nftables```
    * ```/etc/nftables.conf``` (the configuration itself can be executed)
    * ```nft -f /etc/nftables.conf```

    All three options lead to the same result: the entire ruleset will be discarded and subsequently filled with what you have configured.

## <a name="configuration"></a> Configuration

The general configuration happens across six categories:

* `global_defaults`

    This category defines global, default values to be used by DFW and the other categories.

    [Field reference.](https://dfw.rs/1.3.0/dfw/types/struct.GlobalDefaults.html)

* `backend_defaults`

    This category defines configuration values that are specific to the firewall-backend used.

    [Field reference for `nftables`.](https://dfw.rs/1.3.0/dfw/nftables/types/struct.Defaults.html)

* `container_to_container`

    This controls the communication between containers and across [Docker networks][docker-networks].

    [Field reference.](https://dfw.rs/1.3.0/dfw/types/struct.ContainerToContainer.html)

* `container_to_wider_world`

    This controls if and how containers may access the wider world, i.e. what they can communicate across the `OUTPUT` chain on the host.

    [Field reference.](https://dfw.rs/1.3.0/dfw/types/struct.ContainerToWiderWorld.html)

* `container_to_host`

    To restrict or allow access to the host, this section is used.

    [Field reference.](https://dfw.rs/1.3.0/dfw/types/struct.ContainerToHost.html)

* `wider_world_to_container`

    This controls how the wider world, i.e. whatever comes in through the `INPUT` chain on the host, can communicate with a container or a Docker network.

    [Field reference.](https://dfw.rs/1.3.0/dfw/types/struct.WiderWorldToContainer.html)

* `container_dnat`

    This category allows you to define specific rules for destination network address translation, even or especially across Docker networks.

    [Field reference.](https://dfw.rs/1.3.0/dfw/types/struct.ContainerDNAT.html)

**See the [examples][examples] and [configuration types][types.rs] for detailed descriptions and examples of every configuration section.**

[docker-networks]: https://docs.docker.com/engine/userguide/networking/
[examples]: https://github.com/pitkley/dfw/tree/main/examples
[types.rs]: https://dfw.rs/1.3.0/dfw/types/index.html

## <a name="runningdfw"></a> Running DFW

You have two general options of running DFW:

* Using the official Docker image *(preferred!)*.
* As a binary directly on your host.

### Using the official Docker image

```console
$ docker pull pitkley/dfw:1.3.0
$ docker run -d \
      --name=dfw \
      -v /var/run/docker.sock:/var/run/docker.sock:ro \
      -v /path/to/your/config:/config \
      --net host --cap-add=NET_ADMIN \
      pitkley/dfw:1.3.0 --config-path /config
```

This will download a lightweight image, coming in at around 20 MB, and subsequently run it using your configuration.
The image supports multiple architectures: `amd64`, `arm64`, `armv7` (specifically `armhf`).

Please note that you can also pull the image from the GitHub container registry, GHCR, if you want to avoid potential pull-limitations Docker Hub has put in place:

```console
$ docker pull ghcr.io/pitkley/dfw:1.3.0
$ docker run ... ghcr.io/pitkley/dfw:1.3.0 ...
```

### Using a pre-built binary directly on your host.

You can retrieve the latest pre-built binary from the GitHub releases page:

* [Release page](https://github.com/pitkley/dfw/releases/latest)
* [Direct download](https://github.com/pitkley/dfw/releases/latest/download/dfw-x86_64-unknown-linux-musl) (static Linux x86_64 binary, no further dependencies required)

### Install DFW through crates.io.

For this you need to first [install Rust][rustlang-install] and then install DFW using cargo:

```console
$ cargo install dfw
$ dfw --help
dfw 1.3.0
Docker Firewall Framework, in Rust
...
```
### Build from source.

For this you need to first [install Rust][rustlang-install].
You can then check out the repository and build the binary:

```console
$ git checkout https://github.com/pitkley/dfw
$ cd dfw/
$ cargo build --release
$ target/release/dfw
Docker Firewall Framework, in Rust
...
```

[rustlang-install]: https://www.rust-lang.org/tools/install
