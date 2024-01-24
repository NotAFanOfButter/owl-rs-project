#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bytes(pub usize);

/// A trait to enable buffers to store data as byte representations
pub trait ToByteVec {
    fn to_byte_vec(self) -> Vec<u8>;
    fn stride(&self) -> Bytes;
    #[allow(unused_variables)]
    fn field_offset(&self, field_index: usize) -> Option<Bytes> {
        None
    }
}
impl ToByteVec for u8 {
    fn to_byte_vec(self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
    fn stride(&self) -> Bytes {
        Bytes(1)
    }
}
impl ToByteVec for u16 {
    fn to_byte_vec(self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
    fn stride(&self) -> Bytes {
        Bytes(2)
    }
}
impl ToByteVec for u32 {
    fn to_byte_vec(self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
    fn stride(&self) -> Bytes {
        Bytes(4)
    }
}
impl ToByteVec for u64 {
    fn to_byte_vec(self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
    fn stride(&self) -> Bytes {
        Bytes(8)
    }
}
impl ToByteVec for u128 {
    fn to_byte_vec(self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
    fn stride(&self) -> Bytes {
        Bytes(16)
    }
}
impl ToByteVec for i8 {
    fn to_byte_vec(self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
    fn stride(&self) -> Bytes {
        Bytes(1)
    }
}
impl ToByteVec for i16 {
    fn to_byte_vec(self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
    fn stride(&self) -> Bytes {
        Bytes(2)
    }
}
impl ToByteVec for i32 {
    fn to_byte_vec(self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
    fn stride(&self) -> Bytes {
        Bytes(4)
    }
}
impl ToByteVec for i64 {
    fn to_byte_vec(self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
    fn stride(&self) -> Bytes {
        Bytes(8)
    }
}
impl ToByteVec for i128 {
    fn to_byte_vec(self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
    fn stride(&self) -> Bytes {
        Bytes(16)
    }
}
impl ToByteVec for f32 {
    fn to_byte_vec(self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
    fn stride(&self) -> Bytes {
        Bytes(4)
    }
}
impl ToByteVec for f64 {
    fn to_byte_vec(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
    fn stride(&self) -> Bytes {
        Bytes(8)
    }
}
impl ToByteVec for bool {
    fn to_byte_vec(self) -> Vec<u8> {
        u8::from(self).to_byte_vec()
    }
    fn stride(&self) -> Bytes {
        Bytes(1)
    }
}
impl<T: ToByteVec, const C: usize> ToByteVec for [T; C] {
    fn to_byte_vec(self) -> Vec<u8> {
        self.into_iter().flat_map(ToByteVec::to_byte_vec).collect()
    }
    fn stride(&self) -> Bytes {
        if C > 0 {
            Bytes(self[0].stride().0 * C)
        } else {
            Bytes(0)
        }
    }
}
impl<T: ToByteVec> ToByteVec for Vec<T> {
    fn to_byte_vec(self) -> Vec<u8> {
        self.into_iter().flat_map(ToByteVec::to_byte_vec).collect()
    }
    fn stride(&self) -> Bytes {
        let len = self.len();
        if len > 0 {
            Bytes(self[0].stride().0 * len)
        } else {
            Bytes(0)
        }
    }
}

use trait_derives::ToByteVec;
#[derive(ToByteVec)]
struct Tester;
