use std::sync::Arc;

use mlua::{ExternalError, ExternalResult};
use winit::event::{KeyEvent, WindowEvent};
use winit::event_loop::EventLoop;
use winit::application::ApplicationHandler;
use winit::keyboard::{PhysicalKey};
use winit::window::Window;
use crate::render_state::RenderState;

#[derive(mlua::UserData)]
pub struct UiContext{
    #[lua(skip)]
    state: Option<RenderState>,
    #[lua(skip)]
    exit_error: mlua::Result<()>
}

#[mlua::userdata_impl]
impl UiContext {
    #[lua(skip)]
    pub fn new() -> Self{
        Self{
            state: None,
            exit_error: Ok(())
        }
    }

    #[lua(name="Run")]
    pub fn run(&mut self) -> mlua::Result<()>{

        let event_loop = EventLoop::with_user_event().build().into_lua_err()?;

        event_loop.run_app(self).into_lua_err()?;

        self.exit_error.clone()
    }
}

impl ApplicationHandler<RenderState> for UiContext{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        #[allow(unused_mut)]
        let mut window_attribuites = Window::default_attributes();

        let window = match event_loop.create_window(window_attribuites){
            Ok(win) => win,
            Err(err) => {
                self.exit_error = Err(err.into_lua_err());
                event_loop.exit();

                return;
            }
        };
        
        window.set_title("vara app");

        let arc_window = Arc::new(window);

        let render_state = match pollster::block_on(RenderState::new(arc_window)) {
            Ok(state) => state,
            Err(err) => {
                self.exit_error = Err(err.into_lua_err());
                event_loop.exit();

                return;
            }
        };

        self.state = Some(render_state);
    }

    fn user_event(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop, event: RenderState) {
        self.state = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    )
    {
        let state = match &mut self.state{
            Some(canvas) => canvas,
            None => return,
        };


        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                #[allow(unused)]
                state.render();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => state.handle_key(event_loop, code, key_state.is_pressed()),
            _ => {}
        }
        
    }
}