import base64
import binascii
from dataprocessor import DataProcessor
from blockcoder import BlockCoder

class Base32Encoder(BlockCoder):
    def __init__(self):
        super().__init__(base64.b32encode, 5, True)

class Base32Decoder(BlockCoder):
    def __init__(self):
        super().__init__(base64.b32decode, 8, True)


class Base64Encoder(BlockCoder):
    def __init__(self):
        super().__init__(base64.b64encode, 3, True)

class Base64Decoder(BlockCoder):
    def __init__(self):
        super().__init__(base64.b64decode, 4, True)


class Base85Encoder(BlockCoder):
    def __init__(self):
        super().__init__(base64.b85encode, 4, True)

class Base85Decoder(BlockCoder):
    def __init__(self):
        super().__init__(base64.b85decode, 5, True)


class Ascii85Encoder(BlockCoder):
    def __init__(self):
        super().__init__(base64.a85encode, 4, True)

class Ascii85Decoder(BlockCoder):
    def __init__(self):
        super().__init__(base64.a85decode, 5, True)


class HexEncoder(DataProcessor):
    def process(self, data: memoryview) -> memoryview:
        return binascii.hexlify(data)
    def flush(self) -> memoryview:
        return bytes()

class HexDecoder(BlockCoder):
    def __init__(self):
        super().__init__(binascii.unhexlify, 2, False)
