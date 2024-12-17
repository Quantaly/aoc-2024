use anyhow::{anyhow, Result};
use aoc_2024::{
    board::{Board, Direction},
    input_string,
};
use pest::Parser;
use pest_derive::Parser;
use std::collections::BTreeSet;

#[derive(Parser)]
#[grammar = "src/bin/15/grammar.pest"]
struct InputParser;

fn main() -> Result<()> {
    let input = input_string()?;
    let mut parsed = InputParser::parse(Rule::file, &input)?;

    let board = Board::read(parsed.next().unwrap().as_str().trim_end().as_bytes())?;
    let moves: Vec<_> = parsed
        .next()
        .unwrap()
        .into_inner()
        .map(|movement| match movement.as_rule() {
            Rule::move_up => Direction::North,
            Rule::move_down => Direction::South,
            Rule::move_left => Direction::West,
            Rule::move_right => Direction::East,
            _ => unreachable!(),
        })
        .collect();

    {
        // part 1
        let mut board = board.clone();
        let mut robot_position = None;
        'find_robot: for (i, row) in board.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell == b'@' {
                    robot_position = Some((i, j));
                    break 'find_robot;
                }
            }
        }
        let mut robot_position = robot_position.ok_or_else(|| anyhow!("no robot"))?;

        fn try_move(board: &mut Board, position: (usize, usize), direction: Direction) -> bool {
            if let Some(new_position) = direction.do_move(position) {
                match board.get_at(new_position) {
                    Some(b'.') => {
                        board[new_position.0][new_position.1] = board[position.0][position.1];
                        board[position.0][position.1] = b'.';
                        true
                    }
                    Some(b'O') => {
                        if try_move(board, new_position, direction) {
                            board[new_position.0][new_position.1] = board[position.0][position.1];
                            board[position.0][position.1] = b'.';
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                }
            } else {
                false
            }
        }

        for &direction in &moves {
            if try_move(&mut board, robot_position, direction) {
                robot_position = direction
                    .do_move(robot_position)
                    .expect("try_move only returns true if this is Some")
            }
        }

        let gps_sum = board
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter().enumerate().filter_map(move |(j, &cell)| {
                    if cell == b'O' {
                        Some(100 * i + j)
                    } else {
                        None
                    }
                })
            })
            .sum::<usize>();
        println!("{gps_sum}");
    }

    {
        // part 2
        let mut walls = BTreeSet::new();
        let mut boxes = BTreeSet::new();
        let mut robot_position = None;
        for (i, row) in board.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                match cell {
                    b'#' => {
                        walls.extend([(i, 2 * j), (i, 2 * j + 1)]);
                    }
                    b'O' => {
                        boxes.insert((i, 2 * j));
                    }
                    b'@' => {
                        robot_position = Some((i, 2 * j));
                    }
                    _ => {}
                }
            }
        }
        let walls = walls;
        let mut robot_position =
            robot_position.expect("we got through part 1 so there's def a robot");

        // fn print_board(
        //     height: usize,
        //     width: usize,
        //     walls: &BTreeSet<(usize, usize)>,
        //     boxes: &BTreeSet<(usize, usize)>,
        //     robot_position: (usize, usize),
        // ) {
        //     for i in 0..height {
        //         for j in 0..width {
        //             if walls.contains(&(i, j)) {
        //                 eprint!("#");
        //             } else if boxes.contains(&(i, j)) {
        //                 eprint!("[");
        //             } else if boxes.contains(&(i, j - 1)) {
        //                 eprint!("]");
        //             } else if robot_position == (i, j) {
        //                 eprint!("@");
        //             } else {
        //                 eprint!(".");
        //             }
        //         }
        //         eprintln!();
        //     }
        //     eprintln!();
        // }

        'moveLoop: for &direction in &moves {
            // print_board(
            //     board.len(),
            //     board[0].len() * 2,
            //     &walls,
            //     &boxes,
            //     robot_position,
            // );
            if let Some(new_robot_position) = direction.do_move(robot_position) {
                if walls.contains(&new_robot_position) {
                    continue;
                }

                let mut unchecked_boxes: BTreeSet<_> = boxes
                    .range(
                        (new_robot_position.0, new_robot_position.1.saturating_sub(1))
                            ..=new_robot_position,
                    )
                    .copied()
                    .collect();
                let mut checked_boxes = BTreeSet::new();
                while let Some(box_position) = unchecked_boxes.pop_first() {
                    checked_boxes.insert(box_position);
                    if let Some(new_box_position) = direction.do_move(box_position) {
                        if walls
                            .range(
                                new_box_position
                                    ..=(new_box_position.0, new_box_position.1.saturating_add(1)),
                            )
                            .any(|_| true)
                        {
                            continue 'moveLoop;
                        }
                        unchecked_boxes.extend(
                            boxes
                                .range(
                                    (new_box_position.0, new_box_position.1.saturating_sub(1))
                                        ..=(
                                            new_box_position.0,
                                            new_box_position.1.saturating_add(1),
                                        ),
                                )
                                .copied()
                                .filter(|box_position| !checked_boxes.contains(box_position)),
                        );
                    } else {
                        continue 'moveLoop;
                    }
                }

                // checked_boxes is full of all the boxes that need to move and they all can
                for box_position in &checked_boxes {
                    boxes.remove(box_position);
                }
                boxes.extend(checked_boxes.into_iter().map(|box_position| direction.do_move(box_position).expect("we already tried this in the loop above and wouldn't have gotten here if it wasn't Some")));

                robot_position = new_robot_position;
            }
        }

        // print_board(
        //     board.len(),
        //     board[0].len() * 2,
        //     &walls,
        //     &boxes,
        //     robot_position,
        // );
        let gps_sum = boxes.into_iter().map(|(i, j)| 100 * i + j).sum::<usize>();
        println!("{gps_sum}");
    }

    Ok(())
}
