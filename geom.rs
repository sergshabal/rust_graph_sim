
use std::rand::{task_rng, Rng};




pub struct V2f{
	pub x:f32,
	pub y:f32
}
impl V2f{
	pub fn new(x:f32,y:f32)->V2f{
		V2f{x:x,y:y}
	}
	pub fn new_int(x:i32,y:i32)->V2f{
		V2f{x:x as f32,y:y as f32}
	}
	pub fn len_sqr(&self)->f32{
		self.x*self.x+self.y*self.y
	}
	pub fn rot90deg(&self)->V2f{
		V2f{x:-self.y,y:self.x}
	}
}

impl Add<V2f,V2f> for V2f {
    fn add(&self, rhs: &V2f) -> V2f {
    	let x=self.x+rhs.x;
    	let y=self.y+rhs.y;
    	V2f{x:x,y:y}             	
    }
}
impl Sub<V2f,V2f> for V2f {
    fn sub(&self, rhs: &V2f) -> V2f {
    	let x=self.x-rhs.x;
    	let y=self.y-rhs.y;
    	V2f{x:x,y:y}             	
    }
}

impl Div<f32,V2f> for V2f {
    fn div(&self, rhs: &f32) -> V2f {
    	let x=self.x/ *rhs;
    	let y=self.y/ *rhs;
    	V2f{x:x,y:y}             	
    }
}
impl Mul<f32,V2f> for V2f {
    fn mul(&self, rhs: &f32) -> V2f {
    	let x=self.x* *rhs;
    	let y=self.y* *rhs;
    	V2f{x:x,y:y}             	
    }
}


pub fn max(a:int,b:int)->int{
	if a>b {
		a
	}else{
		b
	}
}


pub struct Rect{
	pub tl:V2f,
	pub dim:V2f
}
impl Rect{
	pub fn new_int(x:i32,y:i32,dimx:i32,dimy:i32)->Rect{
		Rect{tl:V2f::new_int(x,y),dim:V2f::new_int(dimx,dimy)}
	}
	pub fn random_pos(&self)->V2f{
		let mut rng = task_rng();
		/*
		let n: uint = rng.gen_range(0u, 10);
		println!("{}", n);
		*/
		
		let m: f32 = rng.gen_range(self.tl.x,self.tl.x+self.dim.x );
		let n: f32 = rng.gen_range(self.tl.y,self.tl.y+self.dim.y);
		
		//println!("{}", m);

		V2f::new(m,n)
	}
}