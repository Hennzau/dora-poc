# Dora POC with zenoh as a daemon backend

You should start a Daemon on any machine you need your dataflow to be running on. Give a name to this daemon
and a listening address (or more).

## Run a Daemon

```bash
cargo run -p dpoc-cli -- daemon start NAME_OF_THIS_DAEMON --listen udp/127.0.0.1:7447
```

You can also specify a `connect` value that will be used to connect directly this daemon to a network of daemons. Otherwise,
the daemons will connect when the dataflow will be launched (see below).

Example with connect:

```bash
cargo run -p dpoc-cli -- daemon start NAME_OF_THIS_DAEMON --listen udp/127.0.0.1:7447 --connect udp/192.168.1.1:7447 # assuming there is a daemon running on this address
```

## Interact with a Daemon network

Then you can interact with the daemon network:

```bash
cargo run -p dpoc-cli -- daemon check NAME_OF_A_DAEMON --connect udp/127.0.0.1:7447 # connect address of one of the daemon of the network or a zenoh router
```

Expected output:

```bash
NAME_OF_A_DAEMON: OK
```

```bash
cargo run -p dpoc-cli -- daemon list --connect udp/127.0.0.1:7447 # connect address of one of the daemon of the network or a zenoh router
```

Expected output: list of all daemons running, including names and listening addresses.

## Manage a Dataflow

Write some dataflow, here is an example using a more zenoh-flow syntax but dora's syntax is ok:

```YAML
application: talker-listener

network:
  LOCAL_1: udp/127.0.0.1:7447
  LOCAL_2: udp/127.0.0.1:7446

vars:
  LOCAL_1_DIR: /Users/enzolevan/Offline/dpoc/examples/talker-listener

nodes:
  - id: talker
    files:
      LOCAL_1: "{{LOCAL_1_DIR}}/target/release/talker"
    start: ./talker/talker
    outputs:
      - out

  - id: listener
    files:
      LOCAL_1: "{{LOCAL_1_DIR}}/target/release/listener"
    start: ./listener/listener
    inputs:
      - in

flows:
  listener/in: talker/out

distribution:
  talker: LOCAL_1
  listener: LOCAL_2
```

- `application` is the name of the dataflow
- `network` is a list of daemons to connect to
- `distribution` is a list of nodes and the daemon they should run on

### Validate a datatlow

You can validate a dataflow (check if all daemons involved are running and if all files are present on associated daemons):

```bash
cargo run -p dpoc-cli -- validate PATH_TO_YOUR_DATAFLOW
```

This command will connect to all daemons involved (if possible, otherwise it invalidates the dataflow), and will check for all files to be present
on the associated daemons, checking for adequate input/output flows etc...

Expected output:

```bash
`getting-started` is valid.
```

### Distribute a dataflow

You can distribute dataflow to all involved daemons, which means that every, in this example, distributing `getting-started` will download the `listener`
executable provided inside `files` from the daemon `LOCAL_1` to the daemon `LOCAL_2`, because `listener` should run on `LOCAL_2`:

```bash
cargo run -p dpoc-cli -- distribute PATH_TO_YOUR_DATAFLOW
```
