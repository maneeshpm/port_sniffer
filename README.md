# PORT_SNIFFER
A cli tool to list open ports on you router

## Usage:
Help Menu
```
$ cargo run -- -h
Usage: <Options>... <IPADDR>

-j            specify the number of threads
-h, -help     show help
```
Optional Argument to use multiple threads
```
$ cargo run -- -j 100 192.168.1.1
.....
Found open port: 21
Found open port: 22
Found open port: 53
Found open port: 80
Found open port: 443
```
