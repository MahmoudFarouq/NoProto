// automatically generated by the FlatBuffers compiler, do not modify



use std::mem;
use std::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::EndianScalar;

#[allow(unused_imports, dead_code)]
pub mod benchfb {

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;

#[allow(non_camel_case_types)]
#[repr(i16)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Enum {
  Apples = 0,
  Pears = 1,
  Bananas = 2,

}

pub const ENUM_MIN_ENUM: i16 = 0;
pub const ENUM_MAX_ENUM: i16 = 2;

impl<'a> flatbuffers::Follow<'a> for Enum {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::read_scalar_at::<Self>(buf, loc)
  }
}

impl flatbuffers::EndianScalar for Enum {
  #[inline]
  fn to_little_endian(self) -> Self {
    let n = i16::to_le(self as i16);
    let p = &n as *const i16 as *const Enum;
    unsafe { *p }
  }
  #[inline]
  fn from_little_endian(self) -> Self {
    let n = i16::from_le(self as i16);
    let p = &n as *const i16 as *const Enum;
    unsafe { *p }
  }
}

impl flatbuffers::Push for Enum {
    type Output = Enum;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        flatbuffers::emplace_scalar::<Enum>(dst, *self);
    }
}

#[allow(non_camel_case_types)]
pub const ENUM_VALUES_ENUM:[Enum; 3] = [
  Enum::Apples,
  Enum::Pears,
  Enum::Bananas
];

#[allow(non_camel_case_types)]
pub const ENUM_NAMES_ENUM:[&'static str; 3] = [
    "Apples",
    "Pears",
    "Bananas"
];

pub fn enum_name_enum(e: Enum) -> &'static str {
  let index = e as i16;
  ENUM_NAMES_ENUM[index as usize]
}

// struct Foo, aligned to 8
#[repr(C, align(8))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Foo {
  id_: u64,
  count_: i16,
  prefix_: i8,
  padding0__: u8,
  length_: u32,
} // pub struct Foo
impl flatbuffers::SafeSliceAccess for Foo {}
impl<'a> flatbuffers::Follow<'a> for Foo {
  type Inner = &'a Foo;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a Foo>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a Foo {
  type Inner = &'a Foo;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<Foo>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for Foo {
    type Output = Foo;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const Foo as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b Foo {
    type Output = Foo;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const Foo as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}


impl Foo {
  pub fn new<'a>(_id: u64, _count: i16, _prefix: i8, _length: u32) -> Self {
    Foo {
      id_: _id.to_little_endian(),
      count_: _count.to_little_endian(),
      prefix_: _prefix.to_little_endian(),
      length_: _length.to_little_endian(),

      padding0__: 0,
    }
  }
  pub fn id<'a>(&'a self) -> u64 {
    self.id_.from_little_endian()
  }
  pub fn count<'a>(&'a self) -> i16 {
    self.count_.from_little_endian()
  }
  pub fn prefix<'a>(&'a self) -> i8 {
    self.prefix_.from_little_endian()
  }
  pub fn length<'a>(&'a self) -> u32 {
    self.length_.from_little_endian()
  }
}

// struct Bar, aligned to 8
#[repr(C, align(8))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bar {
  parent_: Foo,
  time_: i32,
  ratio_: f32,
  size__: u16,
  padding0__: u16,  padding1__: u32,
} // pub struct Bar
impl flatbuffers::SafeSliceAccess for Bar {}
impl<'a> flatbuffers::Follow<'a> for Bar {
  type Inner = &'a Bar;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a Bar>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a Bar {
  type Inner = &'a Bar;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<Bar>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for Bar {
    type Output = Bar;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const Bar as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b Bar {
    type Output = Bar;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const Bar as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}


impl Bar {
  pub fn new<'a>(_parent: &'a Foo, _time: i32, _ratio: f32, _size_: u16) -> Self {
    Bar {
      parent_: *_parent,
      time_: _time.to_little_endian(),
      ratio_: _ratio.to_little_endian(),
      size__: _size_.to_little_endian(),

      padding0__: 0,padding1__: 0,
    }
  }
  pub fn parent<'a>(&'a self) -> &'a Foo {
    &self.parent_
  }
  pub fn time<'a>(&'a self) -> i32 {
    self.time_.from_little_endian()
  }
  pub fn ratio<'a>(&'a self) -> f32 {
    self.ratio_.from_little_endian()
  }
  pub fn size_<'a>(&'a self) -> u16 {
    self.size__.from_little_endian()
  }
}

pub enum FooBarOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct FooBar<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for FooBar<'a> {
    type Inner = FooBar<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> FooBar<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        FooBar {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args FooBarArgs<'args>) -> flatbuffers::WIPOffset<FooBar<'bldr>> {
      let mut builder = FooBarBuilder::new(_fbb);
      builder.add_rating(args.rating);
      if let Some(x) = args.name { builder.add_name(x); }
      if let Some(x) = args.sibling { builder.add_sibling(x); }
      builder.add_postfix(args.postfix);
      builder.finish()
    }

    pub const VT_SIBLING: flatbuffers::VOffsetT = 4;
    pub const VT_NAME: flatbuffers::VOffsetT = 6;
    pub const VT_RATING: flatbuffers::VOffsetT = 8;
    pub const VT_POSTFIX: flatbuffers::VOffsetT = 10;

  #[inline]
  pub fn sibling(&self) -> Option<&'a Bar> {
    self._tab.get::<Bar>(FooBar::VT_SIBLING, None)
  }
  #[inline]
  pub fn name(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(FooBar::VT_NAME, None)
  }
  #[inline]
  pub fn rating(&self) -> f64 {
    self._tab.get::<f64>(FooBar::VT_RATING, Some(0.0)).unwrap()
  }
  #[inline]
  pub fn postfix(&self) -> u8 {
    self._tab.get::<u8>(FooBar::VT_POSTFIX, Some(0)).unwrap()
  }
}

