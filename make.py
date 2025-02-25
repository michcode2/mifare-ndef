import ndef

url = "http://images.michcode.space/seals/idc.gif"

record = ndef.UriRecord(url)
text = b''.join(ndef.message_encoder([record]))
msg_len = int( 3 + len(text.hex())/2).to_bytes(1, 'big').hex() # no clue why that 3 needs to be there but it does!

message = 'd101' + str(msg_len) +'5500'+ bytes(url, 'utf-8').hex()

tlv_prefix = '03' + int(len(message)/2).to_bytes(1, 'big').hex()
print(tlv_prefix)

full_message = tlv_prefix + message + 'fe'

print('hf mf ndefwrite -d ' + full_message)
print(text)