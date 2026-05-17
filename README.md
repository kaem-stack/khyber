# khyber

KAEM's encryption binary. Handles key generation, encryption, and decryption across pluggable cryptographic backends. Currently uses CRYSTALS-Kyber (ML-KEM-768) for key encapsulation paired with ChaCha20-Poly1305 for symmetric encryption.

## Build

```bash
cargo build --release
```

The binary is at `target/release/khyber`.

## Commands

### generate-keys

Generate an ML-KEM-768 keypair and write it to a directory.

```bash
khyber generate-keys --out <DIR>
```

| Flag | Short | Description |
|---|---|---|
| `--out` | `-o` | Directory to write key files into *(required)* |
| `--algorithm` | | Algorithm to use. Default: `ml-kem-768` |

**Output files:**
- `<DIR>/khyber.pub` — encapsulation key (public, 1184 bytes)
- `<DIR>/khyber.key` — decapsulation key (secret, 64 bytes)

**Example:**
```bash
khyber generate-keys --out ~/.khyber/
```

---

### encrypt

Encrypt a message using a public key. Outputs base64-encoded ciphertext.

```bash
khyber encrypt <MESSAGE> --key <PUB_KEY_FILE> [--output <FILE>]
```

| Argument / Flag | Short | Description |
|---|---|---|
| `<MESSAGE>` | | Plaintext message to encrypt *(required)* |
| `--key` | `-k` | Path to the public key file (`khyber.pub`) *(required)* |
| `--output` | `-o` | Write ciphertext to file instead of stdout |

**Example:**
```bash
# Print ciphertext to stdout
khyber encrypt "hello world" --key ~/.khyber/khyber.pub

# Write to a file
khyber encrypt "hello world" --key ~/.khyber/khyber.pub --output message.enc
```

---

### decrypt

Decrypt a base64-encoded ciphertext using the secret key.

```bash
khyber decrypt <CIPHERTEXT> --key <SECRET_KEY_FILE> [--output <FILE>]
```

| Argument / Flag | Short | Description |
|---|---|---|
| `<CIPHERTEXT>` | | Base64-encoded ciphertext to decrypt *(required)* |
| `--key` | `-k` | Path to the secret key file (`khyber.key`) *(required)* |
| `--output` | `-o` | Write plaintext to file instead of stdout |

**Example:**
```bash
# Decrypt to stdout
khyber decrypt <CIPHERTEXT> --key ~/.khyber/khyber.key

# Decrypt to a file
khyber decrypt <CIPHERTEXT> --key ~/.khyber/khyber.key --output message.txt
```

---

## Full example

```bash
# 1. Generate a keypair
khyber generate-keys --out ~/.khyber/

# 2. Encrypt a message
khyber encrypt "hello world" --key ~/.khyber/khyber.pub

# 3. Decrypt it
khyber decrypt <ciphertext> --key ~/.khyber/khyber.key

# Round trip in one line
khyber decrypt $(khyber encrypt "hello world" --key ~/.khyber/khyber.pub) --key ~/.khyber/khyber.key
```

---

## Wire format

The encrypted output is base64-encoded and contains:

```
[ ML-KEM-768 ciphertext: 1088 bytes ]
[ ChaCha20-Poly1305 nonce:  12 bytes ]
[ Encrypted message + auth tag       ]
```

The KEM ciphertext size is fixed, so no length prefix is needed to parse the frame.
