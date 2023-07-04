use crate::{code::OpCode, source::ByteStream};

use super::constant_pool::CpPool;

#[derive(Debug, Clone)]
pub enum Attribute {
    Other(Vec<u8>),
    // critical for correct interpretation
    ConstantValue,
    Code(CodeAttribute),
    StackMapTable(StackMapTableAttribute),
    BootstrapMethods,
    NestHost,
    NestMembers,
    PermittedSubclasses,
    // critical for correct interpretation of class libraries
    Exceptions,
    InnerClasses,
    EnclosingMethod,
    Synthetic,
    Signature,
    Record,
    SourceFile(SourceFileAttribute),
    LineNumberTable(LineNumberTableAttribute),
    LocalVariableTable,
    LocalVariableTypeTable,
    // non critical
    SourceDebugExtension,
    Deprecated,
    RuntimeVisibleAnnotations,
    RuntimeInvisibleAnnotations,
    RuntimeVisibleParameterAnnotations,
    RuntimeInvisibleParameterAnnotations,
    RuntimeVisibleTypeAnnotations,
    RuntimeInvisibleTypeAnnotations,
    AnnotationDefault,
    MethodParameters,
    Module,
    ModulePackages,
    ModuleMainClass,
}

#[derive(Debug, Clone)]
pub struct StackMapTableAttribute {
    pub entries: Vec<StackMapFrame>,
}

#[derive(Debug, Clone)]
pub struct SourceFileAttribute {
    pub sourcefile_index: usize,
    pub sourcefile: String,
}

#[derive(Debug, Clone)]
pub struct LineNumberTableAttribute {
    pub line_number_table: Vec<LineNumber>,
}

#[derive(Debug, Clone)]
pub struct LineNumber {
    pub start_pc: usize,
    pub line_number: usize,
}

#[derive(Debug, Clone)]
pub struct CodeAttribute {
    pub max_stack: usize,
    pub max_locals: usize,
    pub bytecode_to_op: Vec<usize>,
    pub op_to_bytecode: Vec<usize>,
    pub code: Vec<OpCode>,
    pub code_raw: Vec<u8>,
    pub exception_table: Vec<Exception>,
    pub attributes: Vec<AttributeInfo>,
}

#[derive(Debug, Clone)]
pub struct Exception {
    pub start_pc: usize,
    pub end_pc: usize,
    pub handler_pc: usize,
    pub catch_type: usize,
}

#[derive(Debug, Clone)]
pub struct AttributeInfo {
    pub attribute_name: String,
    pub attribute_name_index: usize,
    pub attribute: Attribute,
}

