# ethdenver2024

## Run

In root path, run
```shell
RISC0_DEV_MODE=1 cargo run --release
```

## Project ideas

Idea 1:

Verify ChatGPT output with JSON API response verification.

Idea 2:

Verify credit score based on chain loan data to save gas.

Idea 3:

Verify total trading volume on DEXs for HarryPotterObamaSonic10Inu (ETH) to get listed on CEXs. 

## Idea 1 Next Steps

get an enum
ordered list of messages, either client or server

start - 
decapsulate a tls record

- [x]
17 03 03 - always first bytes
u16 big endian length field - check that it matches
rest if cyphertext
last 16 bytes are authtag, but 
pass the rest in

- [ ] 
(assuming we know the key)
when we decrypt it

- [ ] 
need aes 128 gcm library to decrypt it
feed it the key and bag of bytes
and will get back plaintext

- [ ] 
once you have plaintext -
   - [ ] 
   strip any trailing 0 bytes
   - [ ]
  strip last nonzero byte - check that it's equl to 17 (hex) which is tag for application data

will be http traffic

- [ ] 
inputs to aes gcm function
   {client}  derive write traffic keys for application data:

      PRK (32 octets):  9e 40 64 6c e7 9a 7f 9d c0 5a f8 88 9b ce 65 52
         87 5a fa 0b 06 df 00 87 f7 92 eb b7 c1 75 04 a5

      key info (13 octets):  00 10 09 74 6c 73 31 33 20 6b 65 79 00

      key expanded (16 octets):  17 42 2d da 59 6e d5 d9 ac d8 90 e3 c6
         3f 50 51

      iv info (12 octets):  00 0c 08 74 6c 73 31 33 20 69 76 00

      iv expanded (12 octets):  5b 78 92 3d ee 08 57 90 33 e5 23 d9



   {client}  send application_data record:

      payload (50 octets):  00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e
         0f 10 11 12 13 14 15 16 17 18 19 1a 1b 1c 1d 1e 1f 20 21 22 23
         24 25 26 27 28 29 2a 2b 2c 2d 2e 2f 30 31

      complete record (72 octets):  17 03 03 00 43 a2 3f 70 54 b6 2c 94
         d0 af fa fe 82 28 ba 55 cb ef ac ea 42 f9 14 aa 66 bc ab 3f 2b
         98 19 a8 a5 b4 6b 39 5b d5 4a 9a 20 44 1e 2b 62 97 4e 1f 5a 62
         92 a2 97 70 14 bd 1e 3d ea e6 3a ee bb 21 69 49 15 e4



