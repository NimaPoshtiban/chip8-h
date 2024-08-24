use sdl2::sys::*;
use std::ffi::{CString,c_void};

use std::ptr::null;
pub struct Platform {
    window: *mut SDL_Window,
    renderer: *mut SDL_Renderer,
    texture: *mut SDL_Texture,
}

impl Platform {
    pub fn new(title: String, width: i32, height: i32,texture_width:i32,texture_height:i32) -> Self {
        let window = unsafe {
            SDL_CreateWindow(
                CString::new(title).unwrap().as_ptr(),
                0,
                0,
                width.into(),
                height.into(),
                SDL_WindowFlags::SDL_WINDOW_SHOWN as u32,
            )
        };
        let renderer = unsafe{
            SDL_CreateRenderer(window, -1, SDL_RendererFlags::SDL_RENDERER_ACCELERATED as u32)
        };
        let texture = unsafe{
            SDL_CreateTexture(renderer, SDL_PixelFormatEnum::SDL_PIXELFORMAT_RGBA8888 as u32, SDL_TextureAccess::SDL_TEXTUREACCESS_STREAMING as i32, texture_width, texture_height)
        };
        Platform{window,renderer,texture}
    }
    
    pub unsafe fn update(&mut self,buffer:*const c_void,pitch:i32){
        SDL_UpdateTexture(self.texture, null(), buffer, pitch);
		SDL_RenderClear(self.renderer);
		SDL_RenderCopy(self.renderer, self.texture, null(), null());
		SDL_RenderPresent(self.renderer);
    }
    
    pub unsafe fn destroy(&mut self){
        SDL_DestroyTexture(self.texture);
        SDL_DestroyRenderer(self.renderer);
		SDL_DestroyWindow(self.window);
		SDL_Quit();
    }
    pub unsafe  fn process(&mut self,keys:*mut i8)->bool{
        let mut quit = false;

		let event: *mut SDL_Event= std::ptr::null_mut();

		while SDL_PollEvent(event) != 0
		{
			match event
			{
				 SDL_QUIT =>
				{
					quit = true;
				} ,

				 SDL_KEYDOWN =>
				{
					match (*(event)).key.keysym.sym
					{
						 SDLK_ESCAPE =>
						{
							quit = true;
						},

						SDLK_x =>
						{
							*(keys.wrapping_add(0)) = 1;
						},

						SDLK_1 =>
						{
							*(keys.wrapping_add(1)) = 1;
						},

						 SDLK_2 =>
						{
							*(keys.wrapping_add(2)) = 1;
						} ,

						 SDLK_3 =>
						{
							*(keys.wrapping_add(3)) = 1;
						} ,

						 SDLK_q =>
						{
							*(keys.wrapping_add(4)) = 1;
						} ,

						 SDLK_w =>
						{
							*(keys.wrapping_add(5)) = 1;
						} ,

						 SDLK_e =>
						{
							*(keys.wrapping_add(6)) = 1;
						} ,

						 SDLK_a =>
						{
							*(keys.wrapping_add(7)) = 1;
						} ,

						 SDLK_s =>
						{
							*(keys.wrapping_add(8)) = 1;
						} ,

						 SDLK_d =>
						{
							*(keys.wrapping_add(9)) = 1;
						} ,

						 SDLK_z =>
						{
							*(keys.wrapping_add(0xA)) = 1;
						} ,

						 SDLK_c =>
						{
							*(keys.wrapping_add(0xB)) = 1;
						} ,

						 SDLK_4 =>
						{
							*(keys.wrapping_add(0xC)) = 1;
						} ,

						 SDLK_r =>
						{
							*(keys.wrapping_add(0xD)) = 1;
						} ,

						 SDLK_f =>
						{
							*(keys.wrapping_add(0xE)) = 1;
						} ,

						 SDLK_v =>
						{
							*(keys.wrapping_add(0xF)) = 1;
						} ,
					}
				} ,

				 SDL_KEYUP =>
				{
					match (*(event)).key.keysym.sym
					{
						 SDLK_x =>
						{
							*(keys.wrapping_add(0)) = 0;
						} ,

						 SDLK_1 =>
						{
							*(keys.wrapping_add(1)) = 0;
						} ,

						 SDLK_2 =>
						{
							*(keys.wrapping_add(2)) = 0;
						} ,

						 SDLK_3 =>
						{
							*(keys.wrapping_add(3)) = 0;
						} ,

						 SDLK_q =>
						{
							*(keys.wrapping_add(4)) = 0;
						} ,

						 SDLK_w =>
						{
							*(keys.wrapping_add(5)) = 0;
						} ,

						 SDLK_e =>
						{
							*(keys.wrapping_add(6)) = 0;
						} ,

						 SDLK_a =>
						{
							*(keys.wrapping_add(7)) = 0;
						} ,

						 SDLK_s =>
						{
							*(keys.wrapping_add(8)) = 0;
						} ,

						 SDLK_d =>
						{
							*(keys.wrapping_add(9)) = 0;
						} ,

						 SDLK_z =>
						{
							*(keys.wrapping_add(0xA)) = 0;
						} ,

						 SDLK_c =>
						{
							*(keys.wrapping_add(0xB)) = 0;
						} ,

						 SDLK_4 =>
						{
							*(keys.wrapping_add(0xC)) = 0;
						} ,

						 SDLK_r =>
						{
							*(keys.wrapping_add(0xD)) = 0;
						} ,

						 SDLK_f =>
						{
							*(keys.wrapping_add(0xE)) = 0;
						} ,

						 SDLK_v =>
						{
							*(keys.wrapping_add(0xF)) = 0;
						} ,
					}
				} ,
			}
		}

		return quit;
    }
    

}
