extern crate piston_window;
extern crate ai_behavior;
extern crate sprite;
extern crate find_folder;

extern crate graphics;
extern crate rand;

extern crate gfx_device_gl;

use graphics::types::SourceRectangle;
use std::rc::Rc;


use piston_window::*;
use sprite::*;
use ai_behavior::{
    Action,
    Sequence,
    Wait,
    WaitForever,
    While,
};

mod draw;
mod gencard;
mod rummy;

fn main() {
    /*
    let mut game = rummy::game::Game::new();
    let xpos = 1.2;
    let ypos = 2.5;
    while true{
    game.play_game(xpos,ypos);
    }
    */
    //rummy::game::play_game();

    let (width, height) = (1000, 600);
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow =
        WindowSettings::new("piston: sprite", (width, height))
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let mut draw_stuff=draw::Drawing::new(&mut window);

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("Images").unwrap();
    let id;
    let mut scene = Scene::new();
    let tex = Rc::new(Texture::from_path(
            &mut window.factory,
            assets.join("cards.png"),
            Flip::None,
            &TextureSettings::new()
        ).unwrap());

    let rust_logo = assets.join("background.jpg");
    let rust_logo = Texture::from_path(
            &mut window.factory,
            &rust_logo,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();

    let k:SourceRectangle=[0.0,0.0,75.0,100.0];
    let mut sprite = Sprite::from_texture_rect(tex.clone(),k);
    let k:SourceRectangle=[75.0,0.0,75.0,100.0];
    let mut sprite2 = Sprite::from_texture_rect(tex.clone(),k); 



    sprite.set_position(width as f64 / 2.0, height as f64 / 2.0);
    sprite2.set_position(width as f64 / 2.0-100.0, height as f64 / 2.0-125.0);

    id = scene.add_child(sprite);
    //let id2 = scene.add_child(sprite2);


    // Run a sequence of animations.
    let seq = Sequence(vec![
        Action(Ease(EaseFunction::CubicOut, Box::new(ScaleTo(2.0, 0.5, 0.5)))),
        Action(Ease(EaseFunction::BounceOut, Box::new(MoveBy(1.0, 0.0, 100.0)))),
        Action(Ease(EaseFunction::ElasticOut, Box::new(MoveBy(2.0, 0.0, -100.0)))),
        Action(Ease(EaseFunction::BackInOut, Box::new(MoveBy(1.0, 0.0, -100.0)))),
        Wait(0.5),
        Action(Ease(EaseFunction::ExponentialInOut, Box::new(MoveBy(1.0, 0.0, 100.0)))),
        Action(Blink(1.0, 5)),
        While(Box::new(WaitForever), vec![
            Action(Ease(EaseFunction::QuadraticIn, Box::new(FadeOut(1.0)))),
            Action(Ease(EaseFunction::QuadraticOut, Box::new(FadeIn(1.0)))),
        ]),
    ]);
    scene.run(id, &seq);

    // This animation and the one above can run in parallel.
    let rotate = Action(Ease(EaseFunction::ExponentialInOut,
        Box::new(RotateTo(2.0, 360.0))));
    scene.run(id, &rotate);
    //scene.run(id2,&rotate);
    println!("Press any key to pause/resume the animation!");
    while let Some(e) = window.next() {
        scene.event(&e);

        window.draw_2d(&e, |c, g| {
            clear([1.0, 1.0, 1.0, 1.0], g);
            image(&rust_logo, c.transform, g);
            scene.draw(c.transform, g);
            draw_stuff.draw(&c,g);
            sprite2.draw(c.transform,g);
        });
        if let Some(_) = e.press_args() {
            scene.toggle(id, &seq);
            scene.toggle(id, &rotate);
        }
        if let Some(button) = e.press_args() {
            println!("button: {:?}",button);
            //use piston_window::Key;
            //if button == Keyboard(Key::G)
            //if let Some(Button::Keyboard(Key::A)) = e.press_args()
        }

        // if let Some(button) = e.press_args() {
        //     if button == Button::Mouse(MouseButton::Left) {
        //         draw = true;
        // }
        if let Some(pos) = e.mouse_cursor_args() {
            println!("{:?}",pos);
            let (x, y) = (pos[0] as f64, pos[1] as f64);
            sprite2.set_position(x, y);
        }
    }
}
 