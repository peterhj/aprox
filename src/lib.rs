extern crate half;
extern crate mlx_ffi;
extern crate once_cell;
extern crate smol_str;

use mlx_ffi::*;
use once_cell::sync::{Lazy};
use smol_str::{SmolStr};

use std::cell::{Cell};
use std::collections::{HashMap};
use std::sync::{Arc, Mutex};

static LOC_TABLE: Lazy<Mutex<LocTable>> = Lazy::new(|| {
  let mut tab = LocTable{buf: Vec::new()};
  tab.buf.push(LocDesc::_Nil);
  tab.buf.push(LocDesc::Local);
  Mutex::new(tab)
});

pub struct LocTable {
  buf:  Vec<LocDesc>,
}

#[derive(Clone, Copy, Debug)]
pub enum LocDesc {
  _Nil,
  Local,
  // TODO: same host, multiple links.
  Remote{addr: u32, port: u16},
}

pub struct PCache {
  nmap: HashMap<SmolStr, (u64, Arc<dyn PArray>)>,
  map:  HashMap<u64, Arc<dyn PArray>>,
}

pub trait PArray {
}

pub struct DummyPArray {
}

impl PArray for DummyPArray {
}

pub struct ProxyPArray {
  loc:  u64,
  key:  u64,
}

impl PArray for ProxyPArray {
}

pub struct MlxPArray {
  inner: MlxArray,
}

impl PArray for MlxPArray {
}

thread_local! {
  static TL_EVAL_CTX: EvalCtx = EvalCtx::new();
}

pub struct EvalCtx {
  trace: Cell<bool>,
}

impl EvalCtx {
  pub fn new() -> EvalCtx {
    EvalCtx{
      trace: Cell::new(false),
    }
  }
}

pub struct SpineEntry {
  op:   Arc<dyn Op>,
  args: Vec<u64>,
  outs: Vec<u64>,
}

pub struct Spine {
  ctr:  u64,
  free: Vec<u64>,
  env:  HashMap<u64, ()>,
  log:  Vec<SpineEntry>,
}

pub struct OpTable {
}

pub trait Op {
  //fn _name(&self) -> &'static str;
}

pub struct Function {
}

pub struct Array {
  phy:  Arc<dyn PArray>,
}

impl Array {
  pub fn put_loc(&self, loc: ()) -> () {
  }
}
