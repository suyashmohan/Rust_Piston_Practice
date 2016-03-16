extern crate piston_window;

use piston_window::*;

struct Game{
    rotation: f64,
    x : f64,
    y : f64,
    up_key: bool,
    down_key: bool,
    left_key: bool,
    right_key: bool
}

impl Game{
    fn new() -> Game{
        Game {
            rotation : 0.0,
            x: 0.0,
            y: 0.0,
            up_key: false,
            down_key: false,
            left_key: false,
            right_key: false
        }
    }

    fn on_update(&mut self, upd:UpdateArgs){
        if(self.up_key){
            self.y += (-50.0) * upd.dt;
        }
        if(self.down_key){
            self.y += (50.0) * upd.dt;
        }
        if(self.left_key){
            self.x += (-50.0) * upd.dt;
        }
        if(self.right_key){
            self.x += (50.0) * upd.dt;
        }
        self.rotation += 3.0 * upd.dt;
    }

    fn on_render(&mut self, ren:RenderArgs, e:PistonWindow){
        e.draw_2d(|c, g|{
            clear([0.0, 0.0, 0.0, 1.0], g);
            let center = c.transform.trans((ren.width/2) as f64, (ren.height/2) as f64);
            let square = rectangle::square(0.0, 0.0, 100.0);
            let red = [1.0, 0.0, 0.0, 1.0];

            rectangle(red, square, center.trans(self.x, self.y).rot_rad(self.rotation).trans(-50.0, -50.0), g);
        });
    }

    fn on_input(&mut self, inp:Input){
        match inp{
            Input::Press(key) => {
                match key{
                    Button::Keyboard(Key::Up) => {
                        self.up_key = true;
                    }
                    Button::Keyboard(Key::Down) => {
                        self.down_key = true;
                    }
                    Button::Keyboard(Key::Left) => {
                        self.left_key = true;
                    }
                    Button::Keyboard(Key::Right) => {
                        self.right_key = true;
                    }
                    _ => {}
                }
            }
            Input::Release(key) => {
                match key{
                    Button::Keyboard(Key::Up) => {
                        self.up_key = false;
                    }
                    Button::Keyboard(Key::Down) => {
                        self.down_key = false;
                    }
                    Button::Keyboard(Key::Left) => {
                        self.left_key = false;
                    }
                    Button::Keyboard(Key::Right) => {
                        self.right_key = false;
                    }
                    _ => {}
                }
            }
            _ =>{}
        }
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
            Some(Event::Input(inp)) => {
                game.on_input(inp);
            }
            _ =>{
            }
        }
    }
}
