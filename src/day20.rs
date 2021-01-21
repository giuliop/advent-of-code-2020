use std::collections::{HashMap, HashSet};
use std::fs;
use Direction::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Side(usize);

impl Side {
    fn from_str(s: &str) -> Self {
        //assert!(s.len() == 10);
        Side(s.chars().enumerate().fold(0, |acc, (i, c)| match c {
            '#' => acc + (1 << i),
            '.' => acc,
            _ => unreachable!(),
        }))
    }

    fn flip(&self) -> Self {
        let mut res = 0;
        let mut n = self.0;
        for _ in 0..10 {
            res <<= 1;
            if n & 1 == 1 {
                res ^= 1
            }
            n >>= 1;
        }
        Side(res)
    }

    fn _to_chars(&self) -> Vec<char> {
        let mut n = self.0;
        let mut res = Vec::new();
        for _ in 0..10 {
            res.push(if n & 1 == 1 { '#' } else { '.' });
            n >>= 1;
        }
        res
    }

    fn _to_string(&self) -> String {
        self._to_chars().iter().collect()
    }
}

type TileSides = [Side; 4];

#[derive(Debug)]
struct Tile {
    id: usize,
    all_sides: HashSet<Side>,
    configurations: [TileSides; 4],
    configurations_flipped: [TileSides; 4],
    image: String,
}

impl Tile {
    fn from_str(s: &str) -> (usize, Self) {
        let id: usize = s[5..9].parse().unwrap();
        let s1 = &s[11..21];
        let s3 = &s[110..120];
        let mut s2 = String::new();
        let mut s4 = String::new();
        for line in s[11..].lines() {
            let mut line = line.chars();
            s4.push(line.nth(0).unwrap());
            s2.push(line.nth(8).unwrap());
        }
        let side_1 = Side::from_str(&s1);
        let side_3 = Side::from_str(&s3);
        let side_2 = Side::from_str(&s2);
        let side_4 = Side::from_str(&s4);
        let configurations = configurations(side_1, side_2, side_3, side_4);
        let configurations_flipped =
            flip_configurations(side_1, side_2, side_3, side_4);
        (
            id,
            Tile {
                id,
                all_sides: vec![
                    side_1,
                    side_1.flip(),
                    side_2,
                    side_2.flip(),
                    side_3,
                    side_3.flip(),
                    side_4,
                    side_4.flip(),
                ]
                .iter()
                .copied()
                .collect(),
                configurations,
                configurations_flipped,
                image: s[11..].to_string(),
            },
        )
    }
}

fn fit<'a>(
    dir: Direction,
    side: Side,
    tiles: &'a HashMap<usize, Tile>,
    tiles_left: &HashSet<usize>,
) -> TileAndOrientation<'a> {
    for id in tiles_left {
        let tile = tiles.get(id).unwrap();
        for base in vec![tile.configurations, tile.configurations_flipped] {
            for conf in 0..4 {
                if side
                    == match dir {
                        Up => base[conf][0],
                        Right => base[conf][1],
                        Down => base[conf][2],
                        Left => base[conf][3],
                    }
                {
                    return TileAndOrientation {
                        tile,
                        flipped: if base == tile.configurations {
                            false
                        } else {
                            true
                        },
                        rotations: conf,
                    };
                }
            }
        }
    }
    unreachable!()
}

fn turn_right(t: TileSides) -> TileSides {
    [t[3].flip(), t[0], t[1].flip(), t[2]]
}

fn configurations(b1: Side, b2: Side, b3: Side, b4: Side) -> [TileSides; 4] {
    let t1 = [b1, b2, b3, b4];
    let t2 = turn_right(t1);
    let t3 = turn_right(t2);
    let t4 = turn_right(t3);
    [t1, t2, t3, t4]
}

fn flip_configurations(b1: Side, b2: Side, b3: Side, b4: Side) -> [TileSides; 4] {
    let t1 = [b1.flip(), b4, b3.flip(), b2];
    let t2 = turn_right(t1);
    let t3 = turn_right(t2);
    let t4 = turn_right(t3);
    [t1, t2, t3, t4]
}

