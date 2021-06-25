use std::rc::Rc;
use std::sync::Arc;
use std::iter::Iterator;

use crate::prelude::{ProtoEntry};

macro_rules! impl_line_slice {
    ($ref_type:ident, $line_name:ident, $slice_name:ident) => {
        #[derive(Clone)]
        pub struct $line_name {
            pub entries: Vec<$ref_type<ProtoEntry>>,
        }
 
        #[derive(Clone)]
        pub struct $slice_name {
            pub line: $ref_type<$line_name>,
            pub index: usize,
            pub count: usize,
        }

        impl $line_name {
            pub fn new(entries: Vec<$ref_type<ProtoEntry>>) -> Self {
                Self {entries}
            }

            pub fn get_slice(self, index: usize, count: usize) -> $slice_name {
                $slice_name {
                    line: $ref_type::new(self),
                    index,
                    count,
                }
            }
        }

        impl From<Vec<ProtoEntry>> for $line_name {
            fn from(v: Vec<ProtoEntry>) -> Self {
                let entries : Vec<$ref_type<ProtoEntry>> = v.into_iter().map(
                    |entry| {
                        $ref_type::new(entry)
                    }
                ).collect();
                Self::new(entries)
            }
        }

        impl From<Vec<$ref_type<ProtoEntry>>> for $line_name {
            fn from(v: Vec<$ref_type<ProtoEntry>>) -> Self {
                Self::new(v)
            }
        }

        impl $line_name {
            pub fn from_iterator(iter: impl Iterator<Item=ProtoEntry>) -> Self {
                iter.collect::<Vec<ProtoEntry>>().into()
            }
            pub fn from_entries(iter: impl Iterator<Item=$ref_type<ProtoEntry>>) -> Self {
                iter.collect::<Vec<$ref_type<ProtoEntry>>>().into()
            }
        }

        impl From<Vec<$line_name>> for $line_name {
            fn from(v: Vec<$line_name>) -> Self {
                let mut entries = Vec::<$ref_type<ProtoEntry>>::new();
                for x in v {
                    entries.append(&mut x.entries.clone());
                }
                Self::new(entries)
            }
        }

        impl $line_name {
            pub fn from_lines_iterator(iter: impl Iterator<Item=$line_name>) -> Self {
                iter.collect::<Vec<$line_name>>().into()
            }
        }
    }
}

impl_line_slice!(Rc, RcLine, RcSlice);
impl_line_slice!(Arc, ArcLine, ArcSlice);
