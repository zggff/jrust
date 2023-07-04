#![feature(iter_advance_by)]
#![allow(dead_code)]

use class::attribute::CodeAttribute;
use code::OpCode;
use source::ByteStream;
use std::{collections::HashMap, env, fs::File, io::Read, process::exit};

use anyhow::Context;

use crate::class::{
    attribute::Attribute,
    constant_pool::{CpInfo, IntegerInfo, StringInfo},
    Class,
};

macro_rules! cast {
    ($target: expr, $pat: path) => {{
        if let $pat(a) = $target {
            a
        } else {
            panic!("mismatch variant when cast to {}", stringify!($pat)); // #2
        }
    }};
}

mod class;
mod code;
mod source;

struct Method {}

fn main() -> anyhow::Result<()> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("usage: {} file", &args[0]);
        exit(-1);
    }

    let mut file = File::open(args[1].clone()).context("failed to open file")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let mut f = ByteStream::from(buffer);

    let class = Class::parse(&mut f).context("could not parse class file")?;
    println!("----CONSTANTS----");
    for (index, attribute) in class.cp.iter().enumerate() {
        println!("{}:\t{attribute:?}", index + 1);
    }

    println!();
    println!("----FIELDS----");
    for field in &class.fields {
        println!(
            "{:?} {:?} {:?}",
            field.access_flags, field.descriptor, field.name,
        );
    }

    println!();

    let mut functions = HashMap::new();

    println!("----METHODS----");
    for method in &class.methods {
        println!(
            "{:?} {:?} {:?}",
            method.access_flags, method.descriptor, method.name,
        );
        let code = method
            .attributes
            .iter()
            .find(|a| a.attribute_name == "Code")
            .unwrap();
        let code = cast!(&code.attribute, Attribute::Code);
        functions.insert((method.name.clone(), method.descriptor.clone()), code);
        println!(
            "stack: {:?}, locals: {:?}\n",
            code.max_stack, code.max_locals
        );
    }

    println!("----EXECUTION----");
    println!();

    let entry = functions
        .get(&(String::from("main"), String::from("([Ljava/lang/String;)V")))
        .context("no main function found")?;
    let locals = vec![0; entry.max_locals];
    let stack = Vec::with_capacity(entry.max_stack);
    exec(&class, &functions, entry, locals, &mut vec![], stack)?;

    Ok(())
}

