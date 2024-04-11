#![allow(unused)]

use std::{borrow::BorrowMut, fs::File, io::{self, stdout, BufRead, BufReader, Bytes, Stdout, Write}, iter, string, thread::{self, sleep}, time::Duration};

use grid::Grid;
use objects::{Coin, Obstacle, Player};

use crossterm::{cursor::MoveTo, event::{poll, read, Event, KeyCode, KeyEventKind}, execute, queue, terminal::{self, Clear, ClearType}, QueueableCommand};

mod grid;
mod objects;

enum Direction {
    Up,
    Down,
    Left, 
    Right
}

fn load_data(){
    let mut player= Player::new(String::from(""), true, (0, 0));
    let mut obstacles= vec![];
    let mut coins = vec![];
    let mut grid: Grid;

    let mut height = 0;
    let mut width= 0;
    
    let file = File::open("./levels/level.txt").expect("failed to open the file");

    let reader =io::BufReader::new(file);
   

    for (y, lines) in reader.lines().enumerate(){
        let line = lines.expect("error reading the lines");
        println!("{}", line);
        height +=1;
        for (x, char) in line.as_bytes().iter().enumerate(){
            width = x;
            match char {
                b'p' => {
                    print!("py ");
                    player = Player::new(String::from("perro"), true, (x as u8, y as u8));
                    
                }
                b'o' => {
                    print!("obs ");
                    obstacles.push(Obstacle::new((x as u8, y as u8)));
                    
                }
                b'c' => {
                    coins.push(Coin::new((x as u8, y as u8), 10));
                    
                }
                b' ' => {
                }
                _ => {}
            }
        }
    }
    //width = width/height;

    width+= 1;

    println!("{:?}", &player);
    println!("{:?}", &obstacles);
    println!("{:?}", &coins);

    grid = Grid::new(width as u8, height);
    grid.update_grid(&player, &obstacles, &coins);
    grid.print_grid();
    
    println!("width: {}, height {}", width, height);
}

fn game(){

    let mut stdout = stdout();

    load_data();
    

    let mut play_grid = Grid::new(21, 11);
    println!("empty");
    play_grid.print_grid();

    let mut player_1 = Player::new(String::from("german"), true, (1, 1));
    let mut obstacles_1: Vec<Obstacle> = vec![Obstacle::new((10, 9)), Obstacle::new((10, 8)), Obstacle::new((10, 7)), Obstacle::new((9, 7))];
    let mut coins_1: Vec<Coin> = vec![Coin::new((20, 10), 10), Coin::new((2, 5), 10)];

    let mut game_on = true;

    while game_on {

        


        while poll(Duration::from_millis(200)).expect("no sirvio el poll") {
            match read().unwrap() {
                Event::Key(event) => {match (event.code, event.kind) {
                    (KeyCode::Right, KeyEventKind::Press)=> {
                        if !check_collition(&player_1, &obstacles_1, Direction::Right){
                            //println!("error");
                            player_1.move_right();
                        }
                    },
                    (KeyCode::Left, KeyEventKind::Press)=> {
                        if !check_collition(&player_1, &obstacles_1 ,Direction::Left){
                            player_1.move_left();
                        }
                    },

                    (KeyCode::Up, KeyEventKind::Press)=> {
                        if !check_collition(&player_1, &obstacles_1 ,Direction::Up){
                            player_1.move_up();
                        }
                    }
                    (KeyCode::Down, KeyEventKind::Press)=> {
                        if !check_collition(&player_1, &obstacles_1, Direction::Down){
                            player_1.move_down()
                        }
                    }

                    _ => {}
                }},
                _ => {},
            }
        }
        println!("{:?}", (player_1.get_x(), player_1.get_y()));
        check_coins(&player_1, &mut coins_1);
        //clear_screen(&mut stdout);
        play_grid.update_grid(&player_1, &obstacles_1, &coins_1);
        play_grid.print_grid();
        
        sleep(Duration::from_millis(500));
        
    }

}

fn check_coins(player: &Player, coins: &mut Vec<Coin>){
    let mut col = false;
    let mut index = 0;
    for (i ,coin) in coins.iter().enumerate(){
        let p_right_o = player.get_x() < coin.get_x();
        let p_left_o = player.get_x() > coin.get_x();
        let p_under_o = player.get_y() > coin.get_y();
        let p_over_o = player.get_y() < coin.get_y();

        if !(p_left_o || p_over_o || p_right_o || p_under_o){
            println!("colision");
            col = true;
            index = i;
        }
    }

    if col{
        coins.remove(index);
    }
}

fn check_collition(player: &Player, obstacles: &Vec<Obstacle>, dir: Direction)->bool{

    for obstacle in obstacles {
        let col;
        match dir {
            Direction::Down => {
                let p_right_o = player.get_x() < obstacle.get_x();
                let p_left_o = player.get_x() > obstacle.get_x();
                let p_under_o = player.get_y()+1 > obstacle.get_y();
                let p_over_o = player.get_y()+1 < obstacle.get_y();

                col= !(p_left_o || p_over_o || p_right_o || p_under_o);
            }
            Direction::Left => {
                let p_right_o = player.get_x()-1 < obstacle.get_x();
                let p_left_o = player.get_x()-1 > obstacle.get_x();
                let p_under_o = player.get_y() > obstacle.get_y();
                let p_over_o = player.get_y() < obstacle.get_y();

                col= !(p_left_o || p_over_o || p_right_o || p_under_o);
            }
            Direction::Right => {
                let p_right_o = player.get_x()+1 < obstacle.get_x();
                let p_left_o = player.get_x()+1 > obstacle.get_x();
                let p_under_o = player.get_y() > obstacle.get_y();
                let p_over_o = player.get_y() < obstacle.get_y();

                col= !(p_left_o || p_over_o || p_right_o || p_under_o);
            }
            Direction::Up => {
                let p_right_o = player.get_x() < obstacle.get_x();
                let p_left_o = player.get_x() > obstacle.get_x();
                let p_under_o = player.get_y()-1 > obstacle.get_y();
                let p_over_o = player.get_y()-1 < obstacle.get_y();

                col= !(p_left_o || p_over_o || p_right_o || p_under_o);
            }
        }
        
        if col {
            return true;
        }
    }
    false
}

fn clear_screen(stdout: &mut Stdout){
    stdout.queue(Clear(ClearType::All)).unwrap();
    stdout.queue(Clear(ClearType::Purge)).unwrap();
    let (width, height) = terminal::size().unwrap();
    //stdout.queue(MoveTo(width/2, height/2)).unwrap();
    stdout.flush().unwrap();
}

fn main() {
    //to do:
    
    //-game logic
    game();
    //-control input
    //-menu
}
