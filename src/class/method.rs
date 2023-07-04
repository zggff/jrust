use bitflags::bitflags;

use crate::source::ByteStream;

use super::{attribute::AttributeInfo, constant_pool::CpPool};

#[derive(Debug, Clone)]
pub struct MethodInfo {
    pub access_flags: MethodAccessFlags,
    pub name: String,
    pub descriptor: String,
    pub name_index: usize,
    pub descriptor_index: usize,
    pub attributes: Vec<AttributeInfo>,
}

impl MethodInfo {
    pub fn parse(f: &mut ByteStream, cp: &CpPool) -> Option<Self> {
        let access_flags = MethodAccessFlags { bits: f.next_u2()? };
        let name_index = f.next_u2()?;
        let descriptor_index = f.next_u2()?;
        let attribute_count = f.next_u2()?;

        let name = cp.get_utf(name_index)?.bytes.clone();
        let descriptor = cp.get_utf(descriptor_index)?.bytes.clone();

        let mut attributes = Vec::with_capacity(attribute_count);
        for _ in 0..attribute_count {
            attributes.push(AttributeInfo::parse(f, cp)?);
        }
        Some(MethodInfo {
            access_flags,
            name,
            descriptor,
            name_index,
            descriptor_index,
            attributes,
        })
    }
}

bitflags! {
    pub struct MethodAccessFlags: usize {
        const PUBLIC	   = 0x0001;
        const PRIVATE	   = 0x0002;
        const PROTECTED	   = 0x0004;
        const STATIC	   = 0x0008;
        const FINAL	       = 0x0010;
        const SYNCHRONIZED = 0x0020;
        const BRIDGE	   = 0x0040;
        const VARARGS	   = 0x0080;
        const NATIVE	   = 0x0100;
        const ABSTRACT	   = 0x0400;
        const STRICT	   = 0x0800;
        const SYNTHETIC	   = 0x1000;
    }
}
