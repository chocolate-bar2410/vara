use std::path::PathBuf;

use mlua::ExternalError;
use rfd::{MessageButtons, MessageDialogResult, MessageLevel};



fn string_to_level(level: String) -> mlua::Result<MessageLevel>{
    return match level.as_str() {
        "INFO" => Ok(MessageLevel::Info),
        "WARN" => Ok(MessageLevel::Warning),
        "ERROR" => Ok(MessageLevel::Error),
        _ => Err(format!("Invalid message level, expected INFO, WARN or ERROR got {}",level).into_lua_err()),
    }
}

pub async fn info_dialogue(_: mlua::Lua,(title,description,level): (String, String, Option<String>)) -> mlua::Result<()>{
    
    let message_level = string_to_level(level.unwrap_or("INFO".to_string()))?;
    
    
    rfd::AsyncMessageDialog::new()
    .set_title(title)
    .set_description(description)
    .set_level(message_level)
    .show()
    .await;    

    Ok(())
}

pub async fn ok_dialogue(_: mlua::Lua,(title,description,cancellable,level): (String, String,bool, Option<String>)) -> mlua::Result<bool>{
    
    let message_level = string_to_level(level.unwrap_or("INFO".to_string()))?;
    
    let buttons = if cancellable {MessageButtons::OkCancel} else {MessageButtons::Ok};
    
    let result = rfd::AsyncMessageDialog::new()
    .set_title(title)
    .set_description(description)
    .set_level(message_level)
    .set_buttons(buttons)
    .show()
    .await;  

    match result {
        MessageDialogResult::Ok => Ok(true),
        MessageDialogResult::Cancel => Ok(false),
        _ => unreachable!()
    }
}

pub async fn yes_no_dialogue(_: mlua::Lua,(title,description,cancellable,level): (String, String,bool, Option<String>)) -> mlua::Result<Option<bool>>{
    
    let message_level = string_to_level(level.unwrap_or("INFO".to_string()))?;
    
    let buttons = if cancellable {MessageButtons::YesNoCancel} else {MessageButtons::YesNo};
    
    let result = rfd::AsyncMessageDialog::new()
    .set_title(title)
    .set_description(description)
    .set_level(message_level)
    .set_buttons(buttons)
    .show()
    .await;  

    match result {
        MessageDialogResult::Yes => Ok(Some(true)),
        MessageDialogResult::No => Ok(Some(false)),
        MessageDialogResult::Cancel => Ok(None),
        _ => unreachable!()
    }
}

pub async fn file_pick_dialog(_: mlua::Lua,(title,directory,filter): (String,Option<mlua::Value>,Option<mlua::Table>)) -> mlua::Result<Option<vara_files::file_obj::File>>{
    
    let mut file_dialog = rfd::AsyncFileDialog::new()
    .set_title(title);


    if let Some(mlua::Value::UserData(userdata)) = directory {

        if !userdata.is::<vara_files::Directory>(){
            return Err(mlua::Error::FromLuaConversionError { 
                from: "userdata", 
                to: "Directory".to_string(), 
                message: Some("expected a Directory".to_string()) 
            });
        } 

        let dir = userdata.borrow::<vara_files::Directory>()?;

        file_dialog = file_dialog.set_directory(&dir.path);
    }

    
    if let Some(filter) = filter {
        for pair in filter.pairs::<String,Vec<String>>() {
            let (key,value) = pair?;

            file_dialog = file_dialog.add_filter(key, &value);
        }
    }

    let file_handle = file_dialog.pick_file().await;

    match file_handle {
        Some(file_handle) => Ok(Some(vara_files::file_obj::File::new(&PathBuf::from(file_handle.path()))?)),
        None => Ok(None)
    }
}
