// automatically generated by the FlatBuffers compiler, do not modify
// @generated
extern crate alloc;
use super::*;
// struct Test, aligned to 2
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct Test(pub [u8; 4]);
impl Default for Test { 
  fn default() -> Self { 
    Self([0; 4])
  }
}
impl ::core::fmt::Debug for Test {
  fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
    f.debug_struct("Test")
      .field("a", &self.a())
      .field("b", &self.b())
      .finish()
  }
}

impl ::flatbuffers::SimpleToVerifyInSlice for Test {}
impl<'a> ::flatbuffers::Follow<'a> for Test {
  type Inner = &'a Test;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a Test>::follow(buf, loc)
  }
}
impl<'a> ::flatbuffers::Follow<'a> for &'a Test {
  type Inner = &'a Test;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    ::flatbuffers::follow_cast_ref::<Test>(buf, loc)
  }
}
impl<'b> ::flatbuffers::Push for Test {
    type Output = Test;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        let src = ::core::slice::from_raw_parts(self as *const Test as *const u8, <Self as ::flatbuffers::Push>::size());
        dst.copy_from_slice(src);
    }
    #[inline]
    fn alignment() -> ::flatbuffers::PushAlignment {
        ::flatbuffers::PushAlignment::new(2)
    }
}

impl<'a> ::flatbuffers::Verifiable for Test {
  #[inline]
  fn run_verifier(
    v: &mut ::flatbuffers::Verifier, pos: usize
  ) -> Result<(), ::flatbuffers::InvalidFlatbuffer> {
    use ::flatbuffers::Verifiable;
    v.in_buffer::<Self>(pos)
  }
}

impl<'a> Test {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    a: i16,
    b: i8,
  ) -> Self {
    let mut s = Self([0; 4]);
    s.set_a(a);
    s.set_b(b);
    s
  }

  pub const fn get_fully_qualified_name() -> &'static str {
    "MyGame.Example.Test"
  }

  pub fn a(&self) -> i16 {
    let mut mem = ::core::mem::MaybeUninit::<<i16 as ::flatbuffers::EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    ::flatbuffers::EndianScalar::from_little_endian(unsafe {
      ::core::ptr::copy_nonoverlapping(
        self.0[0..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        ::core::mem::size_of::<<i16 as ::flatbuffers::EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_a(&mut self, x: i16) {
    let x_le = ::flatbuffers::EndianScalar::to_little_endian(x);
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      ::core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[0..].as_mut_ptr(),
        ::core::mem::size_of::<<i16 as ::flatbuffers::EndianScalar>::Scalar>(),
      );
    }
  }

  pub fn b(&self) -> i8 {
    let mut mem = ::core::mem::MaybeUninit::<<i8 as ::flatbuffers::EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    ::flatbuffers::EndianScalar::from_little_endian(unsafe {
      ::core::ptr::copy_nonoverlapping(
        self.0[2..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        ::core::mem::size_of::<<i8 as ::flatbuffers::EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_b(&mut self, x: i8) {
    let x_le = ::flatbuffers::EndianScalar::to_little_endian(x);
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      ::core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[2..].as_mut_ptr(),
        ::core::mem::size_of::<<i8 as ::flatbuffers::EndianScalar>::Scalar>(),
      );
    }
  }

  pub fn unpack(&self) -> TestT {
    TestT {
      a: self.a(),
      b: self.b(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TestT {
  pub a: i16,
  pub b: i8,
}
impl TestT {
  pub fn pack(&self) -> Test {
    Test::new(
      self.a,
      self.b,
    )
  }
}

