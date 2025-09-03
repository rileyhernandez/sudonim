# sudonim

`sudonim` is a command-line tool for managing and connecting to devices on your local network. It allows you to save devices with custom names, so you don't have to remember their IP addresses, which can frequently change. The command-line interface for this tool is `snim`.

## Installation

To install `sudonim`, you need to have Rust and Cargo installed. You can then clone this repository and build the project:

```bash
git clone <repository-url>
cd sudonim
cargo install --path .
```

## Dependencies

`sudonim` relies on the following external command-line tools:

*   `nmap`: For scanning the network to discover devices.
*   `iproute2`: For showing neighbor information (specifically the `ip neigh show` command).
*   `ssh`: For connecting to devices.

Please ensure these tools are installed and available in your system's `PATH`.

## Configuration

Before you can use `sudonim`, you need to initialize its configuration file. This is done with the `init` command:

```bash
snim init
```

This will create a `config.toml` file at `~/.config/sudonim/config.toml`.

## Usage

`sudonim` provides several commands for managing your devices:

### `init`

Initializes the configuration file.

```bash
snim init
```

### `list`

Lists all the devices you have saved.

```bash
snim list
```

### `new <subnet>`

Scans the network for new devices and provides an interactive prompt to add a new device to your saved list.

```bash
snim new 192.168.1.0/24
```

### `remove <device>`

Removes a device from your saved list.

```bash
snim remove my-server
```

### `ssh <device>`

Connects to a device using SSH.

```bash
snim ssh my-server
```

### `edit <device>`

Edits the IP address of a saved device.

```bash
snim edit my-server
```

### `rescan <device> --subnet <subnet>`

Rescans the network to find the new IP address of a device based on its MAC address. This is useful when a device's IP address has changed.

```bash
snim rescan my-server --subnet 192.168.1.0/24
```

### `scan --subnet <subnet>`

Scans the network and lists all the devices found, along with their IP and MAC addresses.

```bash
snim scan --subnet 192.168.1.0/24
```