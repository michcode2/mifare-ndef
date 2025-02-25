import ndef

octets = bytearray.fromhex("d9012d015533687474703a2f2f696d616765732e6d696368636f64652e73706163652f7365616c732f7969707065652e6769666665")
for record in ndef.message_decoder(octets):
    print(record)