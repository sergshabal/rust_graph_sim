//#![feature(struct_variant)]
/*
#![feature(globs)]

extern crate gl;
extern crate sdl2;
extern crate native;
*/
/*
use gl::types::*;
use std::mem;
use std::ptr;
use std::str;
*/
/*
mod game;
mod video;

fn main() {	
	video::main();
}*/

extern crate native;
extern crate rsfml;


//use rsfml::system::Vector2f;
//use rsfml::window::{ContextSettings, VideoMode, event, Close};
//use rsfml::graphics::{RenderWindow, CircleShape, Color};

mod geom;
mod game;
/*
#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}*/

mod video;







fn main () -> () {	
    let mut msys=video::MSys::create();
	    

    let mut g=game::GameInstance::create();
    //msys.mouseEvent.register(&g);
    
    while msys.window_is_open() {
        // Handle events
        msys.step(&mut g);
    
    	g.step();
    	
    	g.draw(  &mut msys);
    	
        
        msys.end_step();
        
    }
}