use std::{fs, path::Path};


use directories::UserDirs;

const SERDE_TYPEDEF: &str = include_str!("../../typedef/api/serde.luau");
const FS_TYPEDEF: &str = include_str!("../../typedef/api/fs.luau");
const IO_TYPEDEF: &str = include_str!("../../typedef/api/io.luau");
const SYSTEM_TYPEDEF: &str = include_str!("../../typedef/api/system.luau");
const CHRONO_TYPEDEF: &str = include_str!("../../typedef/api/chrono.luau");
const MEDIA_TYPEDEF: &str = include_str!("../../typedef/api/media.luau");
const NET_TYPEDEF: &str = include_str!("../../typedef/api/net.luau");
const TASK_TYPEDEF: &str = include_str!("../../typedef/api/task.luau");
const UI_TYPEDEF: &str = include_str!("../../typedef/api/ui.luau");

const MAIN_DEF: &str = include_str!("../../typedef/definitions/types.d.luau");
const NET_DEF: &str = include_str!("../../typedef/definitions/net.d.luau");
const UI_DEF: &str = include_str!("../../typedef/definitions/ui.d.luau");

pub async fn setup() -> Result<(), Box<dyn std::error::Error>>{

    generate_typedef().await.expect("failed to generate type definitions");

    let mut luaurc_contents = get_luaurc_contents().await.expect("failed to read .luaurc");
    write_luaurc(&mut luaurc_contents).await.expect("failed to write to .luaurc");

    let mut vscode_contents = get_vscode_contents().await.expect("failed to read .vscode/settings.json");
    write_vscode(&mut vscode_contents).await.expect("failed to write to .vscode/settings.json");

    println!("Type definitions for vara v{} have been created\
    \nYou may have to restart your editor if the changes haven't taken affect.",env!("CARGO_PKG_VERSION"));

    Ok(())
}

async fn get_luaurc_contents() -> Result<serde_json::Value, Box<dyn std::error::Error>>{
    let luaurc_path = Path::new(".luaurc");


    if luaurc_path.exists() {
        let contents = fs::read(luaurc_path)?;

        match serde_json::from_slice(&contents) {
            Ok(content) => Ok(content),
            Err(err) => Err(Box::new(err)),
        }
    } else {
        Ok(serde_json::Value::Object(serde_json::Map::new()))
    }
}

async fn write_luaurc(contents : &mut serde_json::Value) -> Result<(), Box<dyn std::error::Error>>{
    let version_string = env!("CARGO_PKG_VERSION");
    let typedef_directory = serde_json::Value::String(format!("~/.vara/.typedefs/{}/api",version_string));

    let field = String::from("aliases");
    let alias = String::from("vara");

    if let Some(serde_json::Value::Object(aliases)) = contents.get_mut(&field) {
        if aliases.contains_key(&alias) {
            if aliases.get(&alias).unwrap() != &typedef_directory {
                aliases.insert(alias, typedef_directory);
            }
        } else {
            aliases.insert(alias, typedef_directory);
        }
    } else {
        let mut map = serde_json::Map::new();
        map.insert(alias, typedef_directory);
        contents[field] = serde_json::Value::Object(map);
    }

    let encoded = serde_json::to_vec_pretty(&contents)?;

    fs::write(".luaurc", encoded)?;

    Ok(())
}

// will probably add more integrations for neovim and zed
async fn get_vscode_contents() -> Result<serde_json::Value, Box<dyn std::error::Error>>{
    let settings_path = Path::new(".vscode/settings.json");

    if settings_path.exists() {
        let contents = fs::read_to_string(settings_path)?;

        match jsonc_parser::parse_to_serde_value(&contents,&jsonc_parser::ParseOptions::default()) {
            Ok(content) => Ok(content),
            Err(err) => Err(Box::new(err)),
        }
    } else {
        return Ok(serde_json::Value::Object(serde_json::Map::new()));
    }
}

async fn write_vscode(contents : &mut serde_json::Value) -> Result<(), Box<dyn std::error::Error>>{
    let version_string = env!("CARGO_PKG_VERSION");

    let definition = String::from("luau-lsp.types.definitionFiles");

    let paths : [(String,String);3] = [
        (String::from("vara"),format!("~/.vara/.typedefs/{}/definitions/types.d.luau",version_string)),
        (String::from("vara/net"),format!("~/.vara/.typedefs/{}/definitions/net.d.luau",version_string)),
        (String::from("vara/ui"),format!("~/.vara/.typedefs/{}/definitions/ui.d.luau",version_string)),
    ];

    if let Some(serde_json::Value::Object(definition)) = contents.get_mut(&definition) {

        for pair in paths {
            let (key,value) = pair;

            let value_string = serde_json::Value::String(value);

            if definition.contains_key(&key) {
                if definition.get(&key).unwrap() != &value_string {
                    definition.insert(key, value_string);
                }
            } else {
                definition.insert(key, value_string);
            }
        }
    } else {
        let mut map = serde_json::Map::new();

        for(key,value) in paths{
            map.insert(key.to_string(), serde_json::Value::String(value.to_string()));
        }

        contents[definition] = serde_json::Value::Object(map);
    }

    let encoded = serde_json::to_vec_pretty(&contents)?;

    if !Path::new(".vscode").exists() {
        fs::create_dir(".vscode")?;
    }

    fs::write(".vscode/settings.json", encoded)?;

    Ok(())
}

async fn generate_typedef() -> Result<(), Box<dyn std::error::Error>> {
    let version_string = env!("CARGO_PKG_VERSION");
    
    let typedef_directory = UserDirs::new()
        .ok_or("failed to load typedefs")?
        .home_dir()
        .join(".vara")
        .join(".typedefs")
        .join(version_string)
        .join("definitions");

    let api_directory = UserDirs::new()
        .ok_or("failed to load typedefs")?
        .home_dir()
        .join(".vara")
        .join(".typedefs")
        .join(version_string)
        .join("api");

    async_fs::create_dir_all(&api_directory).await?;
    async_fs::create_dir_all(&typedef_directory).await?;

    async_fs::write(api_directory.join("serde").with_extension("luau"), SERDE_TYPEDEF).await?;
    async_fs::write(api_directory.join("fs").with_extension("luau"), FS_TYPEDEF).await?;
    async_fs::write(api_directory.join("io").with_extension("luau"),IO_TYPEDEF).await?;
    async_fs::write(api_directory.join("system").with_extension("luau"),SYSTEM_TYPEDEF).await?;
    async_fs::write(api_directory.join("chrono").with_extension("luau"),CHRONO_TYPEDEF).await?;
    async_fs::write(api_directory.join("media").with_extension("luau"),MEDIA_TYPEDEF).await?;
    async_fs::write(api_directory.join("net").with_extension("luau"),NET_TYPEDEF).await?;
    async_fs::write(api_directory.join("task").with_extension("luau"),TASK_TYPEDEF).await?;
    async_fs::write(api_directory.join("ui").with_extension("luau"),UI_TYPEDEF).await?;

    async_fs::write(typedef_directory.join("types").with_extension("d.luau"),MAIN_DEF).await?;
    async_fs::write(typedef_directory.join("net").with_extension("d.luau"),NET_DEF).await?;
    async_fs::write(typedef_directory.join("ui").with_extension("d.luau"),UI_DEF).await?;
    Ok(())
}