extern crate piston_window;

use piston_window::*;

struct Game{
    rotation: f64
}

impl Game{
    fn new() -> Game{
        Game { rotation : 0.0 }
    }

    fn on_update(&mut self, upd:UpdateArgs){
        self.rotation += 3.0 * upd.dt;
    }

    fn on_render(&mut self, ren:RenderArgs, e:PistonWindow){
        e.draw_2d(|c, g|{
            clear([0.0, 0.0, 0.0, 1.0], g);
            let center = c.transform.trans((ren.width/2) as f64, (ren.height/2) as f64);
            let square = rectangle::square(0.0, 0.0, 100.0);
            let red = [1.0, 0.0, 0.0, 1.0];

            rectangle(red, square, center.rot_rad(self.rotation).trans(-50.0, -50.0), g);
        });
    }
}

fn main() {
    let window : PistonWindow = WindowSettings::new(
        "Piston test",
        [800, 480]
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut game = Game::new();

    for e in window{
        match e.event {
            Some(Event::Update(upd)) => {
                game.on_update(upd);
            }
            Some(Event::Render(ren)) => {
                game.on_render(ren, e);
            }
            _ =>{
            }
        }
    }
}
