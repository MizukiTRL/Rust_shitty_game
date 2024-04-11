#![allow(unused)]

use crate::objects::{self, Coin};

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Tiles{
    Obstacle,
    Coin,
    Player,
}

#[derive(Debug)]
pub struct Grid{
    board: Vec<Vec<Option<Tiles>>>,
    width: u8,
    height: u8,
}

impl Grid {
    pub fn new(w:u8, h: u8)->Self{
        let mut grid: Vec<Vec<Option<Tiles>>> = vec![];
        for _i in 0..h {        
            let mut row: Vec<Option<Tiles>> = vec![]; 
            for _j in 0..w {
                row.push(None);
            }
            grid.push(row);
        }

        Grid { board: grid, width: w, height: h }
    }

    pub fn print_grid(&self){

        let h = &self.height;
        let w = &self.width;

        let border_h = h+1;
        let border_w = w+1;

        for i in 0..h+2 {
            for j in 0..w+2 {
                match (j, i) {
                    (0,0) => print!("╔"),
                    (0, b) if b == border_h => print!("╚"),
                    (b, 0) if b == border_w => print!("╗"),
                    (a, b) if a == border_w && b == border_h => print!("╝"),
                    (b, _) if b==0 || b==border_w => print!("║"),
                    (_, b) if b==0 || b==border_h => print!("═"),
                    _ => {
                        //println!("{:?}", (j, i));
                        match &self.board[i as usize -1][j as usize -1] {
                        Some(a) => match &a {
                            Tiles::Player => print!("@"),
                            Tiles::Coin => print!("☺"),
                            Tiles::Obstacle => print!("█"),
                        },
                        None => print!(" ")
                        }
                    }
                }
            }
            println!("");
        }
    }

    pub fn update_grid(&mut self, player: &objects::Player, obstacles:&Vec<objects::Obstacle>, coins: &Vec<objects::Coin>){
        
        for rows in &mut self.board{
            for i in 0..self.width as usize{
                rows[i] = None;
            }
        }
        //assign player
        if self.board[player.get_y()][player.get_x()] == None{
            self.board[player.get_y()][player.get_x()] = Some(Tiles::Player);
        }
        //assign obstacles
        for obstacle in obstacles {

            if self.board[obstacle.get_y()][obstacle.get_x()] == None{
                //println!("obstacle: {:?}",(obstacle.get_x(), obstacle.get_y()));
                self.board[obstacle.get_y()][obstacle.get_x()]= Some(Tiles::Obstacle);
            }
        }
        //assign coins

        for coin in coins {
            if self.board[coin.get_y()][coin.get_x()] == None{
                self.board[coin.get_y()][coin.get_x()] = Some(Tiles::Coin);
            }
        }
    }
}