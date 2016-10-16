use NibbleVec;
use endian_type::{LittleEndian, BigEndian};

/// Trait for types which can be used to key a Radix Trie.
///
/// Types that implement this trait should be convertible to a vector of bytes
/// such that no two instances of the type convert to the same vector. This is essentially
/// serialisation, and may be combined with some serialisation library in the future.
///
/// If a type fails to implement this trait correctly, the Radix Trie will panic upon
/// encountering a conflict. Be careful!
pub trait TrieKey: PartialEq + Eq {
    /// Encode a value as a vector of bytes.
    fn encode_bytes(&self) -> Vec<u8> {
        panic!("implement this method or TrieKey::encode");
    }

    /// Encode a value as a NibbleVec.
    fn encode(&self) -> NibbleVec {
        NibbleVec::from_byte_vec(self.encode_bytes())
    }
}

/// Key comparison result.
#[derive(Debug)]
pub enum KeyMatch {
    /// The keys match up to the given index.
    Partial(usize),
    /// The first key is a prefix of the second.
    FirstPrefix,
    /// The second key is a prefix of the first.
    SecondPrefix,
    /// The keys match exactly.
    Full,
}

/// Compare two Trie keys.
pub fn match_keys(start_idx: usize, first: &NibbleVec, second: &NibbleVec) -> KeyMatch {
    let first_len = first.len() - start_idx;
    let min_length = ::std::cmp::min(first_len, second.len());

    for i in 0..min_length {
        if first.get(start_idx + i) != second.get(i) {
            return KeyMatch::Partial(i);
        }
    }

    match (first_len, second.len()) {
        (x, y) if x < y => KeyMatch::FirstPrefix,
        (x, y) if x == y => KeyMatch::Full,
        _ => KeyMatch::SecondPrefix,
    }
}

/// Check two keys for equality and panic if they differ.
pub fn check_keys<K>(key1: &K, key2: &K)
    where K: TrieKey
{
    if *key1 != *key2 {
        panic!("multiple-keys with the same bit representation.");
    }
}

/// --- TrieKey Implementations for standard types --- ///

// This blanket implementation goes into play when specialization is stabilized
// impl<T> TrieKey for T where T: Into<Vec<u8>> + Clone + Eq + PartialEq {
// fn encode_bytes(&self) -> Vec<u8> {
// self.clone().into()
// }
// }


impl TrieKey for Vec<u8> {
    fn encode_bytes(&self) -> Vec<u8> {
        self.clone()
    }
}

impl<'a> TrieKey for &'a [u8] {
    fn encode_bytes(&self) -> Vec<u8> {
        self.clone().to_vec()
    }
}

impl TrieKey for String {
    fn encode_bytes(&self) -> Vec<u8> {
        self.as_bytes().encode_bytes()
    }
}

impl<'a> TrieKey for &'a str {
    fn encode_bytes(&self) -> Vec<u8> {
        self.as_bytes().encode_bytes()
    }
}

impl TrieKey for i8 {
    fn encode_bytes(&self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::with_capacity(1);
        v.push(*self as u8);
        return v;
    }
}

impl TrieKey for u8 {
    fn encode_bytes(&self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::with_capacity(1);
        v.push(*self);
        return v;
    }
}

impl<T> TrieKey for LittleEndian<T>
    where T: Eq + Copy
{
    fn encode_bytes(&self) -> Vec<u8> {
        self.as_bytes().encode_bytes()
    }
}

impl<T> TrieKey for BigEndian<T>
    where T: Eq + Copy
{
    fn encode_bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

macro_rules! int_keys {
    ( $( $t:ty ),* ) => {
        $(
        impl TrieKey for $t {
            fn encode_bytes(&self) -> Vec<u8> {
                let be: BigEndian<$t> = From::from(*self);
                be.encode_bytes()
            }
        }
        )*
    };
}

int_keys!(u16, u32, u64, i16, i32, i64, usize, isize);
