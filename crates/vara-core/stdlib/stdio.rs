use std::{io::{self, Read, Write}, sync::OnceLock};
use mlua::ExternalResult;
use smol::channel::{self, Receiver};
use vara_util::table_builder::TableBuilder;

static INPUT: OnceLock<Receiver<u8>> = OnceLock::new();


fn write(_: &mlua::Lua, arg: mlua::LuaString) -> Result<(), mlua::error::Error> {
    let message = arg.as_bytes();

    io::stdout().write_all(&message)?;
    Ok(())
}

fn read(
    context: &mlua::Lua,
    chunk_size: Option<usize>,
) -> Result<mlua::LuaString, mlua::error::Error> {
    io::stdout().flush().map_err(mlua::Error::external)?;

    let mut stdin = io::stdin().lock();
    let size = chunk_size.unwrap_or(1024);
    let mut buffer = vec![0; size];

    let bytes_read = stdin.read(&mut buffer).into_lua_err()?;

    buffer.truncate(bytes_read);

    context.create_string(&buffer)
}

fn readline(context: &mlua::Lua, _: ()) -> Result<mlua::LuaString, mlua::error::Error> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    context.create_string(&input.trim())
}

fn flush(_: &mlua::Lua, () : ()) -> mlua::Result<()>{
    io::stdout().flush()?;
    Ok(())
}

fn input_reciever() -> Receiver<u8> {
    INPUT.get_or_init(|| {
        let (sender,reciever) = channel::bounded::<u8>(1024);

        std::thread::spawn(move || {
            let mut stdin = io::stdin().lock();
            let mut byte = [0u8; 1];

            loop {
                match stdin.read_exact(&mut byte) {
                    Ok(_) => if sender.send_blocking(byte[0]).is_err() {
                        break;
                    },
                    Err(_) => break,
                }
            }
        });

        reciever
    }).clone()
}

async fn read_async(context: mlua::Lua,chunk_size: Option<usize>) -> mlua::Result<mlua::LuaString> {
    let size = chunk_size.unwrap_or(1024);

    if size == 0 {
        return context.create_string([]);
    }

    let reciever = input_reciever();
    let mut buffer = Vec::with_capacity(size);

    match reciever.recv().await {
        Ok(byte) => buffer.push(byte),
        Err(_) => return context.create_string([])
    }

    while buffer.len() < size {
        match reciever.try_recv() {
            Ok(byte) => buffer.push(byte),
            Err(_) => break,
        }
    }

    context.create_string(buffer)
}


pub fn get_module(context: &mlua::Lua) -> mlua::Result<mlua::Table> {
    TableBuilder::new(context.clone())?
        .add_function("Write", write)?
        .add_function("Read", read)?
        .add_function("ReadLine", readline)?
        .add_function("Flush", flush)?
        .add_async_function("ReadAsync", read_async)?
        .build_readonly()
}
