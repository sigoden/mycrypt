# mycrypt

Encrypt/decrypt your file with aes256

## Usage

```sh
# encrypt 
mycrpt encrypt -f sec.txt sec.bin
mycrpt encrypt sec.bin

# decrypt
mycrypt decrypt -o sec.txt sec.bin
mycrypt decrypt sec.bin > sec.txt
```