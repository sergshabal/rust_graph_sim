use rsfml::system::Vector2f;
use rsfml::window::{ContextSettings, VideoMode, event, Close};
use rsfml::graphics::{RenderWindow, Color,Font,Text};

use rsfml::window::mouse;

use geom;
/*
pub struct DrawThing{
    pub points:Vec<sdl2::rect::Point>,
    pub r:u8,
    pub g:u8,
    pub b:u8
}
impl DrawThing{
    fn create(r:u8,g:u8,b:u8)->DrawThing{
        DrawThing{r:r,g:g,b:b,points:Vec::new()}
    }
    fn add_line(&mut self,p1:sdl2::rect::Point,p2:sdl2::rect::Point){
        self.points.push(p1);
        self.points.push(p2);
    }
    

}
pub struct DrawSys{
    pub draws:Vec<DrawThing>
}
impl DrawSys{
    pub fn create()->DrawSys{
        DrawSys{draws:Vec::new()}
    }
    fn addDrawThing(&mut self,thing:DrawThing)->&mut DrawThing{
        self.draws.push(thing);
        let s=self.draws.len()-1;
        return self.draws.get_mut(s);
        
    }
}
*/
pub trait MMouseEventListener{
    fn on_mouse_down(&mut self,pos:geom::V2f);
    fn on_mouse_up(&mut self,pos:geom::V2f);
}
/*
pub struct MMouseEvent<'a>{
    listeners:Vec<&'a MMouseEventListener>
}
impl<'a> MMouseEvent<'a>{
    pub fn create()->MMouseEvent<'a>{
        MMouseEvent{listeners:Vec::new()}
    }
    pub fn register(&mut self,lis:&'a MMouseEventListener){
        self.listeners.push(lis);
    }
    fn fire_mouse_down(&self,pos:geom::V2f){
        for n in self.listeners.iter(){
            n.on_mouse_down(pos);
        }        
    }
    fn fire_mouse_up(&self,pos:geom::V2f){
        for n in self.listeners.iter(){
            n.on_mouse_up(pos);
        }
    }

}*/

pub struct MSys<'a>{
    //pub mouseEvent:MMouseEvent<'a>,
    pub window:RenderWindow,
    ffont:Font   
}
impl<'a> MSys<'a>{
    pub fn create()->MSys<'a>{

        let cont=ContextSettings{depth_bits:0,stencil_bits:0,antialiasing_level:4,major_version:2,minor_version:0};
        let mut window = match RenderWindow::new(VideoMode::new_init(800, 600, 32),
                                             "SFML Example",
                                             Close,
                                             &cont) {
        Some(window) => window,
        None => fail!("Cannot create a new Render Window.")
        };  

        window.set_framerate_limit(60);
        let font = match Font::new_from_file("res/font2.ttf") {
            Some(font)    => font,
            None          => fail!("Error, cannot load font")
        };

         
        
        MSys{window:window,ffont:font}
                

    }
    
    pub fn draw_text(&mut self,te:&str,pos:&geom::V2f){
        
        // Initialize the pause message
        let mut pauseMessage: Text = match Text::new() {
            Some(text) => text,
            None       => fail!("Error on creating text")
        };
        pauseMessage.set_font(&self.ffont);

        pauseMessage.set_character_size(12);
        pauseMessage.set_position(&Vector2f::new(pos.x,pos.y));
        pauseMessage.set_color(&Color::white());
        pauseMessage.set_string(te);
        self.window.draw(&pauseMessage);
        
    }
    pub fn window_is_open(&self)->bool{
        self.window.is_open()
    }
    pub fn step(&mut self,lis:&mut MMouseEventListener){
        for event in self.window.events() {
            match event {
                event::Closed => self.window.close(),
                
                event::MouseButtonPressed{button,x,y} => {                    
                    match button{
                        mouse::MouseLeft => lis.on_mouse_down(geom::V2f::new_int(x,y)),                        
                        _ => {}
                    }                    
                },
                event::MouseButtonReleased{button,x,y} => {                    
                    match button{
                        mouse::MouseLeft => lis.on_mouse_up(geom::V2f::new_int(x,y)),                        
                        _ => {}
                    }                    
                },

                _             => {/* do nothing */}
            }
        }

        // Clear the window
        self.window.clear(&Color::new_RGB(0, 200, 200));
    }
    pub fn end_step(&mut self){
        self.window.display()
    }   
}