fn exec(
    c: &Class,
    f: &HashMap<(String, String), &CodeAttribute>,
    code: &CodeAttribute,
    mut l: Vec<isize>,
    s0: &mut Vec<isize>,
    mut s: Vec<isize>,
) -> anyhow::Result<()> {
    let mut code = code.code_raw.clone().into();
    while let Some(op) = OpCode::parse(&mut code) {
        match op {
            OpCode::GetStatic(_index) => {
                // here we initialize class or interface
                // TODO: figure out what it means
            }
            OpCode::IConstM1 => s.push(-1),
            OpCode::IConst0 => s.push(0),
            OpCode::IConst1 => s.push(1),
            OpCode::IConst2 => s.push(2),
            OpCode::IConst3 => s.push(3),
            OpCode::IConst4 => s.push(4),
            OpCode::IConst5 => s.push(5),

            OpCode::BiPush(val) => s.push(val as i8 as isize),
            OpCode::SiPush(val) => s.push(val as i16 as isize),

            OpCode::InvokeVirtual(index) => {
                let methodref = c.cp.get_methodref(index).unwrap();
                let class = c.cp.get_class(methodref.class_index).unwrap();
                let _class_name = &c.cp.get_utf(class.name_index).unwrap().bytes;

                let name_type =
                    c.cp.get_name_and_type(methodref.name_and_type_index)
                        .unwrap();
                let fn_name = c.cp.get_utf(name_type.name_index).unwrap().bytes.as_str();
                let fn_type =
                    c.cp.get_utf(name_type.descriptor_index)
                        .unwrap()
                        .bytes
                        .as_str();
                match (fn_name, fn_type) {
                    ("println", "(I)V") => println!("{}", s.pop().unwrap()),
                    ("println", "(Ljava/lang/String;)V") => {
                        println!("{}", c.cp.get_utf(s.pop().unwrap() as usize).unwrap().bytes)
                    }

                    (n, t) => todo!("not implemented {n} {t}"),
                }
            }
            OpCode::InvokeStatic(index) => {
                let methodref = c.cp.get_methodref(index).unwrap();
                let class = c.cp.get_class(methodref.class_index).unwrap();
                let _class_name = c.cp.get_utf(class.name_index).unwrap().bytes.as_str();

                let name_type =
                    c.cp.get_name_and_type(methodref.name_and_type_index)
                        .unwrap();
                let fn_name = c.cp.get_utf(name_type.name_index).unwrap().bytes.clone();
                let fn_type =
                    c.cp.get_utf(name_type.descriptor_index)
                        .unwrap()
                        .bytes
                        .clone();

                let func = f.get(&(fn_name, fn_type)).unwrap();

                let mut locals = Vec::with_capacity(func.max_locals);
                for _ in 0..func.max_locals {
                    locals.push(s.pop().unwrap());
                }
                let stack = Vec::with_capacity(func.max_stack);
                exec(c, f, func, locals, &mut s, stack).unwrap();
            }

            OpCode::ILoad0 => s.push(l[0]),
            OpCode::ILoad1 => s.push(l[1]),
            OpCode::ILoad2 => s.push(l[2]),
            OpCode::ILoad3 => s.push(l[3]),

            OpCode::ALoad0 => s.push(l[0]),
            OpCode::ALoad1 => s.push(l[1]),
            OpCode::ALoad2 => s.push(l[2]),
            OpCode::ALoad3 => s.push(l[3]),

            OpCode::IStore0 => l[0] = s.pop().unwrap(),
            OpCode::IStore1 => l[1] = s.pop().unwrap(),
            OpCode::IStore2 => l[2] = s.pop().unwrap(),
            OpCode::IStore3 => l[3] = s.pop().unwrap(),
            OpCode::AStore0 => l[0] = s.pop().unwrap(),
            OpCode::AStore1 => l[1] = s.pop().unwrap(),
            OpCode::AStore2 => l[2] = s.pop().unwrap(),
            OpCode::AStore3 => l[3] = s.pop().unwrap(),

            OpCode::IfEq(offset) => {
                if s.pop().unwrap() == 0 {
                    code.advance_by(offset).unwrap();
                }
            }
            OpCode::IfNe(offset) => {
                if s.pop().unwrap() != 0 {
                    code.advance_by(offset).unwrap();
                }
            }
            OpCode::IfLt(offset) => {
                if s.pop().unwrap() < 0 {
                    code.advance_by(offset).unwrap();
                }
            }
            OpCode::IfGe(offset) => {
                if s.pop().unwrap() >= 0 {
                    code.advance_by(offset).unwrap();
                }
            }
            OpCode::IfGt(offset) => {
                if s.pop().unwrap() > 0 {
                    code.advance_by(offset).unwrap();
                }
            }
            OpCode::IfLe(offset) => {
                if s.pop().unwrap() <= 0 {
                    code.advance_by(offset).unwrap();
                }
            }
            OpCode::IfICmpEq(offset) => {
                if s.pop().unwrap() == s.pop().unwrap() {
                    code.advance_by(offset).unwrap();
                }
            }

            OpCode::IfICmpNe(offset) => {
                if s.pop().unwrap() != s.pop().unwrap() {
                    code.advance_by(offset).unwrap();
                }
            }

            OpCode::IfICmpLt(offset) => {
                let a = s.pop().unwrap();
                let b = s.pop().unwrap();
                if b < a {
                    code.advance_by(offset).unwrap();
                }
            }
            OpCode::IfICmpGe(offset) => {
                let a = s.pop().unwrap();
                let b = s.pop().unwrap();
                if b >= a {
                    code.advance_by(offset).unwrap();
                }
            }
            OpCode::IfICmpGt(offset) => {
                let a = s.pop().unwrap();
                let b = s.pop().unwrap();
                if b > a {
                    code.advance_by(offset).unwrap();
                }
            }
            OpCode::IfICmpLe(offset) => {
                let a = s.pop().unwrap();
                let b = s.pop().unwrap();
                if b <= a {
                    code.advance_by(offset).unwrap();
                }
            }

            OpCode::Goto(offset) => {
                code.advance_by(offset).unwrap();
            }

            OpCode::IAdd => {
                let a = s.pop().unwrap();
                let b = s.pop().unwrap();
                s.push(a + b);
            }

            OpCode::ISub => {
                let a = s.pop().unwrap();
                let b = s.pop().unwrap();
                s.push(b - a);
            }

            OpCode::IMul => {
                let a = s.pop().unwrap();
                let b = s.pop().unwrap();
                s.push(a * b);
            }

            OpCode::Ldc(index) => match c.cp.get(index).unwrap() {
                &CpInfo::Integer(IntegerInfo { val }) => s.push(val as i32 as isize),
                &CpInfo::String(StringInfo { string_index }) => s.push(string_index as isize),

                a => todo!("not implemented {:?}", a),
            },
            OpCode::IReturn => {
                s0.push(s.pop().unwrap());
                break;
            }
            OpCode::Return => break,

            op => todo!("opcode not implemented: {op:?}"),
        }
    }
    Ok(())
}
