use std::{env, path::PathBuf, sync::LazyLock};

use mlua::chunk::Compiler;
use vara_core::stdlib::files as vara_fs;

use crate::luau::Runtime;

pub static CURRENT_EXE : LazyLock<PathBuf> = LazyLock::new(|| env::current_exe().expect("failed to retrieve current exe"));

const MAGIC : &[u8; 6]= b"$$VARA";

pub async fn compile(path : PathBuf) -> Result<(),Box<dyn std::error::Error>>{

    if !path.exists(){
        return Err(format!("could not find file {}",path.display()).into());
    }

    let file_contents = &vara_fs::read_file_as_binary(&path)?;
    let compiler = Compiler::new();

    let bytecode = compiler.compile(file_contents)?;

    let mut metadata : Vec<u8> = Vec::new();
    metadata.extend_from_slice(&bytecode);
    metadata.extend_from_slice(&(bytecode.len() as u64).to_be_bytes());
    metadata.extend_from_slice(MAGIC);

    let mut source = async_fs::read(CURRENT_EXE.to_path_buf())
        .await
        .unwrap_or_default();

    source.extend_from_slice(&metadata);

    
    let filename = path.file_stem()
        .and_then(|os_str| os_str.to_str())
        .map(|s| s.to_string())
        .unwrap_or("program".to_string());

    async_fs::write(format!("./{}.exe",filename), source).await?;

    println!("{}",format!("compiled {} successfully",path.display()));
    Ok(())
}

pub async fn check() -> (bool,Vec<u8>){
    let contents = async_fs::read(CURRENT_EXE.to_path_buf())
        .await
        .unwrap_or_default();

    let is_binary = contents.ends_with(MAGIC);

    (is_binary,contents)
}

pub fn run_standalone(contents : Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    const SIZE_BYTES : usize = 8;
    const MAGIC_BYTES : usize = 6;
    const FOOTER_SIZE : usize = SIZE_BYTES + MAGIC_BYTES;

    if contents.len() < FOOTER_SIZE {
        return Err("standalone binary too small".into());
    }

    if !contents.ends_with(MAGIC) {
        return Err("this is not a standalone binary".into());
    }

    let size_start = contents.len() - FOOTER_SIZE;
    let size_end = size_start + SIZE_BYTES;

    let bytecode_bytesize : [u8;SIZE_BYTES] = contents[size_start..size_end].try_into()?;

    let bytecode_size = usize::try_from(u64::from_be_bytes(bytecode_bytesize))?;

    if bytecode_size > size_start {
        return Err(format!("invalid bytecode size: expected {} bytes but only {} bytes are before the footer",bytecode_size,size_start).into());
    }


    let bytecode = contents[(size_start - bytecode_size)..size_start].to_vec();

    let args = env::args().skip(1).collect::<Vec<_>>();
    let runtime = Runtime::new(args).expect("failed to setup runtime exe");

    runtime.run_bytecode(bytecode)?;

    Ok(())
}