
use std::ptr;
use std::fmt;
use std::rc::Rc;
use rsfml::system::Vector2f;
use rsfml::graphics::{ CircleShape, Color,ConvexShape};
use std::rand::{task_rng, Rng};
use video::{MSys,MMouseEventListener};
use geom;

//use std::num;



struct TraderID{
	id:int	
}
impl TraderID{
	fn new(id:int)->TraderID{
		TraderID{id:id}
	}
	fn equals(&self,b:TraderID)->bool{
		b.id==self.id
	}
	fn empty()->TraderID{
		TraderID::new(-1)
	}
}



struct WorldID{
	id:int
}
impl WorldID{
	fn new(id:int)->WorldID{
		WorldID{id:id}
	}
	fn equals(&self,b:WorldID)->bool{
		b.id==self.id
	}
	fn empty()->WorldID{
		WorldID::new(-1)
	}
}
impl fmt::Show for WorldID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {    	
        write!(f, "{}",self.id)
    }
}
impl fmt::Show for TraderID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {    	
        write!(f, "{}",self.id)
    }
}


trait Selectable{
	fn get_pos(&self)->geom::V2f;	
}

struct Trader{
	id:TraderID,
	movingPriceAvg:ResArr<f32>,
	resources:ResArr<int>,
	money:int,
	pos:geom::V2f
}
impl Trader{
	/*
	fn updatePrices(&self,world:&World){
		weight=0.4;
		perPriceAvg=perPriceAvg*(1-width)+world.price*weight;
	}
	fn trade(&self,world:&World){
		if(world.prices>self.perPriceAvg){
			sell();
		}
	}*/
	
	fn draw(pos:geom::V2f,sys:&mut MSys){
		let mut circle = match CircleShape::new() {
	        Some(circle) => circle,
	        None       => fail!("Error, cannot create ball")
	    };
	    let rad=2.0;
	    circle.set_radius(rad);	    
	    circle.set_origin(&Vector2f::new(rad,rad));
	    circle.set_fill_color(&Color::red());
	    circle.set_position(&Vector2f::new(pos.x,pos.y));    
	    sys.window.draw(&circle);	  
	}

}
impl fmt::Show for Trader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "id: {} \n money:{} \n res:{} \n moving avs:{}",
        	 self.id,self.money,self.resources,self.movingPriceAvg)
    }
}

/*
impl fmt::Show for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "id: {} \n money:{} \n res:{} \n delta:{} \n prices:{}",
        	 self.id,self.money,self.resources,self.prates,self.prices)
    }
}*/


impl Selectable for Trader{
	fn get_pos(&self)->geom::V2f{
		self.pos
	}
}
impl Selectable for World{
	fn get_pos(&self)->geom::V2f{
		self.pos
	}
}
struct Journey{
	disTraveled:f32,
	traderid:TraderID
}
struct OutLane{
	dest:WorldID,
	dis:f32,
	journeys:Vec<Journey>	
}


struct ResArr<T>{
	a:[T,..4]
}
impl<T> ResArr<T>{
	fn new(a:[T,..4])->ResArr<T>{
		ResArr{a:a}
	}
}
impl<T> Index<uint,T> for ResArr<T> {
    fn index<'a>(&'a self, rhs: &uint) -> &'a T {
    	&self.a[*rhs]       	
    }
}
impl<T> IndexMut<uint,T> for ResArr<T> {
    fn index_mut<'a>(&'a mut self, _rhs: &uint) -> &'a mut T {
        //println!("Indexing!");
        &mut self.a[*_rhs]
    }
}
impl<T:fmt::Show> fmt::Show for ResArr<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let s=self;
        write!(f, "{},{},{},{}",s[0],s[1],s[2],s[3])
        //write!(f, "{},",s[0])
    }
}


