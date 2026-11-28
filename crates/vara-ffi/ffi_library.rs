use std::{
    collections::{HashMap, HashSet},
    sync::{Arc}
};

use libloading::Library;
use mlua::{ExternalError, ExternalResult};

use libffi::{low::CodePtr, middle::{Cif, Type as ffi_type,arg}};

#[derive(Clone,Copy)]
enum NativeType {
    Int,
    Uint,
    Char,
    Uchar,
    Long,
    LongLong,
    Ulong,
    Float,
    Double,
    Void,
}

enum NativeValue {
    Int(i32),
    Uint(u32),
    Char(i8),
    Uchar(u8),
    Long(std::ffi::c_long),
    LongLong(std::ffi::c_longlong),
    Ulong(std::ffi::c_ulong),
    Float(f32),
    Double(f64),
}

impl NativeType {
    fn to_ffi(self) -> ffi_type {
        match self {
            Self::Int => ffi_type::c_int(),
            Self::Uint => ffi_type::c_uint(),
            Self::Char => ffi_type::c_schar(),
            Self::Uchar => ffi_type::c_uchar(),
            Self::Long => ffi_type::c_long(),
            Self::LongLong => ffi_type::c_longlong(),
            Self::Ulong => ffi_type::c_ulong(),
            Self::Float => ffi_type::f32(),
            Self::Double => ffi_type::f64(),
            Self::Void => ffi_type::void(),
        }
    }
}

#[derive(mlua::UserData)]
pub struct ForeignLibrary {
    #[lua(skip)]
    lib: Arc<Library>,
    #[lua(name="Defined")]
    pub defined: bool,
    #[lua(skip)]
    args: HashMap<String,Vec<NativeType>>,
    #[lua(skip)]
    return_values: HashMap<String,NativeType>,
    #[lua(skip)]
    pub functions: HashSet<String>
}

#[mlua::userdata_impl]
impl ForeignLibrary {
    #[lua(skip)]
    pub fn new(path: String) -> mlua::Result<Self>{
        unsafe{
            Ok(Self { 
                lib: Arc::new(Library::new(path).into_lua_err()?),
                defined: false,
                args: HashMap::new(),
                return_values: HashMap::new(),
                functions: HashSet::new(),
            })
        }
    }

    #[lua(name = "Define")]
    fn define(this: mlua::AnyUserData,name: String, args: Vec<String>,returns : Option<String>) -> mlua::Result<mlua::AnyUserData>{
        {
            let mut foreign_library = this.borrow_mut::<ForeignLibrary>()?;
            foreign_library.defined = true;

            let return_type = string_to_ffitype(returns.unwrap_or("void".to_string()))?;

            let mut arg_types = Vec::new();

            for arg_string in args {
                arg_types.push(string_to_ffitype(arg_string)?);
            }

            foreign_library.args.insert(name.clone(), arg_types);
            foreign_library.return_values.insert(name.clone(), return_type);
            foreign_library.functions.insert(name.clone());
        }

        Ok(this)
    }

    #[lua(meta, name="__index")]
    fn __index(&self,context: &mlua::Lua,name: String) -> mlua::Result<mlua::Value>{
        if !self.defined {
            return Err("No functions have been defined for this library.".into_lua_err());
        }

        if !self.functions.contains(&name){
            return Err(format!("{name} is not a defined function for this library.").into_lua_err())
        }

        let args = self.args.get(&name).ok_or(format!("No arguments were defined for function '{}'",name)).into_lua_err()?.clone();
        let return_value = self.return_values.get(&name).ok_or(format!("No return value was defined for function '{}'",name)).into_lua_err()?.clone();

        let native_args: Vec<ffi_type> = args.iter().map(|ty| ty.to_ffi()).collect();

        let cif = Cif::new(native_args, return_value.to_ffi());

        let symbol = unsafe {
            self.lib.get::<unsafe extern "C" fn()>(format!("{}\0",name).as_bytes())
        }.into_lua_err()?;

        let fn_ptr = *symbol as *const () as *mut std::ffi::c_void;

        let library = Arc::clone(&self.lib);

        let callback = context.create_function(move |_: &mlua::Lua,arguments: mlua::MultiValue| {
            
            let _keep_alive = &library;

            let values = convert_args(arguments, &args)?;
            let ffi_args = build_ffi_arguments(&values);

            call_c_function(&cif, fn_ptr, &ffi_args, return_value)
        })?;


        Ok(mlua::Value::Function(callback))
    }
}

