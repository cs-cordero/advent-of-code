use advent_of_code::*;

fn main() {
    let input_lines = read_input_as_lines("2020/day12/src/input.txt")
        .into_iter()
        .map(|line| {
            let mut chars = line.chars();
            let command = chars.next().unwrap();
            let dist = chars.collect::<String>().as_str().parse::<i32>().unwrap();
            (command, dist)
        })
        .collect::<Vec<(char, i32)>>();

    let answer1 = {
        // Ship Position
        let mut posx = 0;
        let mut posy = 0;

        // Ship Orientation
        let mut dirx = 1;
        let mut diry = 0;

        for (command, dist) in input_lines.clone() {
            match command {
                'N' => posy += dist,
                'S' => posy -= dist,
                'E' => posx += dist,
                'W' => posx -= dist,
                'L' => {
                    let mut angle = dist;
                    while angle > 0 {
                        let (dx, dy) = (dirx, diry);
                        dirx = -dy;
                        diry = dx;
                        angle -= 90;
                    }
                }
                'R' => {
                    let mut angle = dist;
                    while angle > 0 {
                        let (dx, dy) = (dirx, diry);
                        dirx = dy;
                        diry = -dx;
                        angle -= 90;
                    }
                }
                'F' => {
                    posx += dirx * dist;
                    posy += diry * dist;
                }
                _ => panic!("Invalid"),
            };
        }
        posx.abs() + posy.abs()
    };

    let answer2 = {
        // Ship Position
        let mut posx = 0;
        let mut posy = 0;

        // Waypoint Position
        let mut wayx = 10;
        let mut wayy = 1;

        for (command, dist) in input_lines {
            match command {
                'N' => wayy += dist,
                'S' => wayy -= dist,
                'E' => wayx += dist,
                'W' => wayx -= dist,
                'L' => {
                    let mut angle = dist;
                    while angle > 0 {
                        let (wx, wy) = (wayx, wayy);
                        wayx = -wy;
                        wayy = wx;
                        angle -= 90;
                    }
                }
                'R' => {
                    let mut angle = dist;
                    while angle > 0 {
                        let (wx, wy) = (wayx, wayy);
                        wayx = wy;
                        wayy = -wx;
                        angle -= 90;
                    }
                }
                'F' => {
                    posx += wayx * dist;
                    posy += wayy * dist;
                }
                _ => panic!("Invalid"),
            }
        }

        posx.abs() + posy.abs()
    };

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}
