use cxx::{type_id, ExternType, UniquePtr};
use std::fmt::{Debug, Formatter};

pub struct ExConvCode {
    inner: UniquePtr<ffi::ExConvCodeBridge>,
}

// TODO: Safety...
unsafe impl Send for ffi::ExConvCodeBridge {}
unsafe impl Sync for ffi::ExConvCodeBridge {}

impl ExConvCode {
    pub fn new(
        message_size: u64,
        code_size: u64,
        expander_weight: u64,
        accumulator_weight: u64,
    ) -> Self {
        let mut inner = ffi::new_ex_conv_code();
        inner
            .pin_mut()
            .config(message_size, code_size, expander_weight, accumulator_weight);
        Self { inner }
    }

    pub fn dual_encode_byte(&mut self, e: &mut [u8]) {
        self.inner.pin_mut().dual_encode_byte(e);
    }

    pub fn dual_encode_block(&mut self, e: &mut [Block]) {
        self.inner.pin_mut().dual_encode_block(e);
    }

    ///
    /// # Panics
    /// If e1 is not aligned to a 16-byte boundary.
    pub fn dual_encode2_block(&mut self, e0: &mut [Block], e1: &mut [u8]) {
        assert_eq!(
            0,
            e1.as_ptr() as usize % 16,
            "e1 must be 16-byte aligned. Allocate buffer with aligned-vec"
        );
        self.inner.pin_mut().dual_encode2_block(e0, e1);
    }
}

impl Debug for ExConvCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExConvCode").finish()
    }
}

#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug, Default, Eq, PartialEq)]
#[repr(C, align(16))]
pub struct Block {
    data: u128,
}

unsafe impl ExternType for Block {
    type Id = type_id!("osuCrypto::block");
    type Kind = cxx::kind::Trivial;
}

#[cxx::bridge(namespace = "osuCryptoBridge")]
mod ffi {

    unsafe extern "C++" {
        include!("libote-codes/src/SilentEncoderBridge.h");

        #[namespace = "osuCrypto"]
        #[cxx_name = "block"]
        type Block = super::Block;

        type ExConvCodeBridge;
        #[cxx_name = "newExConvCode"]
        fn new_ex_conv_code() -> UniquePtr<ExConvCodeBridge>;

        fn config(
            self: Pin<&mut ExConvCodeBridge>,
            message_size: u64,
            code_size: u64,
            expander_weight: u64,
            accumulator_size: u64,
        );

        #[cxx_name = "dualEncodeByte"]
        fn dual_encode_byte(self: Pin<&mut ExConvCodeBridge>, e: &mut [u8]);

        #[cxx_name = "dualEncodeBlock"]
        fn dual_encode_block(self: Pin<&mut ExConvCodeBridge>, e: &mut [Block]);

        #[cxx_name = "dualEncode2Block"]
        fn dual_encode2_block(self: Pin<&mut ExConvCodeBridge>, e0: &mut [Block], e1: &mut [u8]);
    }
}

#[cfg(test)]
mod tests {
    use crate::{Block, ExConvCode};
    use std::slice;

    #[test]
    fn new_ex_conv_code() {
        let _code = ExConvCode::new(100, 200, 7, 16);
    }

    #[test]
    fn ex_conv_dual_encode() {
        let mut code = ExConvCode::new(100, 200, 7, 16);
        let mut e = vec![Block::default(); 200];
        code.dual_encode_block(&mut e)
    }

    #[test]
    fn ex_conv_dual_encode2() {
        let mut code = ExConvCode::new(256, 512, 7, 24);
        let mut e0 = vec![Block::default(); 512];
        // create aligned u8 slice
        let mut e1 = vec![Block::default(); 512 / 16];
        let e1 = unsafe { slice::from_raw_parts_mut(e1.as_mut_ptr() as *mut u8, 512) };
        code.dual_encode2_block(&mut e0, e1);
    }
}