pub struct FooBarArgs<'a> {
    pub sibling: Option<&'a  Bar>,
    pub name: Option<flatbuffers::WIPOffset<&'a  str>>,
    pub rating: f64,
    pub postfix: u8,
}
impl<'a> Default for FooBarArgs<'a> {
    #[inline]
    fn default() -> Self {
        FooBarArgs {
            sibling: None,
            name: None,
            rating: 0.0,
            postfix: 0,
        }
    }
}
pub struct FooBarBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> FooBarBuilder<'a, 'b> {
  #[inline]
  pub fn add_sibling(&mut self, sibling: &'b  Bar) {
    self.fbb_.push_slot_always::<&Bar>(FooBar::VT_SIBLING, sibling);
  }
  #[inline]
  pub fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(FooBar::VT_NAME, name);
  }
  #[inline]
  pub fn add_rating(&mut self, rating: f64) {
    self.fbb_.push_slot::<f64>(FooBar::VT_RATING, rating, 0.0);
  }
  #[inline]
  pub fn add_postfix(&mut self, postfix: u8) {
    self.fbb_.push_slot::<u8>(FooBar::VT_POSTFIX, postfix, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> FooBarBuilder<'a, 'b> {
    let start = _fbb.start_table();
    FooBarBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<FooBar<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum FooBarContainerOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct FooBarContainer<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for FooBarContainer<'a> {
    type Inner = FooBarContainer<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> FooBarContainer<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        FooBarContainer {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args FooBarContainerArgs<'args>) -> flatbuffers::WIPOffset<FooBarContainer<'bldr>> {
      let mut builder = FooBarContainerBuilder::new(_fbb);
      if let Some(x) = args.location { builder.add_location(x); }
      if let Some(x) = args.list { builder.add_list(x); }
      builder.add_fruit(args.fruit);
      builder.add_initialized(args.initialized);
      builder.finish()
    }

    pub const VT_LIST: flatbuffers::VOffsetT = 4;
    pub const VT_INITIALIZED: flatbuffers::VOffsetT = 6;
    pub const VT_FRUIT: flatbuffers::VOffsetT = 8;
    pub const VT_LOCATION: flatbuffers::VOffsetT = 10;

  #[inline]
  pub fn list(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<FooBar<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<FooBar<'a>>>>>(FooBarContainer::VT_LIST, None)
  }
  #[inline]
  pub fn initialized(&self) -> bool {
    self._tab.get::<bool>(FooBarContainer::VT_INITIALIZED, Some(false)).unwrap()
  }
  #[inline]
  pub fn fruit(&self) -> Enum {
    self._tab.get::<Enum>(FooBarContainer::VT_FRUIT, Some(Enum::Apples)).unwrap()
  }
  #[inline]
  pub fn location(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(FooBarContainer::VT_LOCATION, None)
  }
}

pub struct FooBarContainerArgs<'a> {
    pub list: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<FooBar<'a >>>>>,
    pub initialized: bool,
    pub fruit: Enum,
    pub location: Option<flatbuffers::WIPOffset<&'a  str>>,
}
impl<'a> Default for FooBarContainerArgs<'a> {
    #[inline]
    fn default() -> Self {
        FooBarContainerArgs {
            list: None,
            initialized: false,
            fruit: Enum::Apples,
            location: None,
        }
    }
}
pub struct FooBarContainerBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> FooBarContainerBuilder<'a, 'b> {
  #[inline]
  pub fn add_list(&mut self, list: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<FooBar<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(FooBarContainer::VT_LIST, list);
  }
  #[inline]
  pub fn add_initialized(&mut self, initialized: bool) {
    self.fbb_.push_slot::<bool>(FooBarContainer::VT_INITIALIZED, initialized, false);
  }
  #[inline]
  pub fn add_fruit(&mut self, fruit: Enum) {
    self.fbb_.push_slot::<Enum>(FooBarContainer::VT_FRUIT, fruit, Enum::Apples);
  }
  #[inline]
  pub fn add_location(&mut self, location: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(FooBarContainer::VT_LOCATION, location);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> FooBarContainerBuilder<'a, 'b> {
    let start = _fbb.start_table();
    FooBarContainerBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<FooBarContainer<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

#[inline]
pub fn get_root_as_foo_bar_container<'a>(buf: &'a [u8]) -> FooBarContainer<'a> {
  flatbuffers::get_root::<FooBarContainer<'a>>(buf)
}

#[inline]
pub fn get_size_prefixed_root_as_foo_bar_container<'a>(buf: &'a [u8]) -> FooBarContainer<'a> {
  flatbuffers::get_size_prefixed_root::<FooBarContainer<'a>>(buf)
}

#[inline]
pub fn finish_foo_bar_container_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<FooBarContainer<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_foo_bar_container_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<FooBarContainer<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
}  // pub mod benchfb
