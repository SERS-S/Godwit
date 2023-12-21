from Crypto.PublicKey import RSA
import os
current_directory = os.getcwd()
key = RSA.generate(2048)
privateKey = key.export_key()
publicKey = key.publickey().export_key()

ip_data = open(f"{os.getcwd()}/keys_server/currentAddr.txt").readline()
# save private key to file
with open(f'{os.getcwd()}/keys_server/private_{ip_data}.pem', 'wb') as f:
    f.write(privateKey)
 
# save public key to file
with open(f'{os.getcwd()}/keys_server/public_{ip_data}.pem', 'wb') as f:
    f.write(publicKey)
os.remove(f"{os.getcwd()}/keys_server/currentAddr.txt")