use std::{collections::HashMap};

use crate::win_ui::CallbackSlot;

pub struct EventManager{
    on_close: HashMap<usize,CallbackSlot>,
    on_resize: HashMap<usize,CallbackSlot>
}

pub enum EventType{
    CLOSE,
    RESIZE
}

impl EventManager {
    pub fn new() -> Self{
        Self {
            on_close: HashMap::new(),
            on_resize: HashMap::new()
        }
    }

    pub fn get(&self,context: &mlua::Lua,event_type: EventType,id: usize) -> mlua::Result<Option<mlua::Function>>{
        let map = match event_type {
            EventType::CLOSE => &self.on_close,
            EventType::RESIZE => &self.on_resize,
        };

        if let Some(slot) = map.get(&id) {
            return get_callback(context, slot)
        }

        Ok(None)
    }

    pub fn set(&mut self,context: &mlua::Lua,event_type: EventType,id: usize,callback_fn: mlua::Function) -> mlua::Result<()>{
        let map = match event_type {
            EventType::CLOSE => &self.on_close,
            EventType::RESIZE => &self.on_resize,
        };


        if let Some(slot) = map.get(&id) {
            return set_callback(context, slot,callback_fn)
        }

        Ok(())
    }

    pub fn register(&mut self,event_type: EventType,id: usize,slot: CallbackSlot) -> mlua::Result<()>{
        let map = match event_type {
            EventType::CLOSE => &mut self.on_close,
            EventType::RESIZE => &mut self.on_resize,
        };

        map.insert(id,slot);

        Ok(())
    }

}

fn get_callback(context: &mlua::Lua,slot: &CallbackSlot) -> mlua::Result<Option<mlua::Function>>{
    let slot = slot.borrow();

    match slot.as_ref() {
        Some(key) => context.registry_value(key).map(Some),
        None => Ok(None)
    }
}

fn set_callback(context: &mlua::Lua,slot: &CallbackSlot,callback_fn: mlua::Function) -> mlua::Result<()>{
    let mut slot = slot.borrow_mut();

    if let Some(key) = slot.as_mut(){
        context.replace_registry_value(key, callback_fn)
    } else {
        *slot = Some(context.create_registry_value(callback_fn)?);
        Ok(())
    }
}