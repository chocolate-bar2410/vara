use std::{io::Cursor, path::PathBuf};
use mlua::{ExternalError, IntoLua};
use symphonia::core::{formats::{TrackType, probe::Hint}, io::MediaSourceStream};

use symphonia::core::errors::Error as symphonia_error;

use vara_util::table_builder::TableBuilder;

use vara_files::file_obj::File as vara_file;

mod image_obj;
mod image_format;
mod image_interpolation;

mod audio_obj;
mod audio_interpolation;

fn create_image(context : &mlua::Lua,data: mlua::Value) -> mlua::Result<image_obj::Image>{
    match data {
        mlua::Value::UserData(userdata) => {
            if !userdata.is::<vara_file>(){
                return Err(mlua::Error::FromLuaConversionError { 
                    from: "userdata", 
                    to: "file".to_string(), 
                    message: Some("expected a string, buffer or file".to_string()) 
                });
            } 

            let file = userdata.borrow::<vara_file>()?;
            let filepath = file.path.to_string_lossy();
            
            create_image(context,filepath.into_lua(context)?)
        },
        mlua::Value::String(s) => {
            let bytes = vara_files::read_file_as_binary(&PathBuf::from(s.to_string_lossy()))?;

            let reader = image::ImageReader::new(Cursor::new(bytes))
                .with_guessed_format()?;

            let format = reader.format();

            let img = reader
                .decode()
                .map_err(|err| err.into_lua_err())?;

            let data = match format {
                Some(image::ImageFormat::Jpeg) => img.to_rgb8().into_raw(),
                _ => img.to_rgba8().into_raw(),
            };

            let colour_channels = match format {
                Some(image::ImageFormat::Jpeg) => 3,
                _ => 4,
            };

            Ok(image_obj::Image::new(data, img.width(), img.height(),colour_channels))
        },
        mlua::Value::Buffer(b) => {
            let bytes = b.to_vec();
            Ok(image_obj::Image::new(bytes, b.len() as u32,1,4))
        }
        _ => {
            return Err(mlua::Error::FromLuaConversionError {
                from: data.type_name(),
                to: "string or buffer".to_string(),
                message: Some("expected a string, buffer or file".to_string()),
            });
        }
    }
}

fn create_audio(context : &mlua::Lua,data: mlua::Value) -> mlua::Result<audio_obj::Audio>{
    match data {
        mlua::Value::UserData(userdata) => {
            if !userdata.is::<vara_file>(){
                return Err(mlua::Error::FromLuaConversionError { 
                    from: "userdata", 
                    to: "file".to_string(), 
                    message: Some("expected a string, file or table".to_string()) 
                });
            } 

            let file = userdata.borrow::<vara_file>()?;
            let filepath = file.path.to_string_lossy();
            
            create_audio(context,filepath.into_lua(context)?)
        },
        mlua::Value::String(s) => {
            let path = PathBuf::from(s.to_string_lossy());
            let bytes = vara_files::read_file_as_binary(&path)?;


            let mss = MediaSourceStream::new(Box::new(Cursor::new(bytes)), Default::default());

            let mut hint = Hint::new();
            if let Some(extension) = path.extension().and_then(|ext| ext.to_str()){
                hint.with_extension(extension);    
            }
            
            let mut probed = symphonia::default::get_probe()
                .probe(&hint, mss, Default::default(), Default::default())
                .map_err(|err| err.into_lua_err())?;

            let track = probed.default_track(TrackType::Audio)
                .ok_or("file has no tracks")
                .map_err(|err| err.into_lua_err())?;

            let codec_params = track.codec_params
                .as_ref()
                .ok_or("could not get codec parameters")
                .map_err(|err| err.into_lua_err())?;

            let audio_params = codec_params
                .audio()
                .ok_or("could not get audio parameters")
                .map_err(|err| err.into_lua_err())?;

            let sample_rate = audio_params.sample_rate
                .ok_or("could not get sample rate")
                .map_err(|err| err.into_lua_err())?;

            let mut frames = 0;

            let mut decoder = symphonia::default::get_codecs()
            .make_audio_decoder(
                track.codec_params
                    .as_ref()
                    .ok_or("codec parameters missing")
                    .map_err(|err| err.into_lua_err())?
                    .audio()
                    .unwrap(),
                &Default::default()
            ).map_err(|err| err.into_lua_err())?;

            let track_id = track.id;
            let mut tracks : Vec<Vec<f32>> = Vec::new();
            let mut packet_tracks : Vec<Vec<f32>> = Vec::new();

            loop {
                let packet = match probed.next_packet() {
                    Ok(Some(packet)) => packet,
                    Ok(None) => {
                        break;
                    }
                    Err(symphonia_error::ResetRequired) => {
                        // for chained OGG physical streams.
                        // will implement this later
                        unimplemented!();
                    }
                    Err(err) => {
                        // A unrecoverable error occurred, halt decoding.
                        return Err(format!("{}", err).into_lua_err());
                    }
                };

                // Consume any new metadata that has been read since the last packet.
                while !probed.metadata().is_latest() {
                    // Pop the old head of the metadata queue.
                    probed.metadata().pop();

                    // Consume the new metadata at the head of the metadata queue.
                }

                // If the packet does not belong to the selected track, skip over it.
                if packet.track_id != track_id {
                    continue;
                }

                // Decode the packet into audio samples.
                let decoded = match decoder.decode(&packet) {
                    Ok(_decoded) => _decoded,
                    Err(symphonia_error::IoError(_)) => {
                        // The packet failed to decode due to an IO error, skip the packet.
                        continue;
                    }
                    Err(symphonia_error::DecodeError(_)) => {
                        // The packet failed to decode due to invalid data, skip the packet.
                        continue;
                    }
                    Err(err) => {
                        // An unrecoverable error occurred, halt decoding.
                        return Err(format!("{}", err).into_lua_err());
                    }
                };

                frames += decoded.frames();
                decoded.copy_to_vecs_planar(&mut packet_tracks);

                if tracks.is_empty(){
                    tracks = vec![Vec::new();packet_tracks.len()];
                }

                for (child_index,channel_data) in packet_tracks.iter().enumerate(){
                    tracks[child_index].extend(channel_data);
                }
            }
        
            audio_obj::Audio::new(tracks,sample_rate,frames as u64)
        }
        mlua::Value::Table(tab) => {
            let track_table : mlua::Table = tab.get::<mlua::Table>("Tracks")?;
            let sample_rate : u32 = tab.get::<u32>("SampleRate")?;
            let frames : u64 = tab.get::<u64>("Frames")?;

            let mut tracks : Vec<Vec<f32>> = Vec::new();

            for value in track_table.sequence_values::<mlua::Buffer>(){
                let track_buffer = value?;
                let bytes : Vec<u8> = track_buffer.to_vec();

                let float_bytes : Vec<f32> = bytes.chunks_exact(4).map(|chunk| f32::from_ne_bytes(chunk.try_into().unwrap())).collect();

                tracks.push(float_bytes);
            }

            audio_obj::Audio::new(tracks,sample_rate,frames)
        },
        _ => {
            return Err(mlua::Error::FromLuaConversionError {
                from: data.type_name(),
                to: "string or buffer".to_string(),
                message: Some("expected a string, file or table".to_string()),
            });
        }
    }
}

pub fn get_module(context: &mlua::Lua) -> mlua::Result<mlua::Table> {
    TableBuilder::new(context.clone())?
    .add_function("Image", create_image)?
    .add_function("Audio", create_audio)?
    .build_readonly()
}