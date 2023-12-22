#!/usr/bin/env bash
chmod +x runtcpserver.sh
echo -e "\033[1m\033[94mThe first file is compiled\033[0m"
sleep 0.75
chmod +x runtcpclient.sh
echo -e "\033[1m\033[94mThe second file is compiled\033[0m"
sleep 0.75
chmod +x transmission/GenKey/genK.sh
echo -e "\033[1m\033[94mThe third file is compiled\033[0m"
sleep 0.75
chmod +x transmission/GenKey/encryptK.sh
echo -e "\033[1m\033[94mThe fourth file is compiled\033[0m"
sleep 0.75
chmod +x transmission/GenKey/decryptK.sh
echo -e "\033[1m\033[94mThe fifth file is compiled\033[0m"
sleep 0.75
cd transmission && cd tcpclient && cargo check && cd .. && cd tcpserver && cargo check
echo -e "\033[1m\033[94mThe cargo environment is configured\033[0m"
sleep 0.75
cd .. && cd GenKey && python3 -m venv venv && source venv/bin/activate && pip install --upgrade pip && pip3 install pycryptodome
echo -e "\033[1m\033[94mThe python environment is configured\033[0m"
cd ../.. && rm -- "$0"
