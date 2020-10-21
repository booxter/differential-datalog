#![allow(
    unused_imports,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    unused_parens,
    non_shorthand_field_patterns,
    dead_code,
    overflowing_literals,
    unreachable_patterns,
    unused_variables,
    clippy::unknown_clippy_lints,
    clippy::missing_safety_doc
)]

use ::std::convert::TryFrom;
use ::std::ffi;
use ::std::fmt;
use ::std::hash::Hash;
use ::std::result;

use ::serde::Deserialize;
use ::serde::Serialize;

use ::differential_datalog::ddval::*;
use ::differential_datalog::int::*;
use ::differential_datalog::program::*;
use ::differential_datalog::record;
use ::differential_datalog::record::FromRecord;
use ::differential_datalog::record::IntoRecord;
use ::differential_datalog::record::RelIdentifier;
use ::differential_datalog::uint::*;

use ::fnv::FnvHashMap;
use ::once_cell::sync::Lazy;
use ::ordered_float::OrderedFloat;

use ::types::closure;

/* FlatBuffers bindings generated by `ddlog` */
#[cfg(feature = "flatbuf")]
pub mod flatbuf;

impl TryFrom<&RelIdentifier> for Relations {
    type Error = ();

    fn try_from(rel_id: &RelIdentifier) -> result::Result<Self, ()> {
        match rel_id {
            RelIdentifier::RelName(rname) => Relations::try_from(rname.as_ref()),
            RelIdentifier::RelId(id) => Relations::try_from(*id),
        }
    }
}

/*- !!!!!!!!!!!!!!!!!!!! -*/
// Don't edit this line
// Code below this point is needed to test-compile template
// code and is not part of the template.

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Relations {
    X = 0,
}

#[allow(clippy::trivially_copy_pass_by_ref)]
impl Relations {
    pub fn is_input(&self) -> bool {
        panic!("Relations::is_input not implemented")
    }

    pub fn is_output(&self) -> bool {
        panic!("Relations::is_output not implemented")
    }
}

impl TryFrom<&str> for Relations {
    type Error = ();

    fn try_from(rname: &str) -> result::Result<Self, ()> {
        panic!("Relations::try_from::<&str> not implemented")
    }
}

impl TryFrom<RelId> for Relations {
    type Error = ();

    fn try_from(rid: RelId) -> result::Result<Self, ()> {
        panic!("Relations::try_from::<RelId> not implemented")
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Indexes {
    X = 0,
}

impl TryFrom<&str> for Indexes {
    type Error = ();

    fn try_from(_iname: &str) -> result::Result<Self, ()> {
        panic!("Indexes::try_from::<&str> not implemented")
    }
}

impl TryFrom<IdxId> for Indexes {
    type Error = ();

    fn try_from(_iid: IdxId) -> result::Result<Self, ()> {
        panic!("Indexes::try_from::<IdxId> not implemented")
    }
}

pub fn relval_from_record(
    _rel: Relations,
    _rec: &record::Record,
) -> result::Result<DDValue, String> {
    panic!("relval_from_record not implemented")
}

pub fn relkey_from_record(
    _rel: Relations,
    _rec: &record::Record,
) -> result::Result<DDValue, String> {
    panic!("relkey_from_record not implemented")
}

pub fn idxkey_from_record(idx: Indexes, _rec: &record::Record) -> result::Result<DDValue, String> {
    panic!("idxkey_from_record not implemented")
}

pub fn relid2name(_rid: RelId) -> Option<&'static str> {
    panic!("relid2name not implemented")
}

pub fn relid2cname(_rid: RelId) -> Option<&'static ffi::CStr> {
    panic!("relid2cname not implemented")
}

pub static RELIDMAP: Lazy<FnvHashMap<Relations, &'static str>> = Lazy::new(FnvHashMap::default);
pub static INPUT_RELIDMAP: Lazy<FnvHashMap<Relations, &'static str>> =
    Lazy::new(FnvHashMap::default);
pub static OUTPUT_RELIDMAP: Lazy<FnvHashMap<Relations, &'static str>> =
    Lazy::new(FnvHashMap::default);

pub fn indexid2name(_iid: IdxId) -> Option<&'static str> {
    panic!("indexid2name not implemented")
}

pub fn indexid2cname(_iid: IdxId) -> Option<&'static ffi::CStr> {
    panic!("indexid2cname not implemented")
}

pub fn indexes2arrid(idx: Indexes) -> ArrId {
    panic!("indexes2arrid not implemented")
}

pub static IDXIDMAP: Lazy<FnvHashMap<Indexes, &'static str>> = Lazy::new(FnvHashMap::default);
