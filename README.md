[![Build Status](https://travis-ci.org/iCEAGE/dedup_signature.svg?branch=master)](https://travis-ci.org/iCEAGE/dedup_signature)

# Deduplication Signature

Deduplication Signature generates a hash of textual fields for deduplication. Currently it only supports [TextProfileSignature](https://wiki.apache.org/solr/TextProfileSignature).

## Installation

### Cargo

Add this to the Cargo.toml:

    [dependencies]
    dedup_signature = "^0.1.0"


## Getting Started

Please follow the [installation](#installation) procedure and then run the following code:

```rust
extern crate dedup_signature;

use dedup_signature::generator::*;

fn main(){
  let profile_generator = TextProfileSignature{ ..TextProfileSignature::default() };

  let text = r#"Liberty, in philosophy, involves free will as contrasted with determinism.[1] In politics, liberty consists of the social and political freedoms enjoyed by all citizens.[2] In theology, liberty is freedom from the bondage of sin.[3] Generally, liberty seems to be distinct from freedom in that freedom concerns itself primarily, if not exclusively, with the ability to do as one wills and what one has the power to do; whereas liberty also takes into account the rights of all involved. As such, liberty can be thought of as freedom limited by rights, and therefore cannot be abused."#;

  let sign = profile_generator.generate_sign(&text);

  assert_eq!("6274be1f2560d8c9b8d344513d0b3942", sign);
}
```

## Documentation for options

|       Name       |  Type |                                     Description                                    | Default value |
|:----------------:|:-----:|:----------------------------------------------------------------------------------:|---------------|
| min_token_length |  int  | The minimum token length to consider                                               | 2             |
|    quant_rate    | float | When multiplied by the maximum token frequency, this determines count quantization | 0.01          |

## License

Copyright (C) 2016  Hamed Ramezanian Nik

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Lesser General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Lesser General Public License for more details.

You should have received a copy of the GNU Lesser General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