struct World{
	id:WorldID,
	pos:geom::V2f,
	connections:Vec<OutLane>,
	resources:ResArr<int>,
	prates:ResArr<int>,
	prices:ResArr<int>,
	money:int
	
}
impl World{	
	fn draw(&self,sys:&mut MSys){
		let mut circle = match CircleShape::new() {
	        Some(circle) => circle,
	        None       => fail!("Error, cannot create ball")
	    };
	    circle.set_radius(6.0);	    
	    circle.set_origin(&Vector2f::new(6.0,6.0));
	    circle.set_fill_color(&Color::blue());
	    circle.set_position(&Vector2f::new(self.pos.x,self.pos.y));    
	    sys.window.draw(&circle);	  
	}
}

impl fmt::Show for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "id: {} \n money:{} \n res:{} \n delta:{} \n prices:{}",
        	 self.id,self.money,self.resources,self.prates,self.prices)
    }
}

struct TraderManager{
	traders:Vec<Trader>
}
impl TraderManager{
	fn create()->TraderManager{
		TraderManager{traders:Vec::new()}
	}
	fn get_trader<'a>(&'a self,t:TraderID)->&'a Trader{
		&self.traders[t.id as uint]
	}
	fn get_mut_trader<'a>(&'a mut self,t:TraderID)->&'a mut Trader{
		self.traders.get_mut(t.id as uint)
	}
	fn create_trader(&mut self,pos:geom::V2f)->TraderID{
		let n=self.traders.len() as int;
		let traderid=TraderID::new(n);
		self.traders.push(Trader{
			id:traderid,
			money:100,
			resources:ResArr::new([10000,..4]),
			movingPriceAvg:ResArr::new([1000.0,..4]),
			pos:geom::V2f::new(0.0,0.0)});		
		return traderid;
	}
}
struct WorldManager{
	worlds:Vec<World>
}

impl WorldManager{
	
	fn get_world<'a>(&'a self,t:WorldID)->&'a World{
		&self.worlds[t.id as uint]
	}
	fn get_mut_world<'a>(&'a mut self,t:WorldID)->&'a mut World{
		self.worlds.get_mut(t.id as uint)
	}
	fn create_world(&mut self,pos:geom::V2f)->int{
		let n=self.worlds.len() as int;		

		let mut prates=[0,..4];
		let mut rng = task_rng();		
		for n in range(0u,4){	
			
			let m: int = rng.gen_range(-5,5 );
			
			prates[n]=m;
		}
		self.worlds.push(World{id:WorldID::new(n),pos:pos,connections:Vec::new(),resources:ResArr::new([100,..4]),
					prates:ResArr::new(prates),
					prices:ResArr::new([0,..4]),
					money:1000
					});
		return n;
	}
	fn set_pos(&mut self,worldid:WorldID,pos:geom::V2f){
		
		//first set pos		
		self.get_mut_world(worldid).pos=pos;
		let p1=self.get_world(worldid).pos;


		//update the outlanes from worldid
		let mut toDo:Vec<f32>=Vec::new();								
		for n in self.get_world(worldid).connections.iter(){
			let p2=self.get_world(n.dest).pos;
			let dis=(p2-p1).len_sqr().sqrt();
			toDo.push( dis );
		}				
		let mut i=0u;
		for n in self.get_mut_world(worldid).connections.mut_iter(){
			n.dis=toDo[i];
			i+=1;
		}
		
		//update the outlanes pointing to worldid		
		let mut toDo:Vec<(WorldID,f32)>=Vec::new();
		for n in self.worlds.iter(){			
			for j in n.connections.iter(){
				if j.dest.equals(worldid) {
					let p2=n.pos;//self.get_world(j.dest).pos;
					let dis=(p2-p1).len_sqr().sqrt();					
					toDo.push(  (n.id,dis) );
				}
			}			
		}
		for &(w1,dis) in toDo.iter(){
			let n=self.get_mut_world(w1);
			for j in n.connections.mut_iter(){
				if j.dest.equals(worldid) {					
					j.dis=dis;
					break;
				}
			}
		}		
	}
	
	fn create()->WorldManager{
		WorldManager{worlds:Vec::new()}
	}

	
	
	
	fn connect_nearby(&mut self,condis:f32){
	
		for n in range(0,self.worlds.len()){
			for m in range(n+1,self.worlds.len()){
				let id1;
				let id2;
				let dis;
				{
					let ref b1=self.worlds[n];
					let ref b2=self.worlds[m];
					let p1=b1.pos;
					let p2=b2.pos;
					let disSqr=(p2-p1).len_sqr();
					dis=disSqr.sqrt();
					id1=b1.id;
					id2=b2.id;
				}
				
				if dis<condis {
					self.connect(id1,id2,dis);
				}
			}
		}	
		
	}
	fn connect(&mut self,id1:WorldID,id2:WorldID,dis:f32){
		{
			let w1=self.get_mut_world(id1);
			w1.connections.push(OutLane{dest:id2,dis:dis,journeys:Vec::new()});
		}
		{
			let w2=self.get_mut_world(id2);
			w2.connections.push(OutLane{dest:id1,dis:dis,journeys:Vec::new()});
		}
		
	}
	
}

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