fn read_tiles(f: &str) -> HashMap<usize, Tile> {
    fs::read_to_string(f)
        .expect("error reading file")
        .split("\n\n")
        .map(Tile::from_str)
        .collect()
}

#[derive(Debug)]
struct MatchingTiles<'a> {
    count: usize,
    tiles: Vec<&'a Tile>,
}

fn matching_sides(tiles: &HashMap<usize, Tile>) -> HashMap<&Side, MatchingTiles> {
    let mut matches = HashMap::new();
    for t in tiles.values() {
        for s in &t.all_sides {
            matches
                .entry(s)
                .and_modify(|e: &mut MatchingTiles| {
                    //if e.tiles.iter().all(|x| x.id != t.id) {
                    e.count += 1;
                    e.tiles.push(&t)
                    //}
                })
                .or_insert(MatchingTiles {
                    count: 1,
                    tiles: vec![&t],
                });
        }
    }
    matches
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct TileAndOrientation<'a> {
    tile: &'a Tile,
    flipped: bool,
    rotations: usize,
}

impl<'a> TileAndOrientation<'a> {
    fn side(&self, dir: Direction) -> Side {
        let conf = if self.flipped {
            self.tile.configurations_flipped
        } else {
            self.tile.configurations
        };
        let i = match dir {
            Up => 0,
            Right => 1,
            Down => 2,
            Left => 3,
        };
        conf[self.rotations][i]
    }

    fn aligned_image(&self) -> Image {
        let mut img = self
            .tile
            .image
            .lines()
            .map(|x| x.chars().collect())
            .collect();
        if self.flipped {
            img = flip_image(&img);
        }
        for _ in 0..self.rotations {
            img = rotate_image(&img)
        }

        img
    }

    fn _draw(&self) {
        let conf = if !self.flipped {
            self.tile.configurations
        } else {
            self.tile.configurations_flipped
        };
        let sides: Vec<Vec<char>> =
            conf[self.rotations].iter().map(|x| x._to_chars()).collect();
        println!("{}", sides[0].iter().collect::<String>());
        for i in 1..sides[1].len() - 1 {
            println!("{}        {}", sides[1][i], sides[3][i]);
        }
        println!("{}", sides[2].iter().collect::<String>());
    }
}

type Image = Vec<Vec<char>>;

fn rotate_image(base: &Image) -> Image {
    let rows = base.len();
    let cols = base[0].len();
    let mut rotated: Vec<Vec<char>> = Vec::new();
    for row in 0..cols {
        rotated.push(Vec::new());
        for col in 0..rows {
            rotated[row].push(base[rows - 1 - col][row])
        }
    }
    rotated
}

fn flip_image(base: &Image) -> Image {
    base.iter()
        .map(|x| x.iter().rev().copied().collect())
        .collect()
}

fn pick_corner<'a>(
    tiles: &'a HashMap<usize, Tile>,
    matches: &HashMap<&Side, MatchingTiles>,
) -> TileAndOrientation<'a> {
    let tile = tiles
        .values()
        .filter(|t| {
            t.configurations[0]
                .iter()
                .filter(|side| matches.get(side).unwrap().count == 1)
                .count()
                == 2
        })
        .next()
        .unwrap();
    let mut rotations = 0;
    for i in 0..4 {
        if matches.get(&tile.configurations[i][0]).unwrap().count == 1
            && matches.get(&tile.configurations[i][3]).unwrap().count == 1
        {
            rotations = i;
            break;
        }
    }
    TileAndOrientation {
        tile,
        flipped: false,
        rotations,
    }
}

fn pick_next<'a>(
    row: usize,
    col: usize,
    image: &Vec<Vec<TileAndOrientation>>,
    tiles: &'a HashMap<usize, Tile>,
    tiles_left: &HashSet<usize>,
    matches: &HashMap<&Side, MatchingTiles>,
) -> TileAndOrientation<'a> {
    match col {
        0 if row == 0 => pick_corner(tiles, matches),
        0 => fit(Up, image[row - 1][col].side(Down), tiles, tiles_left),
        _ => fit(Left, image[row][col - 1].side(Right), tiles, tiles_left),
    }
}

