use crate::custom_error::AocError;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::error::Error;
use nom::{character::complete::newline, combinator::iterator, multi::separated_list1, IResult};
use core::fmt;
use std::collections:: HashSet;
use std::ops::{Index, Add};
use std::ops::IndexMut;
#[derive( PartialEq)]
pub struct TileMap {
    map: Vec<Vec<Tile>>,
}
impl fmt::Debug for TileMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.map.iter().for_each(|line| { line.iter().for_each(|tile| write!(f,"{}",tile).unwrap());
        write!(f,
            "\n").unwrap() },);
        writeln!(f)
    }
}
impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res = match self{
            Type::Empty=>".",
            Type::MirorTopLeft=>"\\",
            Type::MirorTopRight=>"/",
            Type::VerticalSpliter=>"|",
            Type::HorizontalSpliter=>"-",

        }   ;
        write!(f,"{}",res)
    }
}
impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        write!(f,"{}",self.tile_type)
    }
}

impl IndexMut<&Cursor> for TileMap {
    fn index_mut(&mut self, cursor: &Cursor) -> &mut Self::Output {
        &mut self.map[cursor.position.y as usize][cursor.position.x as usize]
    }
}
impl Index<&Cursor> for TileMap {
    type Output = Tile;
    fn index(&self, cursor: &Cursor) -> &Self::Output {
        &self.map[cursor.position.y as usize][cursor.position.x as usize]
    }
}
#[derive(Debug, PartialEq,Clone)]
pub struct Tile {
    tile_type: Type,
    energized_directions: HashSet<Direction>,
}
impl Tile{
    fn is_energized(&self)->bool{
        !self.energized_directions.is_empty()
    }
}
#[derive(Debug,Clone, PartialEq,Hash,Eq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}
#[derive(Debug, PartialEq)]
pub struct Cursor {
    position: Position,
    current_direction: Direction,
}

impl Cursor{
    fn create_new (&self,direction:Direction) -> Self{
            Cursor{position:&self.position+direction.clone(), current_direction:direction}
    }
}
#[derive(Debug, PartialEq,Clone)]
pub struct Position {
    x: isize,
    y: isize,
}
impl Add<Direction> for &Position {
    type Output= Position;
fn add(self, rhs: Direction) -> Self::Output {
        match rhs{
            Direction::Up=> Position{x:self.x,y:self.y-1},
            Direction::Down => Position{x:self.x,y:self.y+1},
            Direction::Right => Position{x:self.x+1,y:self.y},
            Direction::Left => Position{x:self.x-1,y:self.y},
        }
    }
}
#[derive( Debug,PartialEq,Clone)]
pub enum Type {
    Empty,
    MirorTopLeft,
    MirorTopRight,
    VerticalSpliter,
    HorizontalSpliter,
}
fn parse_line(input: &str) -> IResult<&str, Vec<Tile>> {
    let mut it = iterator(
        input,
        alt((
            tag::<&str, &str, Error<&str>>("|"),
            tag("-"),
            tag("\\"),
            tag("/"),
            tag("."),
        )),
    );
    let parsed = it
        .map(|tile_type| Tile {
            tile_type: match tile_type {
                "|" => Type::VerticalSpliter,
                "-" => Type::HorizontalSpliter,
                "\\" => Type::MirorTopLeft,
                "/" => Type::MirorTopRight,
                "." => Type::Empty,
                _ => panic!("the input is wrong there shouldn`t be any other char"),
            },
            energized_directions: HashSet::new(),
        })
        .collect::<Vec<Tile>>();
    let res: IResult<_, _> = it.finish();

    res.map(|(input, _)| (input, parsed))
}
fn parse(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
    let (input,mut  lines) = separated_list1(newline, parse_line)(input)?;
    lines.retain(|line| !line.is_empty());
    Ok((input, lines))
}
pub fn is_outside_of_border(tiles: &TileMap, cursor: &Cursor) -> bool {
    let hor_len = tiles.map[0].len();
    let ver_len = tiles.map.len();
    if hor_len ==0 || ver_len == 0 {
    }
    if cursor.position.x >= hor_len as isize|| cursor.position.x < 0 {
        return true;
    }
    if cursor.position.y >= ver_len as isize  || cursor.position.y < 0 {
        return true;
    }
    false
}

