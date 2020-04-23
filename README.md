# `hostap`

[![Build Status](https://travis-ci.com/mvertescher/hostap-rs.svg?branch=master)](https://travis-ci.com/mvertescher/hostap-rs)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

`hostap` is a Linux tool for debugging 802.11 wireless devices.
It provides an out of the box solution for eavesdropping on Wi-Fi devices for
testing purposes. This is achieved by setting up a local wireless network on an
interface that supports AP mode then routing traffic to the external network.

## Warning

This tool currently requires root access for the following:

- Allow NetworkManager to ignore the selected interface
- Bring up/down the selected wireless interface using iproute2
- Use sysctl to forward ipv4 packets
- Manipulate the host's internal routing table to forward ip packets

## Dependencies

- NetworkMananger (only supported manager currently)
- iproute2
- hostapd
- dhcpd
- iptables

## Tested Network Adapters

To use this tool, you must have a dongle or local wireless interface that supports AP mode.

Supported adaptors:
- Alfa AWUS036NHA

## Quick Start

1. Select your wireless interface from the following list:

```sh
ip link
```

2. Bring up a network on the selected interface:

```sh
hostap -i <interface>
```

## Related Projects

- [create_ap](https://github.com/oblique/create_ap)

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