enum ENUM{
	WorldW,
	TraderW,
	Nil
}
struct SelectThing{
	wid:WorldID,
	tid:TraderID,
	which:ENUM
}
impl SelectThing{
	fn create()->SelectThing{
		SelectThing{wid:WorldID::empty(),tid:TraderID::empty(),which:Nil}
	}
}
pub struct GameInstance{	
	worldman:WorldManager,
	traderman:TraderManager,
	selected:WorldID,
	counter:int,
	selectedThing:SelectThing
}
impl GameInstance{
	
	pub fn create()->GameInstance{

		


		let mut g=GameInstance{worldman:WorldManager::create(),
					traderman:TraderManager::create(),
					selected:WorldID::empty(),
					counter:0,
					selectedThing:SelectThing::create()
					};
		//g.selectThing=TraderID::new(5);
		
		//test:Trader{pos:geom::V2f{x:15.,y:15.}},
		/*
		let mut g=GameInstance{drawSys:DrawSys::create()};
		{
		let k=g.drawSys.addDrawThing(DrawThing::create(0,0,255));
		let p1=sdl2::rect::Point{x:0,y:1};
        let p2=sdl2::rect::Point{x:400,y:5};	
        k.add_line(p1,p2);
    	}
		*/	
		
		for n in range(0u,32){
			let pos=geom::Rect::new_int(0,0,512,512)	.random_pos();
			let _=g.worldman.create_world(pos);
		}	

		g.worldman.connect_nearby(100.);
		

		for n in range(0u,32){
			let wid=WorldID::new(0);
			let traderid=g.traderman.create_trader(g.worldman.get_world(wid).pos);
			g.set_trader_on_random_journey(traderid  ,wid);
		}


		return g;

	}		
	pub fn step(&mut self){
		//println!("worldmanager step. num traders={}",self.traders.len());

		let mut ntoSend:Vec<(TraderID,WorldID,WorldID)>=Vec::new();		
		for w1 in self.worldman.worlds.mut_iter(){

			//update resources
			if self.counter>60{
				for n in range(0u,4){
					let k=w1.resources[n]+w1.prates[n];
					if k>=0 {
						w1.resources[n]=k;
					}					
				}
				//calculate prices
				for n in range(0u,4){
					w1.prices[n]=geom::max(0,1000-w1.resources[n]);
				}

				self.counter=0;
			}
			self.counter+=1;

			

			for cont in w1.connections.mut_iter(){								


				//check which traders reached their destrination
				let mut ntoRemove:Vec<(TraderID,WorldID,WorldID)>=Vec::new();
				for jj in range(0,cont.journeys.len()){				
					let ref journey=cont.journeys[jj];
					//println!("checking: disTraveled={} real dis={}",journey.disTraveled,cont.dis);						
					if journey.disTraveled>=cont.dis{
						//toRemove=jj
						let bla=(journey.traderid,w1.id,cont.dest);
						ntoRemove.push(   bla );
						ntoSend.push(bla);

					}					
				}

				//actually remove the finished journeys
				for &(n,_,m) in ntoRemove.iter(){			
					for ii in range(0,cont.journeys.len()){
						let nid=cont.journeys[ii].traderid;
						if n.equals(nid) {
							match cont.journeys.swap_remove(ii){
								Some(journey)=>{
									//println!("success remove")																
								},
								None=>fail!("fail remove")
							}
							break;
						}
					}					
				}					
				//for remaining traders update distance traveled.
				for journey in cont.journeys.mut_iter() {					
					journey.disTraveled+=0.5;		
				}
			}
		}

		let mut bla:Vec<(TraderID,geom::V2f)>=Vec::new();
		for n in self.worldman.worlds.iter(){
			let p1=n.pos;
			for j in n.connections.iter(){
				let p2=self.worldman.get_world(j.dest).pos;
				let offset=p2-p1;
			    let norm=offset/(offset.len_sqr()).sqrt();
			    for ja in j.journeys.iter(){
			    	let pp=p1+norm*ja.disTraveled;
			    	bla.push( (ja.traderid,pp) );
			    }
			}
		}
		for &(tid,pos) in bla.iter(){
			self.traderman.get_mut_trader(tid).pos=pos;
		}		


		//for all the traders who know are not on a journey, set a new journey
		for &(n,wid,m) in ntoSend.iter(){						
			self.trade_with_trader(wid,n);
			self.set_trader_on_random_journey(n,m);
		}		
	}
	
