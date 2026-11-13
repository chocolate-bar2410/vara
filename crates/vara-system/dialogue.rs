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
        _ => Ok(false)
    }
}


