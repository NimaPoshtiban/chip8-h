use sdl2::libc::c_int;
use sdl2::{sys::*};
use std::ffi::{CString,c_void};

use std::ptr::null;
pub struct Platform {
    window: *mut SDL_Window,
    renderer: *mut SDL_Renderer,
    texture: *mut SDL_Texture,
}

impl Platform {
    pub fn new(title: String, width: i32, height: i32,texture_width:i32,texture_height:i32) -> Self {
		unsafe{SDL_Init(SDL_INIT_VIDEO);}
		
        let window = unsafe {
			SDL_CreateWindow(
				CString::new(title).unwrap().as_ptr(),
				SDL_WINDOWPOS_CENTERED_MASK as i32,
				SDL_WINDOWPOS_CENTERED_MASK as i32,
				width.into(),
				height.into(),
				SDL_WindowFlags::SDL_WINDOW_SHOWN as u32,
			)
        };
        let renderer = unsafe{
            SDL_CreateRenderer(window, -1, SDL_RendererFlags::SDL_RENDERER_ACCELERATED as u32)
        };
        let texture = unsafe{
            SDL_CreateTexture(renderer, SDL_PixelFormatEnum::SDL_PIXELFORMAT_RGBA8888 as u32, SDL_TextureAccess::SDL_TEXTUREACCESS_STREAMING as c_int, texture_width as c_int, texture_height as c_int)
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

		let mut event = std::mem::MaybeUninit::<SDL_Event>::uninit();

		while unsafe { SDL_PollEvent(event.as_mut_ptr()) } != 0
		{
			let event: SDL_Event = unsafe { event.assume_init() };
			
			match event.type_ {
				x if x==(sdl2::sys::SDL_EventType::SDL_QUIT as u32)  => {
					quit = true;
				},
	
				x if x== (sdl2::sys::SDL_EventType::SDL_KEYDOWN as u32) => {
					match unsafe { event.key.keysym.sym } {
						x if x==(sdl2::sys::SDL_KeyCode::SDLK_ESCAPE as i32) => {
							quit = true;
						},
	
						x if x == (sdl2::sys::SDL_KeyCode::SDLK_x as i32) => {
							*(keys.wrapping_add(0)) = 1;
						},
	
						x if x == (sdl2::sys::SDL_KeyCode::SDLK_1 as i32) => {
							*(keys.wrapping_add(1)) = 1;
						},
	
						x if x == (sdl2::sys::SDL_KeyCode::SDLK_2 as i32) => {
							*(keys.wrapping_add(2)) = 1;
						},
	
						x if x == (sdl2::sys::SDL_KeyCode::SDLK_3 as i32) => {
							*(keys.wrapping_add(3)) = 1;
						},
	
						x if x == (sdl2::sys::SDL_KeyCode::SDLK_q as i32) => {
							*(keys.wrapping_add(4)) = 1;
						},
	
						x if x == (sdl2::sys::SDL_KeyCode::SDLK_w as i32) => {
							*(keys.wrapping_add(5)) = 1;
						},
	
						x if x == (sdl2::sys::SDL_KeyCode::SDLK_e as i32) => {
							*(keys.wrapping_add(6)) = 1;
						},
	
						x if x == (sdl2::sys::SDL_KeyCode::SDLK_a as i32) => {
							*(keys.wrapping_add(7)) = 1;
						},
	
						x if x==(sdl2::sys::SDL_KeyCode::SDLK_s as i32) => {
							*(keys.wrapping_add(8)) = 1;
						},
	
						x if x==(sdl2::sys::SDL_KeyCode::SDLK_d as i32) => {
							*(keys.wrapping_add(9))  = 1;
						},
	
						x if x==(sdl2::sys::SDL_KeyCode::SDLK_z as i32) => {
							*(keys.wrapping_add(0xA)) = 1;
						},
	
						x if x==(sdl2::sys::SDL_KeyCode::SDLK_c as i32) => {
							*(keys.wrapping_add(0xB)) = 1;
						},
	
						x if x==(sdl2::sys::SDL_KeyCode::SDLK_4 as i32) => {
							*(keys.wrapping_add(0xC))= 1;
						},
	
						x if x==(sdl2::sys::SDL_KeyCode::SDLK_r as i32) => {
							*(keys.wrapping_add(0xD))  = 1;
						},
	
						x if x==(sdl2::sys::SDL_KeyCode::SDLK_f as i32)=> {
							*(keys.wrapping_add(0xE)) = 1;
						},
	
						x if x==(sdl2::sys::SDL_KeyCode::SDLK_v as i32)=> {
							*(keys.wrapping_add(0xF))  = 1;
						},
	
						_ => {} // Handle other keys if needed
					}
				},
	
				x if x == (sdl2::sys::SDL_EventType::SDL_KEYUP as u32)   => {
					match unsafe { event.key.keysym.sym } {
						x if x == (sdl2::sys::SDL_KeyCode::SDLK_x as i32) => {
							*(keys.wrapping_add(0)) = 0;
						},
	
						x if x == (sdl2::sys::SDL_KeyCode::SDLK_1 as i32) => {
							*(keys.wrapping_add(1)) = 0;
						},
	
						x if x == (sdl2::sys::SDL_KeyCode::SDLK_2 as i32) => {
							*(keys.wrapping_add(2)) = 0;
						},
	
						x if x == (sdl2::sys::SDL_KeyCode::SDLK_3 as i32) => {
							*(keys.wrapping_add(3)) = 0;
						},
	
						x if x == (sdl2::sys::SDL_KeyCode::SDLK_q as i32) => {
							*(keys.wrapping_add(4)) = 0;
						},
	
						x if x == (sdl2::sys::SDL_KeyCode::SDLK_w as i32) => {
							*(keys.wrapping_add(5)) = 0;
						},
	
						x if x == (sdl2::sys::SDL_KeyCode::SDLK_e as i32) => {
							*(keys.wrapping_add(6)) = 0;
						},
	
						x if x == (sdl2::sys::SDL_KeyCode::SDLK_a as i32) => {
							*(keys.wrapping_add(7)) = 0;
						},
	
						x if x == (sdl2::sys::SDL_KeyCode::SDLK_s as i32) => {
							*(keys.wrapping_add(8)) = 0;
						},
	
						x if x == (sdl2::sys::SDL_KeyCode::SDLK_d as i32) => {
							*(keys.wrapping_add(9)) = 0;
						},
	
						x if x == (sdl2::sys::SDL_KeyCode::SDLK_z as i32) => {
							*(keys.wrapping_add(0xA)) = 0;
						},
	
						x if x == (sdl2::sys::SDL_KeyCode::SDLK_c as i32) => {
							*(keys.wrapping_add(0xB)) = 0;
						},
	
						x if x == (sdl2::sys::SDL_KeyCode::SDLK_4 as i32) => {
							*(keys.wrapping_add(0xC)) = 0;
						},
	
						x if x == (sdl2::sys::SDL_KeyCode::SDLK_r as i32) => {
							*(keys.wrapping_add(0xD)) = 0;
						},
	
						x if x == (sdl2::sys::SDL_KeyCode::SDLK_f as i32) => {
							*(keys.wrapping_add(0xE)) = 0;
						},
	
						x if x == (sdl2::sys::SDL_KeyCode::SDLK_v as i32) => {
							*(keys.wrapping_add(0xF)) = 0;
						},
	
						_ => {} // Handle other keys if needed
					}
				},
	
				_ => {} // Handle other event types if needed
			}
		}
	
		quit
    }
    

}