	fn trade_with_trader(&mut self,world:WorldID,traderid:TraderID){
		let mut tr=self.traderman.get_mut_trader(traderid);
		let mut w=self.worldman.get_mut_world(world);
		for n in range(0u,4){
			let f:f32=0.7;
			tr.movingPriceAvg[n]=tr.movingPriceAvg[n]*f+(w.prices[n] as f32)*(1.0-f);

			if tr.movingPriceAvg[n] as int<=w.prices[n]{
				//trader sells to world!
				if w.money>=10&&tr.resources[n]>=40{
					//println!("trade about to occur!: \n {} \n to world:{}",tr,w);
					println!("trader {} sold resource {} to world {}",tr.id,n,w.id);
					tr.money+=10;
					w.money-=10;
					tr.resources[n]-=40;
					w.resources[n]+=40;
					//println!("trade finish!: {} to world:{}",tr,w);
				}				
			}else{
				//buy!
				if tr.money>=10&&w.resources[n]>=40 {
					println!("trader {} bought resource {} from world {}",tr.id,n,w.id);
					//println!("trader:{} bought resource:{} from world:{}",traderid,n,world);
					tr.money-=10;
					w.money+=10;
					tr.resources[n]+=40;
					w.resources[n]-=40;
				}
			
			}
		}
		
	}
	fn set_trader_on_random_journey(&mut self,traderid:TraderID,worldAt:WorldID){
		let world=self.worldman.get_mut_world(worldAt);		
		if world.connections.len()==0{
			println!("no edges to connect to!");
			return;
		}
		let ref trader=self.traderman.get_trader(traderid);

		let mut rng = task_rng();	
		
		let m: int = rng.gen_range(0,world.connections.len() as int );
		
		let con=world.connections.get_mut(m as uint);
		con.journeys.push(Journey{disTraveled:0.0,traderid:traderid});
	}
	pub fn draw(&self,msys:&mut MSys){
		

		//draw lanes
		for w1 in self.worldman.worlds.iter(){
			for cont in w1.connections.iter(){
				let ref w2=self.worldman.get_world(cont.dest);
				let p1=w2.pos;
				//let p2=w2.pos;
				let mut rect = match ConvexShape::new(3) {
			        Some(rect) => rect,
			        None       => fail!("Error, cannot create ball")
			    };			    
			    let off=((w2.pos-w1.pos)/cont.dis).rot90deg();
			    let p2=w1.pos+off*-3.0;
			    let p3=w1.pos+off*3.0;
			    rect.set_point(0,&Vector2f::new(p1.x,p1.y));
			    rect.set_point(1,&Vector2f::new(p2.x,p2.y));
			    rect.set_point(2,&Vector2f::new(p3.x,p3.y));
			    rect.set_fill_color(&Color::new_RGBA(20,20,20,30));
			    msys.window.draw(&rect);
			}
		}

		//draw worlds
		for w1 in self.worldman.worlds.iter(){
			w1.draw(msys);
		}

		//draw bots
		for w1 in self.worldman.worlds.iter(){
			for cont in w1.connections.iter(){
			    for journey in cont.journeys.iter(){			    	
			    	Trader::draw(self.traderman.get_trader(journey.traderid).pos,msys);
			    }				
			}									
		}
		/*
		for w1 in self.worlds.iter(){
			for cont in w1.connections.iter(){
			    for journey in cont.journeys.iter(){
			    	let p1=w1.pos;
			    	let p2=self.worlds[cont.dest as uint].pos;
			    	let offset=p2-p1;
			    	let norm=offset/(offset.len_sqr()).sqrt();

			    	let ap2=p1+norm*journey.disTraveled;

			    	let k=self.traders[journey.traderid as uint];
			    	let f=k.resources;
			    	let z=format!("{}:{},{},{},{}",k.money,f[0],f[1],f[2],f[3]);
			    	msys.draw_text(z.as_slice(),&ap2)
			    }
			}
		}*/
		/*
		//draw text
		for w1 in self.worlds.iter(){
			//let k=w1.resources;
			//let z=format!("{}:{},{},{},{}  \n  {}",w1.money,k[0],k[1],k[2],k[3],w1);
			//msys.draw_text(z.as_slice(),&w1.pos);
			let z=format!("{}",w1);
			msys.draw_text(z.as_slice(),&w1.pos);
		}*/
		let pp=geom::V2f::new(10.0,10.0);
		match self.selectedThing.which{
			WorldW=> {
				let w=self.worldman.get_world(self.selectedThing.wid);
				let z=format!("{}",w);
				msys.draw_text(z.as_slice(),&pp);
			},
			TraderW=> {
				let t=self.traderman.get_trader(self.selectedThing.tid);
				//let z=format!("{}:{},{},{},{}",t.money,f[0],f[1],f[2],f[3]);
				let z=format!("{}",t);
			    msys.draw_text(z.as_slice(),&pp)

			},
			Nil=> {}
		}
	}