fn convert_args(args: mlua::MultiValue,types: &[NativeType]) -> mlua::Result<Vec<NativeValue>>{
    if args.len() != types.len() {
        return Err(format!("wrong ammount of arguments, expected {} arguments got {} instead",types.len(),args.len()).into_lua_err())
    }

    let mut values = Vec::new();

    for (value,ty) in args.into_iter().zip(types.iter()) {
        let number = match value {
            mlua::Value::Integer(v) => v as f64,
            mlua::Value::Number(v) => v,
            _ => return Err("Expected numeric argument".into_lua_err()),
        };

        values.push(match ty {
            NativeType::Int => NativeValue::Int(number as i32),
            NativeType::Uint => NativeValue::Uint(number as u32),
            NativeType::Char => NativeValue::Char(number as i8),
            NativeType::Uchar => NativeValue::Uchar(number as u8),
            NativeType::Long => NativeValue::Long(number as std::ffi::c_long),
            NativeType::LongLong => NativeValue::LongLong(number as std::ffi::c_longlong),
            NativeType::Ulong => NativeValue::Ulong(number as std::ffi::c_ulong),
            NativeType::Float => NativeValue::Float(number as f32),
            NativeType::Double => NativeValue::Double(number as f64),

            NativeType::Void => return Err("void type cannot be an argument".into_lua_err())
        });
    }

    Ok(values)
}

fn build_ffi_arguments(values: &[NativeValue]) -> Vec<libffi::middle::Arg<'_>>{
    values.iter().map(|value| {
        match value {
            NativeValue::Int(n) => arg(n),
            NativeValue::Uint(n) => arg(n),
            NativeValue::Char(n) => arg(n),
            NativeValue::Uchar(n) => arg(n),
            NativeValue::Long(n) => arg(n),
            NativeValue::LongLong(n) => arg(n),
            NativeValue::Ulong(n) => arg(n),
            NativeValue::Float(n) => arg(n),
            NativeValue::Double(n) => arg(n),
        }
    }).collect()
}

fn call_c_function(cif: &Cif, function: *mut std::ffi::c_void, arguments: &[libffi::middle::Arg<'_>], return_type: NativeType) -> mlua::Result<mlua::Value> {
    unsafe { 
        let ptr = CodePtr(function);
        
        Ok(match return_type {
            NativeType::Void => {
                cif.call::<()>(ptr, arguments);
                mlua::Value::Nil
            },

            NativeType::Int => {
                mlua::Value::Integer(cif.call::<i32>(ptr, arguments) as i64)
            },
            NativeType::Uint => {
                mlua::Value::Integer(cif.call::<u32>(ptr, arguments) as i64)
            },
            NativeType::Char => {
                mlua::Value::Integer(cif.call::<i8>(ptr, arguments) as i64)
            },
            NativeType::Uchar => {
                mlua::Value::Integer(cif.call::<u8>(ptr, arguments) as i64)
            },
            NativeType::Long => {
                mlua::Value::Integer(cif.call::<std::ffi::c_long>(ptr, arguments) as i64)
            },
            NativeType::LongLong => {
                mlua::Value::Integer(cif.call::<std::ffi::c_longlong>(ptr, arguments) as i64)
            },
            NativeType::Ulong => {
                mlua::Value::Number(cif.call::<std::ffi::c_ulong>(ptr, arguments) as f64)
            },
            NativeType::Float => {
                mlua::Value::Number(cif.call::<f32>(ptr, arguments) as f64)
            },
            NativeType::Double => {
                mlua::Value::Number(cif.call::<f64>(ptr, arguments) as f64)
            },
        })
    }
}


fn string_to_ffitype(string_type: String) -> mlua::Result<NativeType> {

    match string_type.as_str() {
        "int" => Ok(NativeType::Int),
        "uint" => Ok(NativeType::Uint),
        "char" => Ok(NativeType::Char),
        "uchar" => Ok(NativeType::Uchar),
        "long" => Ok(NativeType::Long),
        "longlong" => Ok(NativeType::LongLong),
        "ulong" => Ok(NativeType::Ulong),
        "float" => Ok(NativeType::Float),
        "double" => Ok(NativeType::Double),
        "void" => Ok(NativeType::Void),
        _ => Err(format!("{} is not a valid C type.",string_type).into_lua_err())
    }
}