[![Build Status](https://travis-ci.org/iCEAGE/dedup_signature.svg?branch=master)](https://travis-ci.org/iCEAGE/dedup_signature)

# De-duplication Signature

This library implements algorithms to generates a hash/signature/footprint in order to be used for detecting duplicate documents. The algorithms are suitable for long text such as a news article or web page. A signature can be implemented in a few ways:

* [TextProfileSignature](http://wiki.apache.org/solr/TextProfileSignature): Fuzzy hashing implementation from Apache Nutch for near duplicate detection. It's tunable but works best on longer text.
* [Lookup3](http://burtleburtle.net/bob/c/lookup3.c): 64-bit hash used for exact duplicate detection. This is much faster than MD5 and smaller to store.


## Installation

### Cargo

Add this to the Cargo.toml:

    [dependencies]
    dedup_signature = "^0.2.1"


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

## Documentation

### Text Profile Signature
First, you need to import the Text Profile Signature generator:
````rust
extern crate dedup_signature;

use dedup_signature::generator::text_profile_signature::TextProfileSignature;
````

Then you need to create a profile struct with the default parameters:
````rust
let profile_generator = TextProfileSignature{ ..TextProfileSignature::default() };
````

Finally, you can generate your signature by calling the generate_sign method:
````rust
let text = r#"Liberty, in philosophy, involves free will as contrasted with determinism.[1] In politics, liberty consists of the social and political freedoms enjoyed by all citizens.[2] In theology, liberty is freedom from the bondage of sin.[3] Generally, liberty seems to be distinct from freedom in that freedom concerns itself primarily, if not exclusively, with the ability to do as one wills and what one has the power to do; whereas liberty also takes into account the rights of all involved. As such, liberty can be thought of as freedom limited by rights, and therefore cannot be abused."#;

let sign = profile_generator.generate_sign(&text);

assert_eq!("6274be1f2560d8c9b8d344513d0b3942", sign);
````

#### Options 

|       Name       |  Type |                                     Description                                    | Default value |
|:----------------:|:-----:|:----------------------------------------------------------------------------------:|---------------|
| min_token_length |  usize  | The minimum token length to consider                                               | 2             |
|    quant_rate    | f32 | When multiplied by the maximum token frequency, this determines count quantization | 0.01          |


### Lookup3
Lookup3 hash generator is 64-bit hash used for exact duplicate detection. This is much faster than MD5 and smaller to store.

First, you need to import lookup3 generator:
````rust
extern crate dedup_signature;

use dedup_signature::generator::lookup3_signature::Lookup3Signature;
````

Then you need to create a profile struct with the default parameters:
````rust
let profile_generator = Lookup3Signature { ..Lookup3Signature::default() };
````

Finally, you can generate your signature by calling the generate_sign method:
````rust
let text = r#"Liberty, in philosophy, involves free will as contrasted with determinism.[1] In politics, liberty consists of the social and political freedoms enjoyed by all citizens.[2] In theology, liberty is freedom from the bondage of sin.[3] Generally, liberty seems to be distinct from freedom in that freedom concerns itself primarily, if not exclusively, with the ability to do as one wills and what one has the power to do; whereas liberty also takes into account the rights of all involved. As such, liberty can be thought of as freedom limited by rights, and therefore cannot be abused."#;

let sign = profile_generator.generate_sign_64(&text);

assert_eq!("0682d4d013cb3b2c", sign);
````

#### Options 

|       Name       |  Type |                                     Description                                    | Default value |
|:----------------:|:-----:|:----------------------------------------------------------------------------------:|---------------|
| seed |  u64  | The initial seed for generating the hash                                               | 12345



## License

Copyright 2016-2017 Hamed Ramezanian Nik

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
