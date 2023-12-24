# Godwit

## Description:

This is software for sending files using FTP protocol with asymmetric encryption feature (**FTPS**). The data movement technology is based on TCP protocol and continuous connection between client and server. The software is written in ***rust*** and partially in ***python***. The program is written so that everyone can use it regardless of the project language they choose for their language. 



https://github.com/SERS-S/Godwit/assets/104014082/56ef76a5-2a15-4a7e-933d-0cd5709a8a7e



## The following presets are required for the software to work:

- Download [python version 3](https://www.python.org/downloads/)

- Download [rust language](https://www.rust-lang.org/tools/install)

## Set Up project:

### For Unix/Linux platforms:
```sh
git clone https://github.com/SERS-S/Godwit.git
chmod +x start.sh 
./start.sh
```

### For Windows platform:
```sh
SOON!
```
## How to work with the project:

### Schematic of the directory:

> [!CAUTION]
> Never delete or change the contents of the following files:
> ~Godwit/settings.json
> ~Godwit/transmission/count.json

```sh
- Godwit
  -- transmission
    --- serverData
    --- GenKey
      --> keys_server / data
      --> keys_client / data
      --> decryptionKey.py
      --> decryptK.sh
      --> encryptionKey.py
      --> encryptK.sh
      --> generateKey.py
      --> genK.sh
    --- tcpclient
    --- tcpserver
    --- countjson
  -- file.json
  -- runtcpclient.sh
  -- runtcpserver.sh
  -- settings.json
  -- start.sh
```

### How and what works:

### ! All files are saved to the following folder: ```Godwit/transmission/serverData```

> **You can start your server with the following command:**
```sh
./runtcpserver.sh
```

> [!IMPORTANT]
> The server automatically detects the public ip and starts the server on it (if there is such ip)

> **You can start the client with the following command:**
  ```sh
./runtcpclient.sh
```


### When starting the client, the file accesses the settings.json file
- **ip_recipient** - ip address to which the file is to be sent
- **file_path** - path to **json** file whose data is to be sent
> [!WARNING]
> The file must be located anywhere in the Godwit directory
>
> The file must be **json** format in the correct form
- **encryption**: ```True``` / ```False``` - asymmetric encryption function


 ### You can financially support me:)
 
 **Toncoin**: ```UQD1LISMBRa99wn0WJRMw0twu6ayhHm88MJ3dj2Z0RFfGWRR```
 
 **USDT**: ```TBtEL8SPKsjpgSLQYGtkYpSCKm54ogSPz7```
