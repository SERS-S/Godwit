import os
from Crypto.PublicKey import RSA
from Crypto.Cipher import PKCS1_OAEP
from Crypto.Cipher import AES

def decrypt(dataFile, privateKeyFile):

    with open(privateKeyFile, 'rb') as f:
        privateKey = f.read()
        key = RSA.import_key(privateKey)

    with open(dataFile, 'rb') as f:
        encryptedSessionKey, nonce, tag, ciphertext = [ f.read(x) for x in (key.size_in_bytes(), 16, 16, -1) ]

    cipher = PKCS1_OAEP.new(key)
    sessionKey = cipher.decrypt(encryptedSessionKey)

    cipher = AES.new(sessionKey, AES.MODE_EAX, nonce)
    data = cipher.decrypt_and_verify(ciphertext, tag)

    [ fileName, fileExtension ] = dataFile.split('.')
    decryptedFile = fileName + '_decrypted.' + fileExtension
    with open(decryptedFile, 'wb') as f:
        f.write(data)

with open(f"{os.getcwd()}/keys_server/data/last_ip.txt", 'r') as file:
    last_ip = file.read()
    modified_last_ip = last_ip.replace(".", "_") 

decrypt(f"{os.getcwd()}/keys_server/data/data_{modified_last_ip}.txt", f"{os.getcwd()}/keys_server/private_{last_ip}.pem")