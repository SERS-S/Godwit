# Godwit

## Description:

This is software for sending files using FTP protocol with asymmetric encryption feature (**FTP Secure**). The data movement technology is based on TCP protocol and continuous connection between client and server. The software is written in ***Rust*** and partially in ***Python***. The program is written so that everyone can use it regardless of the project language they choose. 



https://github.com/SERS-S/Godwit/assets/104014082/56ef76a5-2a15-4a7e-933d-0cd5709a8a7e



## The following presets are required for the software to work:

- Download [python version 3](https://www.python.org/downloads/)

- Download [rust language](https://www.rust-lang.org/tools/install)

## Set Up project:

### For Unix/Linux platforms:
```sh
git clone https://github.com/SERS-S/Godwit.git
cd Godwit
chmod +x start.sh 
./start.sh
```

### For Windows platform:
```sh
git clone --no-checkout https://github.com/SERS-S/Godwit.git
cd Godwit\transmission && (cd tcpclient && cargo check) 
(cd .. && cd tcpserver && cargo check)
cd .. && cd GenKey && python -m venv venv 
venv\Scripts\activate && python -m pip install --upgrade pip 
python -m pip install pycryptodome
mkdir keys_client\data && mkdir keys_server\data
del start.sh
```
## How to work with the project:

### Directory tree:


```
.
└── Godwit
    ├──  transmission
    |    ├── serverData
    |    ├── GenKey
    |    |   ├── keys_server
    |    |   |   └── data             
    |    |   ├── keys_client 
    |    |   |   └── data     
    |    |   ├── decryptionKey.py
    |    |   ├── decryptK.sh
    |    |   ├── encryptionKey.py
    |    |   ├── encryptK.sh
    |    |   ├── generateKey.py
    |    |   └── genK.sh
    |    ├── tcpclient
    |    ├── tcpserver
    |    └── count.json
    ├── file.json
    ├── runtcpclient.sh
    ├── runtcpserver.sh
    ├── settings.json
    └── start.sh
```

> [!CAUTION]
> Never delete or change the contents of the following files:
> ~Godwit/settings.json
> ~Godwit/transmission/count.json

### The way the stuff works:

### ! All the files received by the server are saved to the following folder: ```Godwit/transmission/serverData```

> **You can start your server with the following command:**
```sh
./runtcpserver.sh
```

> [!IMPORTANT]
> The server automatically detects the public ip and starts the server on it (if there is such an ip)

> **You can start the client with the following command:**
  ```sh
./runtcpclient.sh
```


### When starting the client, the file accesses the settings.json file
- **ip_recipient** - ip address to which the file is to be sent
- **file_path** - path to **json** file whose data is to be sent
> [!WARNING]
> The file to be sent must be located somewhere in the Godwit directory
>
> The file to be sent must be **json** format in the correct form
- **encryption**: ```True``` / ```False``` - asymmetric encryption function

## You can make a linux daemon for tcpserver:
You should go to the following directory: ```/etc/systemd/system```

Use the command: ```nano tcprunserver.service```
```
[Unit]
Description=Tcpserver
After=network.target

[Service]
ExecStart=/usr/bin/cargo run --bin runtcpserver
WorkingDirectory=/root/Godwit/transmission/tcpserver
User=root
Group=root
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

 ### You can financially support me:)
 
 **Toncoin**: ```UQD1LISMBRa99wn0WJRMw0twu6ayhHm88MJ3dj2Z0RFfGWRR```
 
 **USDT**: ```TBtEL8SPKsjpgSLQYGtkYpSCKm54ogSPz7```
