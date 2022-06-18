use ggez;
//use ggez::{Context, GameResult};
use ggez::event::EventHandler;
use ggez::graphics;
use ggez::input::keyboard::{self, KeyCode};

//#![allow(non_snake_case)]
//#![allow(non_camel_case_types)]
const SCREENW: f32 = 900.0;
const SCREENH: f32 = 600.0;

//ggez::conf::WindowMode::dimensions(400,800);

struct rectangle_data {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl rectangle_data {
    pub fn getx(&self) -> f32 {
        self.x
    }
    pub fn gety(&self) -> f32 {
        self.y
    }
    pub fn getw(&self) -> f32 {
        self.w
    }
    pub fn geth(&self) -> f32 {
        self.h
    }
}

fn make_rect(X: f32, Y: f32) -> rectangle_data {
    let x = rectangle_data {
        x: X,
        y: Y,
        w: 10.0,
        h: 10.0,
    };
    x
}

struct MainState {
    //put stuff here
    Cube: [[f32; 3]; 8],
}

fn matrix_multiplier(multiplier: [[f32; 3]; 3], cordinate: [f32; 3]) -> [f32; 3] {
    //this works
    let mut return_array = [0.0, 0.0, 0.0];
    let mut iterator = 0;
    for mrow in multiplier {
        for i in 0..3 {
            return_array[iterator] += mrow[i] * cordinate[i];
        }
        iterator += 1;
    }
    return_array
}

impl MainState {
    pub fn new(_ctx: &mut ggez::Context) -> Self {
        MainState {
            Cube: [
                [-50.0, -50.0, -50.0],
                [50.0, -50.0, -50.0],
                [50.0, 50.0, -50.0],
                [-50.0, 50.0, -50.0],
                [-50.0, -50.0, 50.0],
                [50.0, -50.0, 50.0],
                [50.0, 50.0, 50.0],
                [-50.0, 50.0, 50.0],
            ],
        }
    }

    pub fn rotate_y(&mut self, angle: f32) {
        let RotateYMatrix: [[f32; 3]; 3] = [
            [angle.cos(), 0.0, angle.sin()],
            [0.0, 1.0, 0.0],
            [-angle.sin(), 0.0, angle.cos()],
        ];

        for i in 0..self.Cube.len() {
            let result: [f32; 3] = matrix_multiplier(RotateYMatrix, self.Cube[i]);
            self.Cube[i] = result;
        }
    }

    pub fn rotate_z(&mut self, angle: f32) {
        let RotateZMatrix: [[f32; 3]; 3] = [
            [angle.cos(), -angle.sin(), 0.0],
            [angle.sin(), angle.cos(), 0.0],
            [0.0, 0.0, 1.0],
        ];
        for i in 0..self.Cube.len() {
            let result: [f32; 3] = matrix_multiplier(RotateZMatrix, self.Cube[i]);
            self.Cube[i] = result;
        }
    }

    pub fn rotate_x(&mut self, angle: f32) {
        //math is placeholder for angle
        let _iterator: usize = 0;
        let RotateXMatrix: [[f32; 3]; 3] = [
            [1.0, 0.0, 0.0],
            [0.0, angle.cos(), -angle.sin()],
            [0.0, angle.sin(), angle.cos()],
        ];

        for i in 0..self.Cube.len() {
            let result: [f32; 3] = matrix_multiplier(RotateXMatrix, self.Cube[i]);
            self.Cube[i] = result;
        }
    }

    pub fn make_pointlist(&mut self, i: usize, a: usize) -> [[f32; 2]; 2] {
        let points: [[f32; 2]; 2] = [
            [
                //magick number is to prevent building mesh with <3 vertex
                self.Cube[i][0] + SCREENW / 2.0+1.0,
                self.Cube[i][1] + SCREENH / 2.0,
            ],
            [
                self.Cube[a][0] + SCREENW / 2.0,
                self.Cube[a][1] + SCREENH / 2.0+1.0,
            ],
        ];
        points
    }

    /*
        pub fn draw_lines(&mut self,  ctx:&mut ggez::Context){




            for i in 0..self.Cube.len(){
                let  a=i+1;
                if i+1==4{
                    a=0;}
                let points :[[f32;2];2]  =[[self.Cube[i][0],self.Cube[i][1]  ], [self.Cube[a][0], self.Cube[a][1] ] ] ;
               let mesh = graphics::Mesh::new_line(ctx,&points,1.0,graphics::Color::WHITE);
                //graphics::draw(ctx,&mesh,graphics::DrawParam::default())?;

                graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
            }
    /*
            for i in range(4,len(self.Cube)):{
                let :i32 a=i+1
                if i+1==8{
                    a=4;}
                pygame.draw.line(win,color,self.fcube[i],self.fcube[a])
            }

            for i in range(int(len(self.Cube)/2)):{
                pygame.draw.line(win,color,self.fcube[i],self.fcube[i+4])
            }
        }*/


    }*/
}

//impl graphics::Drawable for Result<graphics::Mesh,ggez::GameError>{}

impl EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            self.rotate_x(0.01); //this is also a valis way to call
        }

        if keyboard::is_key_pressed(ctx, KeyCode::S) {
            MainState::rotate_x(self, -0.01);
        }

        if keyboard::is_key_pressed(ctx, KeyCode::A) {
            MainState::rotate_y(self, -0.01);
        }

        if keyboard::is_key_pressed(ctx, KeyCode::D) {
            MainState::rotate_y(self, 0.01);
        }

        if keyboard::is_key_pressed(ctx, KeyCode::Q) {
            MainState::rotate_z(self, -0.01);
        }

        if keyboard::is_key_pressed(ctx, KeyCode::E) {
            MainState::rotate_z(self, 0.01);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, graphics::Color::WHITE);

        //do it this way but better i quess
        //draw lines
        for i in 0..self.Cube.len() {
            let mut a = i + 1;
            if a == 4 {
                a = 0;
            }
            if a == 8 {
                a = 4;
            }
            //draw linemesh needs array of arrays with x and y
            //+screen/2 so that the line is drawn in the centre of the screen.
            let points = self.make_pointlist(i, a);
            let linemesh = graphics::Mesh::new_line(ctx, &points, 1.0, graphics::Color::BLACK)?;
            graphics::draw(ctx, &linemesh, graphics::DrawParam::default())?;
        }

       // }
        
        //4 is half of the cubes length
        for i in 0..4 {
            let mut a = i + 4;
            let points = self.make_pointlist(i, a);
            let linemesh = graphics::Mesh::new_line(ctx, &points, 1.0, graphics::Color::BLACK)?;
            graphics::draw(ctx, &linemesh, graphics::DrawParam::default())?;
}

/*
        for item in self.Cube {
            let x = make_rect(item[0], item[1]);
            //magic numbers are window width half and heigth half this is to be changed when i
            //fugure iut oit
            let rect = graphics::Rect::new(
                x.getx() + SCREENW / 2.0,
                x.gety() + SCREENH / 2.0,
                x.getw(),
                x.geth(),
            );

            let mesh = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                rect,
                graphics::Color::BLACK,
            )?;
            graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        }
*/
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> ggez::GameResult {
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("test", "kimi")
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREENW, SCREENH))
        .build()
        .expect("context builder failure");

    // let main_state = MainState {};
    let main_state = MainState::new(&mut ctx);

    ggez::event::run(ctx, event_loop, main_state)
}