pub fn solve(tiles: &mut TileMap, cursor: Cursor) {
    if is_outside_of_border(tiles, &cursor)  {
        return;
    }
    if tiles[&cursor].energized_directions.contains(&cursor.current_direction){
        return;
    }

    tiles[&cursor].energized_directions.insert(cursor.current_direction.clone());
    match ( &tiles[&cursor].tile_type,&cursor.current_direction ){
        // "\"
        ( Type::MirorTopLeft,Direction::Up ) => solve(tiles,
                cursor.create_new(Direction::Left)),
        (Type::MirorTopLeft,Direction::Down)=> solve(tiles,
                cursor.create_new(Direction::Right)),
        (Type::MirorTopLeft,Direction::Right) =>  solve(tiles,
                cursor.create_new(Direction::Down)),
        (Type::MirorTopLeft,Direction::Left) =>  solve(tiles,
                cursor.create_new(Direction::Up)),

        // "/"
        ( Type::MirorTopRight,Direction::Up ) => solve(tiles,
                cursor.create_new(Direction::Right)),
        (Type::MirorTopRight,Direction::Down)=> solve(tiles,
                cursor.create_new(Direction::Left)),
        (Type::MirorTopRight,Direction::Right) =>  solve(tiles,
                cursor.create_new(Direction::Up)),
        (Type::MirorTopRight,Direction::Left) =>  solve(tiles,
                cursor.create_new(Direction::Down)),

        ( Type::HorizontalSpliter,Direction::Up )|
        (Type::HorizontalSpliter,Direction::Down)=> {
                solve(tiles, cursor.create_new(Direction::Right)); 
                solve(tiles, cursor.create_new(Direction::Left));},
        (Type::HorizontalSpliter,Direction::Right) |
        (Type::HorizontalSpliter,Direction::Left) =>
                solve(tiles,cursor.create_new(cursor.current_direction.clone())),
        ( Type::VerticalSpliter,Direction::Up )|
        (Type::VerticalSpliter,Direction::Down)=> {
                solve(tiles,cursor.create_new(cursor.current_direction.clone()))},
        (Type::VerticalSpliter,Direction::Right) |
        (Type::VerticalSpliter,Direction::Left) =>{
                solve(tiles, cursor.create_new(Direction::Up)); 
                solve(tiles, cursor.create_new(Direction::Down));},
        (Type::Empty,Direction::Up)|
        (Type::Empty,Direction::Down)|
        (Type::Empty,Direction::Right)|
        (Type::Empty,Direction::Left)
                => solve(tiles,cursor.create_new(cursor.current_direction.clone())),
    };
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_input, tiles) = parse(input).expect("the parsing should work");
    let mut cursor_array  :Vec<Cursor> = vec![];
    (0..tiles.len()).for_each(|num| cursor_array.push(Cursor{position:Position{x:0,y:num as isize},current_direction:Direction::Right}));
    (0..tiles.len()).for_each(|num| cursor_array.push(Cursor{position:Position{x:( tiles[0].len()-1 ) as isize,y:num as isize},current_direction:Direction::Left}));
    (0..tiles[0].len()).for_each(|num| cursor_array.push(Cursor{position:Position{x:num as isize,y:0},current_direction:Direction::Down}));
    (0..tiles[0].len()).for_each(|num| cursor_array.push(Cursor{position:Position{x:num as isize,y:( tiles[0].len()-1 ) as isize},current_direction:Direction::Up}));
 let res =    cursor_array.into_iter().map(|cursor| {

    let mut tiles_map = TileMap{map:tiles.iter().map(|line| line.to_vec()).collect::<Vec<_>>()} ;
    solve(&mut tiles_map,cursor);
   let sum = tiles_map.map.iter().map(|line|line.iter().map(|tile| tile.is_energized() as usize).sum::<usize>()).sum::<usize>();
    print!("{:} ",sum);
       sum


    }).max().expect("should be a max");
    // let mut tiles_map = TileMap{map:tiles.iter().map(|line| line.iter().cloned().collect::<Vec<_>>()).collect::<Vec<_>>()} ;
    // solve(&mut tiles_map,Cursor{position:Position{x:0,y:0},current_direction:Direction::Right});
    // let res = tiles_map.map.iter().map(|line|line.iter().map(|tile| tile.is_energized() as usize).sum::<usize>()).sum::<usize>();
    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
        assert_eq!("51".to_string(), process(input)?);
        Ok(())
    }
}
