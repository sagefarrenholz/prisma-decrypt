# Prisma Field Encryption Decrypt

A fast cli tool that decrypts data encrypted in your database by the prisma
middleware
[prisma-field-encryption](https://github.com/47ng/prisma-field-encryption#readme)

## Installation

```sh
brew tap sagefarrenholz/prisma-decrypt
brew install prisma-decrypt
```

or via cargo:

```sh
cargo install prisma-decrypt
```

# Usage

Using `PRISMA_FIELD_ENCRYPTION_KEY` environment variable:

```sh
prisma-decrypt <ENCRYPTED_DATA>
```

Manually specifying decryption key:

```sh
prisma-decrypt --prisma-key <PRISMA_FIELD_ENCRYPTION_KEY> <ENCRYPTED_DATA>
```

## Encoding Output

You can also specify the encoding output (by default utf-8):

```sh
prisma-decrypt --encoding hex <ENCRYPTED_DATA>
prisma-decrypt --encoding base64 <ENCRYPTED_DATA>
```

## Notes

Alternatively, you could use cloak https://www.npmjs.com/package/@47ng/cloak
(`prisma-field-encryption` uses this under the hood), but I built this to be a
bit faster and give more flexibility
