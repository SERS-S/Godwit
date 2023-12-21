import os
from Crypto.PublicKey import RSA
from Crypto.Cipher import PKCS1_OAEP
from Crypto.Cipher import AES

def encrypt(dataFile, publicKeyFile):

    with open(dataFile, 'rb') as f:
        data = f.read()
    
    data = bytes(data)
 
    with open(publicKeyFile, 'rb') as f:
        publicKey = f.read()
    
    key = RSA.import_key(publicKey)
    sessionKey = os.urandom(16)
 
    cipher = PKCS1_OAEP.new(key)
    encryptedSessionKey = cipher.encrypt(sessionKey)
 
    cipher = AES.new(sessionKey, AES.MODE_EAX)
    ciphertext, tag = cipher.encrypt_and_digest(data)
    []

    [ fileName, fileExtension ] = dataFile.split('.')
    encryptedFile = fileName + '_encryption.' + fileExtension
    with open(encryptedFile, 'wb') as f:
        [ f.write(x) for x in (encryptedSessionKey, cipher.nonce, tag, ciphertext) ]

encrypt(f"{os.getcwd()}/keys_client/data/data.txt", f"{os.getcwd()}/keys_client/public.pem")