use std::{env, path::PathBuf, sync::LazyLock};

use darklua_core::{BundleConfiguration, Configuration, Options, Resources, rules::{Rule, bundle::BundleRequireMode}};
use mlua::chunk::Compiler;

use vara_core::runtime::luau as vara_runtime;

pub static CURRENT_EXE : LazyLock<PathBuf> = LazyLock::new(|| env::current_exe().expect("failed to retrieve current exe"));

const MAGIC : &[u8; 6]= b"$$VARA";

pub async fn compile(path : &PathBuf) -> Result<(),Box<dyn std::error::Error>>{

    if !path.exists(){
        return Err(format!("could not find file {}",path.display()).into());
    }

    let resources = Resources::from_file_system();
    
    let config = Configuration::empty()
        .with_bundle_configuration(
            BundleConfiguration::new(BundleRequireMode::Luau(Default::default()))
        )
        .with_generator(darklua_core::GeneratorParameters::default_dense())
        .with_rule(Box::new(darklua_core::rules::RemoveSpaces::default()) as Box<dyn darklua_core::rules::Rule>)
        .with_rule(Box::new(darklua_core::rules::RemoveUnusedVariable::default()) as Box<dyn darklua_core::rules::Rule>)
        .with_rule(Box::new(darklua_core::rules::RemoveUnusedIfBranch::default()) as Box<dyn darklua_core::rules::Rule>)
        .with_rule(Box::new(darklua_core::rules::RemoveUnusedWhile::default()) as Box<dyn darklua_core::rules::Rule>)
        .with_rule(Box::new(darklua_core::rules::RemoveComments::default()) as Box<dyn darklua_core::rules::Rule>)
        .with_rule(Box::new(darklua_core::rules::FilterAfterEarlyReturn::default()) as Box<dyn darklua_core::rules::Rule>)
        .with_rule(Box::new(darklua_core::rules::ComputeExpression::default()) as Box<dyn darklua_core::rules::Rule>)
        .with_rule(Box::new(darklua_core::rules::ConvertSquareRootCall::default()) as Box<dyn darklua_core::rules::Rule>)
        .with_rule(Box::new(darklua_core::rules::ConvertIndexToField::default()) as Box<dyn darklua_core::rules::Rule>)
        .with_rule(Box::new(darklua_core::rules::RemoveEmptyDo::default()) as Box<dyn darklua_core::rules::Rule>);

    let options = Options::new(path)
        .with_configuration(config)
        .with_output("__$VARA_bundled.luau");

    let result = match darklua_core::process(&resources, options) {
        Ok(_result) => _result,
        Err(err) => return Err(format!("{}",err).into()),
    };

    let errors = result.collect_errors();
    if !errors.is_empty() {
        let error_message = errors.iter().map(|err| format!("{err}")).collect::<Vec<_>>().join("\n");

        return Err(format!("darklua failed to bundle code {error_message}").into());
    }

    let bundled = resources.get("__$VARA_bundled.luau").map_err(|_| "failed to get bundled contents from resources")?;

    std::fs::remove_file("__$VARA_bundled.luau")?;

    let file_contents = bundled.as_bytes().to_vec();

    let compiler = Compiler::new();

    let bytecode = compiler.compile(file_contents)?;

    let mut metadata : Vec<u8> = Vec::new();
    metadata.extend_from_slice(&bytecode);
    metadata.extend_from_slice(&(bytecode.len() as u64).to_be_bytes());
    metadata.extend_from_slice(MAGIC);

    let mut source = async_fs::read(CURRENT_EXE.to_path_buf()).await?;

    source.extend_from_slice(&metadata);

    
    let filename = path.file_stem()
        .and_then(|os_str| os_str.to_str())
        .map(|s| s.to_string())
        .unwrap_or("program".to_string());

    async_fs::write(format!("./{}.exe",filename), source).await?;

    println!("compiled {} successfully",path.display());
    Ok(())
}

pub async fn check() -> (bool,Vec<u8>){
    let contents = async_fs::read(CURRENT_EXE.to_path_buf())
        .await
        .unwrap_or_default();

    let is_binary = contents.ends_with(MAGIC);

    (is_binary,contents)
}

pub async fn run_standalone(contents : Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
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
    let runtime = vara_runtime::Runtime::new(args)?;

    runtime.run_bytecode(bytecode).await?;

    Ok(())
}