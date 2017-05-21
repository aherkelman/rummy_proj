use sprite::*;
use piston_window::*;
use graphics::types::SourceRectangle;
use std::rc::Rc;
use gfx_device_gl;
use std;
use find_folder;
//rummy specific drawing

use rummy::tools::PlayingCard;
use rummy::tools::Suit;



pub struct CardSpriteManager{
	tex:Rc<Texture<gfx_device_gl::Resources>>
}

impl CardSpriteManager{
	
	pub fn new(window:&mut PistonWindow)->CardSpriteManager{
		let assets = find_folder::Search::ParentsThenKids(3, 3)
	        .for_folder("Images").unwrap();
	    //let id;
	    //let mut scene = Scene::new();
	    let tex = Rc::new(Texture::from_path(
	            &mut window.factory,
	            assets.join("cards.png"),
	            Flip::None,
	            &TextureSettings::new()
	        ).unwrap());

	    CardSpriteManager{tex}
	}

	pub fn new_card(&self,suite:usize,number:usize)->Cardsprite{
		let kk=number as f64;
        let ii=suite as f64;
        let k:SourceRectangle=[card_width*kk,card_height*ii,card_width,card_height];
        let mut sp=Sprite::from_texture_rect(self.tex.clone(),k);
        sp.set_position(card_width*kk*1.5,card_height*ii*1.5);
        Cardsprite{sprite:sp}   
	}
}


pub struct Cardsprite{
	sprite:Sprite<Texture<gfx_device_gl::Resources>>
}

const card_width:f64=73.0;
const card_height:f64=98.0;



impl std::fmt::Debug for Cardsprite {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    	let (x,y)=self.sprite.get_position();
        write!(f, "Sprite {{ x: {}, y: {} }}", x,y)
    }
}