	fn closestWorld(&self,pos:geom::V2f,maxDis:f32)->(WorldID,f32){		
		let mut closest:WorldID=WorldID::empty();
		let mut closestDis:f32=maxDis;

		for w in self.worldman.worlds.iter(){
			let dis=(w.pos-pos).len_sqr().sqrt();
			if(dis<closestDis){
				closest=w.id;
				closestDis=dis;
			}
		}
		(closest,closestDis)			
	}
	fn closestTrader(&self,pos:geom::V2f,maxDis:f32)->(TraderID,f32){		
		let mut closest:TraderID=TraderID::empty();
		let mut closestDis:f32=maxDis;

		for w in self.traderman.traders.iter(){
			let dis=(w.pos-pos).len_sqr().sqrt();
			if(dis<closestDis){
				closest=w.id;
				closestDis=dis;
			}
		}
		(closest,closestDis)				
	}

}

impl MMouseEventListener for GameInstance{
	fn on_mouse_down(&mut self,pos:geom::V2f){
		
		if self.selected.equals(WorldID::empty()) {

			//find closest World
			let (wid,dis)= self.closestWorld(pos,50.0);

			//find closest Trader
			let (tid,dis2)=self.closestTrader(pos,50.0);

			if wid.equals(WorldID::empty()) && tid.equals(TraderID::empty()){
				self.selectedThing.which=Nil;
			}else{
				//if dis<dis2 {
					self.selectedThing.wid=wid;
					self.selectedThing.which=WorldW;
					/*
				}else{
					self.selectedThing.tid=tid;
					self.selectedThing.which=TraderW
				}*/
			}			
		} else{
			//let mut w=self.worldman.worlds.get_mut(self.selected as uint);
			//w.pos=pos;
			self.worldman.set_pos(self.selected,pos);
			self.selected=WorldID::empty();
		}

		//self.worldman.create_world(pos);
		println!("game mouse down!");
	}
    fn on_mouse_up(&mut self, pos:geom::V2f){
    	println!("game mouse up!");
    }

}