pub fn a() -> String {
    let tiles = read_tiles("../input/day20");
    let image = order_tiles(&tiles);
    let len = image.len();
    let res = image[0][0].tile.id
        * image[0][len - 1].tile.id
        * image[len - 1][0].tile.id
        * image[len - 1][len - 1].tile.id;

    res.to_string()
}

fn order_tiles(tiles: &HashMap<usize, Tile>) -> Vec<Vec<TileAndOrientation>> {
    let matches: HashMap<&Side, MatchingTiles> = matching_sides(tiles);
    let mut image: Vec<Vec<TileAndOrientation>> = Vec::new();
    let len = (tiles.len() as f64).sqrt() as usize;
    let mut tiles_left: HashSet<usize> = tiles.keys().copied().collect();

    for row in 0..len {
        image.push(Vec::new());
        for col in 0..len {
            let tile = pick_next(row, col, &image, &tiles, &tiles_left, &matches);
            tiles_left.remove(&tile.tile.id);
            image[row].push(tile);
        }
    }
    image
}

fn picture_from(image: Vec<Vec<TileAndOrientation>>) -> Image {
    let mut picture: Image = Vec::new();
    for (row, tiles) in image.iter().enumerate() {
        for i in 1..9 {
            picture.push(Vec::new());
            for tile in tiles {
                let img = tile.aligned_image();
                picture[row * 8 + i - 1].extend_from_slice(&img[i][1..9]);
            }
        }
    }
    picture
}

fn _draw_picture(pic: &Image) {
    for i in pic {
        for x in i {
            print!("{}", x);
        }
        println!();
    }
}

fn count_hashes(pic: &Image) -> usize {
    pic.iter()
        .map(|row| row.iter().filter(|&&c| c == '#').count())
        .sum()
}

pub fn b() -> String {
    //_debug()
    let tiles = read_tiles("../input/day20");
    let image = order_tiles(&tiles);
    let picture = picture_from(image);

    let snake: Image =
        "                  # \n#    ##    ##    ###\n #  #  #  #  #  #   "
            .lines()
            .map(|l| l.chars().collect())
            .collect();

    assert!(snake.len() == 3);
    assert!(snake[0].len() == 20);

    let mut snakes = vec![snake];
    snakes.push(flip_image(&snakes[0]));
    for i in 0..3 {
        snakes.push(rotate_image(&snakes[i * 2]));
        snakes.push(rotate_image(&snakes[i * 2 + 1]));
    }
    (count_hashes(&picture)
        - snakes.iter().map(|s| find(s, &picture)).max().unwrap()
            * count_hashes(&snakes[0]))
    .to_string()
}

fn find(snake: &Image, sea: &Image) -> usize {
    let snake_cols = snake[0].len();
    let snake_rows = snake.len();
    let sea_cols = sea[0].len();
    let sea_rows = sea.len();
    let (mut x, mut y) = (0, 0);
    let mut res = 0;

    while y + snake_rows <= sea_rows {
        while x + snake_cols <= sea_cols {
            let is_match = snake.iter().enumerate().all(|(row, line)| {
                line.iter()
                    .enumerate()
                    .all(|(col, &c)| c == ' ' || sea[y + row][x + col] == '#')
            });
            if is_match {
                res += 1;
                x += snake_cols;
            } else {
                x += 1;
            }
        }
        y += 1;
        x = 0;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_side_from_str() {
        let side = ".#....#...";
        assert_eq!(Side::from_str(side).0, 2 + 64);
        let side_flip: String = side.chars().rev().collect();
        assert_eq!(Side::from_str(&side_flip).0, 8 + 256);
        let flip = Side::from_str(side).flip();
        assert_eq!(flip.0, 8 + 256);
    }
}
