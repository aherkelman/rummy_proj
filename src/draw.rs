use sprite::*;
use piston_window::*;
use graphics::types::SourceRectangle;
use std::rc::Rc;
use gfx_device_gl;
use find_folder;

pub struct Drawing
{

    cards:Vec<Sprite<Texture<gfx_device_gl::Resources>>>, // card image
    vals_to_draw:Vec<usize>
    //cardTex:Texture
}

impl Drawing 
{
	pub fn new(window:&mut PistonWindow)->Drawing
	{
		// Get card texture from Images folder
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
	    //Drawing{tex}


	    let mut cards2=Vec::new();
	    let mut vals=Vec::new();
	    let card_dim:[f64;2]=[73.0,98.0];
	    for i in 0..4
	    {
	    	for k in 0..13
	    	{
	    		vals.push((i+1)*k);
	    		let kk=k as f64;
	    		let ii=i as f64;
	    		let k:SourceRectangle=[card_dim[0]*kk,card_dim[1]*ii,card_dim[0],card_dim[1]];
	    		let mut sp=Sprite::from_texture_rect(tex.clone(),k);
	    		sp.set_position(card_dim[0]*kk*1.5,card_dim[1]*ii*1.5);
	    		cards2.push(sp);
	    	}
	    }
	    Drawing{cards:cards2,vals_to_draw:vals}
	}
	pub fn draw(&mut self,c:&Context, g:&mut G2d)
	{
		for i in self.vals_to_draw.iter(){
			self.cards[*i].draw(c.transform,g);
		}
		/*
		for i in self.cards.iter_mut(){
			i.draw(c.transform,g);
		}*/	
	}
	/*
	// Draw only sprites set to be drawn
	pub fn draw_vals(&mut self,c:&Context, g:&mut G2d)
	{

	}*/

	/*
	// Set what sprites should be drawn
	pub fn set_vals(&mut self, nums:Vec<usize>)
	{
         self.vals_to_draw = nums.clone();
	}
	pub fn get_sprite_mut(&mut self,index:usize)->&mut Sprite<Texture<gfx_device_gl::Resources>>{
		&mut self.cards[index]
	}*/
}
