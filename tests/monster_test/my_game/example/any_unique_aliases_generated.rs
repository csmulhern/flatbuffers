// automatically generated by the FlatBuffers compiler, do not modify
// @generated
extern crate alloc;
extern crate flatbuffers;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::mem;
use core::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_ANY_UNIQUE_ALIASES: u8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_ANY_UNIQUE_ALIASES: u8 = 3;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_ANY_UNIQUE_ALIASES: [AnyUniqueAliases; 4] = [
  AnyUniqueAliases::NONE,
  AnyUniqueAliases::M,
  AnyUniqueAliases::TS,
  AnyUniqueAliases::M2,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct AnyUniqueAliases(pub u8);
#[allow(non_upper_case_globals)]
impl AnyUniqueAliases {
  pub const NONE: Self = Self(0);
  pub const M: Self = Self(1);
  pub const TS: Self = Self(2);
  pub const M2: Self = Self(3);

  pub const ENUM_MIN: u8 = 0;
  pub const ENUM_MAX: u8 = 3;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::NONE,
    Self::M,
    Self::TS,
    Self::M2,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::NONE => Some("NONE"),
      Self::M => Some("M"),
      Self::TS => Some("TS"),
      Self::M2 => Some("M2"),
      _ => None,
    }
  }
}
impl core::fmt::Debug for AnyUniqueAliases {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for AnyUniqueAliases {
  type Inner = Self;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = unsafe { flatbuffers::read_scalar_at::<u8>(buf, loc) };
    Self(b)
  }
}

impl flatbuffers::Push for AnyUniqueAliases {
    type Output = AnyUniqueAliases;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        unsafe { flatbuffers::emplace_scalar::<u8>(dst, self.0); }
    }
}

impl flatbuffers::EndianScalar for AnyUniqueAliases {
  type Scalar = u8;
  #[inline]
  fn to_little_endian(self) -> u8 {
    self.0.to_le()
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(v: u8) -> Self {
    let b = u8::from_le(v);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for AnyUniqueAliases {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    u8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for AnyUniqueAliases {}
pub struct AnyUniqueAliasesUnionTableOffset {}

#[allow(clippy::upper_case_acronyms)]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum AnyUniqueAliasesT {
  NONE,
  M(Box<MonsterT>),
  TS(Box<TestSimpleTableWithEnumT>),
  M2(Box<super::example_2::MonsterT>),
}
impl Default for AnyUniqueAliasesT {
  fn default() -> Self {
    Self::NONE
  }
}
impl AnyUniqueAliasesT {
  pub fn any_unique_aliases_type(&self) -> AnyUniqueAliases {
    match self {
      Self::NONE => AnyUniqueAliases::NONE,
      Self::M(_) => AnyUniqueAliases::M,
      Self::TS(_) => AnyUniqueAliases::TS,
      Self::M2(_) => AnyUniqueAliases::M2,
    }
  }
  pub fn pack<'b, A: flatbuffers::Allocator + 'b>(&self, fbb: &mut flatbuffers::FlatBufferBuilder<'b, A>) -> Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>> {
    match self {
      Self::NONE => None,
      Self::M(v) => Some(v.pack(fbb).as_union_value()),
      Self::TS(v) => Some(v.pack(fbb).as_union_value()),
      Self::M2(v) => Some(v.pack(fbb).as_union_value()),
    }
  }
  /// If the union variant matches, return the owned MonsterT, setting the union to NONE.
  pub fn take_m(&mut self) -> Option<Box<MonsterT>> {
    if let Self::M(_) = self {
      let v = core::mem::replace(self, Self::NONE);
      if let Self::M(w) = v {
        Some(w)
      } else {
        unreachable!()
      }
    } else {
      None
    }
  }
  /// If the union variant matches, return a reference to the MonsterT.
  pub fn as_m(&self) -> Option<&MonsterT> {
    if let Self::M(v) = self { Some(v.as_ref()) } else { None }
  }
  /// If the union variant matches, return a mutable reference to the MonsterT.
  pub fn as_m_mut(&mut self) -> Option<&mut MonsterT> {
    if let Self::M(v) = self { Some(v.as_mut()) } else { None }
  }
  /// If the union variant matches, return the owned TestSimpleTableWithEnumT, setting the union to NONE.
  pub fn take_ts(&mut self) -> Option<Box<TestSimpleTableWithEnumT>> {
    if let Self::TS(_) = self {
      let v = core::mem::replace(self, Self::NONE);
      if let Self::TS(w) = v {
        Some(w)
      } else {
        unreachable!()
      }
    } else {
      None
    }
  }
  /// If the union variant matches, return a reference to the TestSimpleTableWithEnumT.
  pub fn as_ts(&self) -> Option<&TestSimpleTableWithEnumT> {
    if let Self::TS(v) = self { Some(v.as_ref()) } else { None }
  }
  /// If the union variant matches, return a mutable reference to the TestSimpleTableWithEnumT.
  pub fn as_ts_mut(&mut self) -> Option<&mut TestSimpleTableWithEnumT> {
    if let Self::TS(v) = self { Some(v.as_mut()) } else { None }
  }
  /// If the union variant matches, return the owned super::example_2::MonsterT, setting the union to NONE.
  pub fn take_m2(&mut self) -> Option<Box<super::example_2::MonsterT>> {
    if let Self::M2(_) = self {
      let v = core::mem::replace(self, Self::NONE);
      if let Self::M2(w) = v {
        Some(w)
      } else {
        unreachable!()
      }
    } else {
      None
    }
  }
  /// If the union variant matches, return a reference to the super::example_2::MonsterT.
  pub fn as_m2(&self) -> Option<&super::example_2::MonsterT> {
    if let Self::M2(v) = self { Some(v.as_ref()) } else { None }
  }
  /// If the union variant matches, return a mutable reference to the super::example_2::MonsterT.
  pub fn as_m2_mut(&mut self) -> Option<&mut super::example_2::MonsterT> {
    if let Self::M2(v) = self { Some(v.as_mut()) } else { None }
  }
}
