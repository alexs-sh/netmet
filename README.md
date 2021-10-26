### About

The silly app for collecting information about TCP.


### Usage


**Server**

```
./netmet
mode:server
address:0.0.0.0:8888
...
```


**Client**
```
./netmet client 172.28.21.18:8888 100 256
mode:client
address:172.28.21.18:8888
cycles:100
payload:256

#        Duration, uS     Status   Name
0        608              true     connect to server
1        32               true     send START
2        2709             true     wait ACK
3        48               true     send request
...
200      41528            true     read response
201      21               true     send request
202      37650            true     read response
203      27               true     send STOP
```

172.29.21.18:8888 - IP address and port of test server
100 - number of cycles
256 - test payload size


### Build


**Host**

```
cargo build --release

```


**ARMv5**

```
 cargo build --release --target armv5te-unknown-linux-gnueabi
```