impl AttributeInfo {
    pub fn parse(f: &mut ByteStream, cp: &CpPool) -> Option<Self> {
        let attribute_name_index = f.next_u2()?;
        let attribute_name = cp.get_utf(attribute_name_index)?.bytes.clone();

        let _attribute_length = f.next_u4()?;

        let attribute = match attribute_name.as_str() {
            "Code" => {
                let max_stack = f.next_u2()?;
                let max_locals = f.next_u2()?;
                let code_length = f.next_u4()?;
                let mut code_raw = Vec::with_capacity(code_length);
                for _ in 0..code_length {
                    code_raw.push(f.next_u1()?);
                }

                let mut code_stream = ByteStream::from(code_raw.clone());
                let mut bytecode_to_op = vec![0; code_raw.len()];
                let mut code = Vec::new();
                let mut op_to_bytecode = Vec::new();
                let mut start = 0;
                while let Some(op) = OpCode::parse(&mut code_stream) {
                    bytecode_to_op[start] = code.len();
                    op_to_bytecode.push(start);
                    start = code_stream.i;
                    code.push(op);
                }

                let exception_table_length = f.next_u2()?;
                let mut exception_table = Vec::with_capacity(exception_table_length);
                for _ in 0..exception_table_length {
                    let exception = Exception {
                        start_pc: f.next_u2()?,
                        end_pc: f.next_u2()?,
                        handler_pc: f.next_u2()?,
                        catch_type: f.next_u2()?,
                    };
                    exception_table.push(exception);
                }
                let attributes_count = f.next_u2()?;
                let mut attributes = Vec::with_capacity(attributes_count);
                for _ in 0..attributes_count {
                    attributes.push(AttributeInfo::parse(f, cp)?);
                }
                Attribute::Code(CodeAttribute {
                    max_stack,
                    max_locals,
                    code,
                    code_raw,
                    bytecode_to_op,
                    op_to_bytecode,
                    exception_table,
                    attributes,
                })
            }
            "LineNumberTable" => {
                let line_number_table_length = f.next_u2()?;
                let mut line_number_table = Vec::with_capacity(line_number_table_length);
                for _ in 0..line_number_table_length {
                    line_number_table.push(LineNumber {
                        start_pc: f.next_u2()?,
                        line_number: f.next_u2()?,
                    });
                }
                Attribute::LineNumberTable(LineNumberTableAttribute { line_number_table })
            }
            "SourceFile" => {
                let sourcefile_index = f.next_u2()?;
                let sourcefile = cp.get_utf(sourcefile_index)?.bytes.clone();
                Attribute::SourceFile(SourceFileAttribute {
                    sourcefile_index,
                    sourcefile,
                })
            }
            "StackMapTable" => {
                let number_of_entries = f.next_u2()?;
                let mut entries = Vec::with_capacity(number_of_entries);
                for _ in 0..number_of_entries {
                    entries.push(StackMapFrame::parse(f, cp)?);
                }
                Attribute::StackMapTable(StackMapTableAttribute { entries })
            }
            name => todo!("parsing attribute: {name}"),
        };

        Some(AttributeInfo {
            attribute_name,
            attribute_name_index,
            attribute,
        })
    }
}

#[derive(Debug, Clone)]
pub enum StackMapFrame {
    SameFrame {
        frame_type: usize,
    },
    SameLocals1StackItemFrame,
    SameLocals1StackItemFrameExtended,
    ChopFrame {
        frame_type: usize,
        offset_delta: usize,
    },
    SameFrameExtended,
    AppendFrame {
        frame_type: usize,
        offset_delta: usize,
        locals: Vec<VerificationTypeInfo>,
    },
    FullFrame,
}

impl StackMapFrame {
    pub fn parse(f: &mut ByteStream, _cp: &CpPool) -> Option<Self> {
        let frame_type = f.next_u1()? as usize;
        match frame_type {
            0..=63 => Some(StackMapFrame::SameFrame { frame_type }),
            248..=250 => {
                let offset_delta = f.next_u2()?;
                Some(StackMapFrame::ChopFrame {
                    frame_type,
                    offset_delta,
                })
            }
            252..=254 => {
                let offset_delta = f.next_u2()?;
                let capacity = frame_type - 251;
                let mut locals = Vec::with_capacity(capacity);
                for _ in 0..capacity {
                    locals.push(VerificationTypeInfo::parse(f)?)
                }
                Some(StackMapFrame::AppendFrame {
                    frame_type,
                    offset_delta,
                    locals,
                })
            }
            a => {
                todo!("stack frame not implemented: {a}");
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum VerificationTypeInfo {
    Top,
    Integer,
    Float,
    Long,
    Double,
    Null,
    UninitializedThis,
    Object(usize),
    Uninitialized(usize),
}

impl VerificationTypeInfo {
    pub fn parse(f: &mut ByteStream) -> Option<Self> {
        let tag = f.next_u1()?;
        let res = match tag {
            0 => Self::Top,
            1 => Self::Integer,
            2 => Self::Float,
            3 => Self::Double,
            4 => Self::Long,
            5 => Self::Null,
            6 => Self::UninitializedThis,
            7 => Self::Object(f.next_u2()?),
            8 => Self::Uninitialized(f.next_u2()?),
            _ => unreachable!(),
        };
        Some(res)
    }
}
