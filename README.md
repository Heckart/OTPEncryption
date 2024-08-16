# OTPEncryption

The one time pad is a theoretically "perfect" encryption system. A good explaination of the cipher from can be found [here](https://cs-mst.gitlab.io/index/Classes/Security/Content/06-OneTimePad.html).

## Code

The file `generate_keys.rs` can be run with the command `cargo run --bin=gen_keys <key_file_to_write>`. It will output a binary file of 50 keys, each 2048 bits long, with each bit randomly generated (the file is `50*2048` bits long). These bits are generated randomly, in a cryptographically strong manner, using the "rand" crate (https://docs.rs/rand/latest/rand/).

The file `encrypt.rs` can be run with the command `cargo run -bin=program <key_file_to_read> <key_number> <input_utf8_file> <output_utf8_file>`. It turns the text in `inputfile.txt` into a vector of bytes (vec of u8), based on each character's unicode code point, one byte per character. This program will only work if all character code points are 255 or less, so they can be safely compressed to a byte. It will XOR that byte vector with the bytes of the 4th key in keyfile.sec, convert it back to a unicode string, and write the resulting data to `outputfile.txt`.

Thus, the script should work symmetrically, encrypting or decrypting. The encrypted message may look like ASCII junk, so don’t worry if it does!

## Notes

1. The input file should be exactly 256 ASCII/UTF-8 characters long. This program does not autmatically pad input for you.

2. A vec of u8 might seem like a strange way to handle byte objects, but it seemed like the most canonical way.
