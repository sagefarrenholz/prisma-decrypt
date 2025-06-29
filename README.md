# prisma-decrypt

<div align="center">

[![Crates.io](https://img.shields.io/crates/v/prisma-decrypt)](https://crates.io/crates/prisma-decrypt)
[![Build Status](https://img.shields.io/github/actions/workflow/status/sagefarrenholz/prisma-decrypt/release.yml)](https://github.com/sagefarrenholz/prisma-decrypt/actions)
[![License](https://img.shields.io/crates/l/prisma-decrypt)](https://github.com/sagefarrenholz/prisma-decrypt#license)

**CLI tool for super fast prisma-field-encryption column decryption**

</div>

A fast cli tool that decrypts data encrypted in your database by the prisma
middleware
[prisma-field-encryption](https://github.com/47ng/prisma-field-encryption#readme)

## Installation

```sh
brew tap sagefarrenholz/prisma-decrypt https://github.com/sagefarrenholz/prisma-decrypt.git
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
