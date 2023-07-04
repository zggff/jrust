use std::ops::{Deref, DerefMut};

use crate::ByteStream;

#[derive(Debug, Clone)]
pub struct ClassInfo {
    pub name_index: usize,
}
#[derive(Debug, Clone)]
pub struct FieldrefInfo {
    pub class_index: usize,
    pub name_and_type_index: usize,
}
#[derive(Debug, Clone)]
pub struct MethodrefInfo {
    pub class_index: usize,
    pub name_and_type_index: usize,
}
#[derive(Debug, Clone)]
pub struct InterfaceMethodrefInfo {
    pub class_index: usize,
    pub name_and_type_index: usize,
}
#[derive(Debug, Clone)]
pub struct NameAndTypeInfo {
    pub name_index: usize,
    pub descriptor_index: usize,
}
#[derive(Debug, Clone)]
pub struct UtfInfo {
    pub bytes: String,
}
#[derive(Debug, Clone)]
pub struct StringInfo {
    pub string_index: usize,
}

#[derive(Debug, Clone)]
pub struct IntegerInfo {
    pub val: usize
}

#[derive(Debug, Clone)]
#[repr(u8)]
pub enum CpInfo {
    Class(ClassInfo),
    Fieldref(FieldrefInfo),
    Methodref(MethodrefInfo),
    InterfaceMethodref(InterfaceMethodrefInfo),
    NameAndType(NameAndTypeInfo),
    Utf(UtfInfo),
    String(StringInfo),
    Integer(IntegerInfo)
}

impl CpInfo {
    pub fn parse(f: &mut ByteStream) -> Option<Self> {
        let tag = f.next_u1()?;
        match tag {
            10 => Some(CpInfo::Methodref(MethodrefInfo {
                class_index: f.next_u2()?,
                name_and_type_index: f.next_u2()?,
            })),
            7 => Some(CpInfo::Class(ClassInfo {
                name_index: f.next_u2()?,
            })),
            12 => Some(CpInfo::NameAndType(NameAndTypeInfo {
                name_index: f.next_u2()?,
                descriptor_index: f.next_u2()?,
            })),
            1 => {
                let len = f.next_u2()?;
                let mut bytes = Vec::with_capacity(len);
                for _ in 0..len {
                    bytes.push(f.next_u1()?);
                }
                Some(CpInfo::Utf(UtfInfo {
                    bytes: String::from_utf8(bytes).ok()?,
                }))
            }
            9 => Some(CpInfo::Fieldref(FieldrefInfo {
                class_index: f.next_u2()?,
                name_and_type_index: f.next_u2()?,
            })),
            8 => Some(CpInfo::String(StringInfo {
                string_index: f.next_u2()?,
            })),
            3 => Some(CpInfo::Integer(IntegerInfo { val: f.next_u4()? })),
            tag => {
                panic!("unknown tag {tag}")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct CpPool(Vec<CpInfo>);

impl CpPool {
    pub fn parse(f: &mut ByteStream) -> Option<Self> {
        let constant_pool_count = f.next_u2()?;
        let mut constant_pool = Vec::with_capacity(constant_pool_count - 1);
        for _ in 0..constant_pool_count - 1 {
            constant_pool.push(CpInfo::parse(f)?);
        }
        Some(CpPool(constant_pool))
    }
    pub fn get(&self, index: usize) -> Option<&CpInfo> {
        self.0.get(index - 1)
    }
    pub fn get_utf(&self, index: usize) -> Option<&UtfInfo> {
        self.get(index).map(|x| cast!(x, CpInfo::Utf))
    }
    pub fn get_class(&self, index: usize) -> Option<&ClassInfo> {
        self.get(index).map(|x| cast!(x, CpInfo::Class))
    }
    pub fn get_string(&self, index: usize) -> Option<&StringInfo> {
        self.get(index).map(|x| cast!(x, CpInfo::String))
    }
    pub fn get_fieldref(&self, index: usize) -> Option<&FieldrefInfo> {
        self.get(index).map(|x| cast!(x, CpInfo::Fieldref))
    }
    pub fn get_name_and_type(&self, index: usize) -> Option<&NameAndTypeInfo> {
        self.get(index).map(|x| cast!(x, CpInfo::NameAndType))
    }
    pub fn get_interface_methodref(&self, index: usize) -> Option<&InterfaceMethodrefInfo> {
        self.get(index)
            .map(|x| cast!(x, CpInfo::InterfaceMethodref))
    }
    pub fn get_methodref(&self, index: usize) -> Option<&MethodrefInfo> {
        self.get(index)
            .map(|x| cast!(x, CpInfo::Methodref))
    }
    pub fn get_integer(&self, index: usize) -> Option<&IntegerInfo> {
        self.get(index).map(|x| cast!(x, CpInfo::Integer))
    }
}

impl Deref for CpPool {
    type Target = Vec<CpInfo>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CpPool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
