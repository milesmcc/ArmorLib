//! This module defines the `hash` preprocessor which makes hash information available to
//! scan modules. The following hashes are currently supported:
//! * `md5`
//! * `sha1`
//! * `sha256`
//! * `sha512`
//! * `sha224`
//! * `sha384`
//! * `sha3-256`
//! * `sha3-512`
//! * `sha3-224`
//! * `sha3-384`
//! The hashes are available under the metadata path `hash/name`, and the returned hash will be
//! in lower case.

use std::collections::HashMap;
use crypto::{md5, sha1, sha2, sha3};
use crypto::digest::Digest;

use preprocessor::Preprocessor;
use binary_object::BinaryObject;

pub struct HashPreprocessor;

impl Preprocessor for HashPreprocessor {
    /// Returns a map where the key is the hash type (md5, sha1, sha2, sha3) and the value is
    /// the hash's value, in lower case. The following hashes are provided:
    /// * `md5`
    /// * `sha1`
    /// * `sha256`
    /// * `sha512`
    /// * `sha224`
    /// * `sha384`
    /// * `sha3-256`
    /// * `sha3-512`
    /// * `sha3-224`
    /// * `sha3-384`
    /// (These hash names also indicate the key under which the string values are stored.)
    fn process(&self, binary_object: &BinaryObject) -> HashMap<String, String> {
        let mut map: HashMap<String, String> = HashMap::new();

        // md5
        let mut builder = md5::Md5::new();
        builder.input(&binary_object.data);
        map.insert(String::from("md5"), builder.result_str());

        // sha1
        let mut builder = sha1::Sha1::new();
        builder.input(&binary_object.data);
        map.insert(String::from("sha1"), builder.result_str());

        // sha256
        let mut builder = sha2::Sha256::new();
        builder.input(&binary_object.data);
        map.insert(String::from("sha256"), builder.result_str());

        // sha512
        let mut builder = sha2::Sha512::new();
        builder.input(&binary_object.data);
        map.insert(String::from("sha512"), builder.result_str());

        // sha224
        let mut builder = sha2::Sha224::new();
        builder.input(&binary_object.data);
        map.insert(String::from("sha224"), builder.result_str());

        // sha384
        let mut builder = sha2::Sha384::new();
        builder.input(&binary_object.data);
        map.insert(String::from("sha384"), builder.result_str());

        // sha3 256
        let mut builder = sha3::Sha3::sha3_256();
        builder.input(&binary_object.data);
        map.insert(String::from("sha3-256"), builder.result_str());

        // sha3 512
        let mut builder = sha3::Sha3::sha3_512();
        builder.input(&binary_object.data);
        map.insert(String::from("sha3-512"), builder.result_str());

        // sha3 224
        let mut builder = sha3::Sha3::sha3_224();
        builder.input(&binary_object.data);
        map.insert(String::from("sha3-224"), builder.result_str());

        // sha3 384
        let mut builder = sha3::Sha3::sha3_384();
        builder.input(&binary_object.data);
        map.insert(String::from("sha3-384"), builder.result_str());

        map
    }

    fn info(&self) -> (&'static str, &'static str) {
        ("hash", "determines the hash of the data")
    }
}

#[cfg(test)]
mod tests {
    use preprocessors::hash::HashPreprocessor;
    use preprocessor::Preprocessor;
    use binary_object::BinaryObject;
    use util::hex_to_vec;

    #[test]
    fn test_hashes() {
        let data = hex_to_vec("41 72 6d 6f 72 4c 69 62 20 69 73 20 74 68 65 20 63 6f 6f 6c 65 73 74 20 6c 69 62 72 61 72 79 20 69 6e 20 74 68 65 20 77 6f 72 6c 64 21").unwrap();
        let preprocessor = HashPreprocessor {};
        let map = preprocessor.process(&BinaryObject::from(data));
        assert_eq!(
            map.get(&(String::from("md5"))),
            Some(&String::from("add27d4d270f00204341ce6195f8482e"))
        );
        assert_eq!(
            map.get(&(String::from("sha1"))),
            Some(&String::from("3f12c3aac86060939a1ad230ba4a0536414a15ec"))
        );
        assert_eq!(
            map.get(&(String::from("sha256"))),
            Some(&String::from(
                "2484bcddf142ad693a0ef13a85d4f16d77309b64a247f682061984c55addb5bf"
            ))
        );
        assert_eq!(
            map.get(&(String::from("sha384"))),
            Some(&String::from(
                "7659854a174e1a076e7c25f9094445722a4531f1cb6726e09cef8863391db727ab297a357b36a939a3dd91640eb59d9b"
            ))
        );
        assert_eq!(
            map.get(&(String::from("sha224"))),
            Some(&String::from(
                "337c7934dcc6baec4a191ead196a8b19f0a2a6e49088af36a0f9df01"
            ))
        );
        assert_eq!(
            map.get(&(String::from("sha512"))),
            Some(&String::from(
                "74a29a9ab188d2a78828846277c30f18af27538fe843e79992e9adc72fd6c50b7eded71a260c6498269bf6dbc19ef9310807994d1c83655f5c937aebffda6eac"
            ))
        );
        assert_eq!(
            map.get(&(String::from("sha3-256"))),
            Some(&String::from(
                "abf1e7f87ee4de30130e184908e48cf75d1d4cf9da2845186ea123f8aa40c2ed"
            ))
        );
        assert_eq!(
            map.get(&(String::from("sha3-384"))),
            Some(&String::from(
                 "6665f5e49050f166abb4879d353d7ef1c608281cc2202f87d5b53ef21ab25814362a7c03a2607aa4e0d7d1da66199168"
             ))
         );
        assert_eq!(
            map.get(&(String::from("sha3-224"))),
            Some(&String::from(
                "6730f10daaf05c3ef1d48d33e5f5a33c3724d6f13a9af4774ab41f79"
            ))
        );
        assert_eq!(
            map.get(&(String::from("sha3-512"))),
            Some(&String::from(
                 "1f72957a14b2ff2fc71faaaacf857eddab99d90a3a1639801df138de894e459310ae2513acfba245fcbeabaa2ef0d741637371b586ea88a236077a3aa512c1b7"
             ))
         );
    }
}
