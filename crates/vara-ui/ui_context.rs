use std::sync::Arc;

use mlua::{ExternalError, ExternalResult};
use winit::dpi::LogicalSize;
use winit::event::{KeyEvent, WindowEvent};
use winit::event_loop::EventLoop;
use winit::application::ApplicationHandler;
use winit::keyboard::{PhysicalKey};
use winit::window::Window;
use crate::components::frame::UiFrame;
use crate::components::textlabel::UiTextLabel;
use crate::render_state::RenderState;

use crate::components::{Renderable, UiComponent};


#[derive(mlua::UserData)]
pub struct UiContext{
    #[lua(skip)]
    state: Option<RenderState>,
    #[lua(skip)]
    exit_error: mlua::Result<()>,
    #[lua(skip)]
    ui_components: Vec<UiComponent>,
    #[lua(skip)]
    window_width: u32,
    #[lua(skip)]
    window_height: u32,
    #[lua(skip)]    
    resizable: bool,
    #[lua(skip)] 
    clear_colour: [f64;4],
}

#[mlua::userdata_impl]
impl UiContext {
    #[lua(skip)]
    pub fn new() -> Self{
        Self{
            state: None,
            exit_error: Ok(()),
            ui_components: Vec::new(),
            window_width: 600,
            window_height: 800,
            resizable: true,
            clear_colour: [0.01,0.01,0.01,1.0],
        }
    }

    #[lua(name="Run")]
    pub fn run(&mut self) -> mlua::Result<()>{

        let event_loop = EventLoop::with_user_event().build().into_lua_err()?;

        event_loop.run_app(self).into_lua_err()?;

        self.exit_error.clone()
    }

    #[lua(skip)]
    pub fn render(&mut self) -> mlua::Result<()>{
        let state = match &mut self.state{
            Some(canvas) => canvas,
            None => return Ok(()),
        };

        for component in self.ui_components.iter_mut(){
            component.render(state)?;
        }

        state.render()?;

        Ok(())
    }

    #[lua(name="Frame")]
    pub fn frame(&mut self,context: &mlua::Lua) -> mlua::Result<mlua::AnyUserData>{
        let frame = UiFrame::new();

        self.ui_components.push(UiComponent::Frame(frame.clone()));

        context.create_userdata(frame)
    }

    #[lua(name="TextLabel")]
    pub fn textlabel(&mut self,context: &mlua::Lua) -> mlua::Result<mlua::AnyUserData>{
        let text = UiTextLabel::new();

        self.ui_components.push(UiComponent::TextLabel(text.clone()));

        context.create_userdata(text)
    }

    #[lua(setter,name="Width")]
    pub fn set_width(&mut self,width: u32) -> mlua::Result<()>{
        self.window_width = width;
        if let Some(state) = &self.state {
            #[allow(unused)]
            state.window.request_inner_size(LogicalSize::new(self.window_width, self.window_height));
        }
        Ok(())
    }

    #[lua(setter,name="Height")]
    pub fn set_height(&mut self,height: u32) -> mlua::Result<()>{
        self.window_height = height;
        if let Some(state) = &self.state {
            #[allow(unused)]
            state.window.request_inner_size(LogicalSize::new(self.window_width, self.window_height));
        }
        Ok(())
    }

    #[lua(setter,name="Resizable")]
    pub fn set_resizable(&mut self,resizable: bool) -> mlua::Result<()>{
        self.resizable = resizable;
        if let Some(state) = &self.state {
            #[allow(unused)]
            state.window.set_resizable(resizable);
        }
        Ok(())
    }

    #[lua(setter,name="BackgroundColour")]
    pub fn set_background_colour(&mut self,colour: [f64;4]) -> mlua::Result<()>{
        self.clear_colour = colour.clone();
        if let Some(state) = &mut self.state {
            state.clear_colour = colour.clone();
        }
        Ok(())
    }

    #[lua(getter,name="Width")]
    pub fn get_width(&self) -> mlua::Result<u32>{
        Ok(self.window_width)
    }

    #[lua(getter,name="Height")]
    pub fn get_height(&self) -> mlua::Result<u32>{
        Ok(self.window_height)
    }

    #[lua(getter,name="Resizable")]
    pub fn get_resizable(&self) -> mlua::Result<bool>{
        Ok(self.resizable)
    }

    #[lua(getter,name="BackgroundColour")]
    pub fn get_background_colour(&self) -> mlua::Result<[f64;4]>{
        Ok(self.clear_colour)
    }

}

impl ApplicationHandler<RenderState> for UiContext{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {

        let size = LogicalSize::new(self.window_width, self.window_height);

        #[allow(unused_mut)]
        let mut window_attribuites = Window::default_attributes()
            .with_resizable(self.resizable)
            .with_inner_size(size)
            .with_title("vara app");

        let window = match event_loop.create_window(window_attribuites){
            Ok(win) => win,
            Err(err) => {
                self.exit_error = Err(err.into_lua_err());
                event_loop.exit();

                return;
            }
        };

        let arc_window = Arc::new(window);

        let mut render_state = match pollster::block_on(RenderState::new(arc_window)) {
            Ok(state) => state,
            Err(err) => {
                self.exit_error = Err(err.into_lua_err());
                event_loop.exit();

                return;
            }
        };

        render_state.clear_colour = self.clear_colour;

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
            WindowEvent::Resized(size) => {

                if !self.resizable || !state.window.is_resizable() {
                    #[allow(unused)]
                    state.window.request_inner_size(LogicalSize::new(self.window_width,self.window_height));
                }

                state.resize(size.width, size.height)
            },
            WindowEvent::RedrawRequested => {
                #[allow(unused)]
                self.render();
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