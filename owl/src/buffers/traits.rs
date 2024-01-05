/// A trait to enable buffers to store data as byte representations
pub trait ToByteVec {
    fn to_byte_vec(self) -> Vec<u8>;
}
impl ToByteVec for u8 {
    fn to_byte_vec(self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}
impl ToByteVec for u16 {
    fn to_byte_vec(self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}
impl ToByteVec for u32 {
    fn to_byte_vec(self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}
impl ToByteVec for u64 {
    fn to_byte_vec(self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}
impl ToByteVec for u128 {
    fn to_byte_vec(self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}
impl ToByteVec for i8 {
    fn to_byte_vec(self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}
impl ToByteVec for i16 {
    fn to_byte_vec(self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}
impl ToByteVec for i32 {
    fn to_byte_vec(self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}
impl ToByteVec for i64 {
    fn to_byte_vec(self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}
impl ToByteVec for i128 {
    fn to_byte_vec(self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}
impl ToByteVec for f32 {
    fn to_byte_vec(self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}
impl ToByteVec for f64 {
    fn to_byte_vec(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
impl ToByteVec for bool {
    fn to_byte_vec(self) -> Vec<u8> {
        (self as u8).to_byte_vec()
    }
}
impl<T: ToByteVec, const C: usize> ToByteVec for [T; C] {
    fn to_byte_vec(self) -> Vec<u8> {
        self.into_iter().map(|t| t.to_byte_vec()).flatten().collect()
    }
}
impl<T: ToByteVec> ToByteVec for &[T] {
    fn to_byte_vec(self) -> Vec<u8> {
        self.into_iter().map(|t| t.to_byte_vec()).flatten().collect()
    }
}
impl<T: ToByteVec> ToByteVec for Vec<T> {
    fn to_byte_vec(self) -> Vec<u8> {
        self.as_slice().to_byte_vec()
    }
}

struct PlaceHolder {
    a: u8,
    b: [u8; 12],
}

impl ToByteVec for PlaceHolder {
    fn to_byte_vec(self) -> Vec<u8> {
        let mut v = Vec::new();
        v.extend(self.a.to_byte_vec());
        v.extend(self.b.to_byte_vec());
        v
    }
}
