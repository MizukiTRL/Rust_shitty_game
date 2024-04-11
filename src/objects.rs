#![allow(unused)]

#[derive(Debug)]
pub struct Player{
    name: String,
    state: bool,
    position: (u8, u8),
}
impl Player{
    pub fn new(n:String, s:bool, p:(u8, u8))-> Self{
        Player{name:n, state:s, position:p}
    }

    pub fn get_x(&self) -> usize{
        self.position.0 as usize
    }

    pub fn get_y(&self) -> usize{
        self.position.1 as usize
    }

    pub fn move_up(&mut self){
        self.position.1 -= 1;
    }
    pub fn move_down(&mut self){
        self.position.1 +=1;
    }
    pub fn move_left(&mut self){
        self.position.0 -= 1
    }
    pub fn move_right(&mut self){
        self.position.0 += 1
    }
    
}

#[derive(Debug)]
pub struct Obstacle{
    position: (u8, u8),
}

impl Obstacle {
    pub fn new(p:(u8, u8))-> Self{
        Obstacle{position: p}
    }

    pub fn get_x(&self) -> usize{
        self.position.0 as usize
    }

    pub fn get_y(&self) -> usize{
        self.position.1 as usize
    }
}

#[derive(Debug)]
pub struct Coin{
    position: (u8, u8),
    points: i32,
}

impl Coin {
    pub fn new(pos: (u8, u8), p: i32)->Self{
        Coin{position: pos, points: p}
    }

    pub fn get_x(&self) -> usize{
        self.position.0 as usize
    }

    pub fn get_y(&self) -> usize{
        self.position.1 as usize
    }
}