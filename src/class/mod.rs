use bitflags::bitflags;
use constant_pool::CpPool;

use crate::source::ByteStream;

use self::{attribute::AttributeInfo, field::FieldInfo, method::MethodInfo};

pub mod attribute;
pub mod constant_pool;
pub mod field;
pub mod method;

bitflags! {
    pub struct ClassAccessFlags: usize {
        const PUBLIC	  =  0x0001;
        const FINAL	      =  0x0010;
        const SUPER	      =  0x0020;
        const INTERFACE	  =  0x0200;
        const ABSTRACT	  =  0x0400;
        const SYNTHETIC	  =  0x1000;
        const ANNOTATION  =  0x2000;
        const ENUM	      =  0x4000;
        const MODULE      =  0x8000;
    }

}

#[derive(Debug, Clone)]
pub struct Class {
    pub magic: usize,
    pub minor_version: usize,
    pub major_version: usize,
    pub cp: CpPool,
    pub access_flags: ClassAccessFlags,
    pub this_class: usize,
    pub this_class_name: String,
    pub super_class: usize,
    pub super_class_name: String,
    pub interfaces: Vec<usize>,
    pub fields: Vec<FieldInfo>,         // raw fields
    pub methods: Vec<MethodInfo>,       // raw methods
    pub attributes: Vec<AttributeInfo>, // raw attributes
}

impl Class {
    pub fn parse(f: &mut ByteStream) -> Option<Class> {
        let magic = f.next_u4()?;
        let minor_version = f.next_u2()?;
        let major_version = f.next_u2()?;

        let constant_pool = CpPool::parse(f)?;

        let access_flags = ClassAccessFlags { bits: f.next_u2()? };
        let this_class = f.next_u2()?;
        let super_class = f.next_u2()?;

        let this_class_name = constant_pool
            .get_utf(constant_pool.get_class(this_class)?.name_index)?
            .bytes
            .clone();

        let super_class_name = constant_pool
            .get_utf(constant_pool.get_class(super_class)?.name_index)?
            .bytes
            .clone();

        let interfaces_count = f.next_u2()?;
        let mut interfaces = Vec::with_capacity(interfaces_count);
        for _ in 0..interfaces_count {
            interfaces.push(f.next_u2()?);
        }

        let fields_count = f.next_u2()?;
        let mut fields = Vec::with_capacity(fields_count);
        for _ in 0..fields_count {
            fields.push(FieldInfo::parse(f, &constant_pool)?);
        }

        let methods_count = f.next_u2()?;
        let mut methods = Vec::with_capacity(methods_count);
        for _ in 0..methods_count {
            methods.push(MethodInfo::parse(f, &constant_pool)?);
        }

        let attributes_count = f.next_u2()?;
        let mut attributes = Vec::with_capacity(attributes_count);
        for _ in 0..attributes_count {
            attributes.push(AttributeInfo::parse(f, &constant_pool)?);
        }

        Some(Class {
            magic,
            minor_version,
            major_version,
            cp: constant_pool,
            access_flags,
            this_class,
            this_class_name,
            super_class,
            super_class_name,
            interfaces,
            fields,
            methods,
            attributes,
        })
    }
}
