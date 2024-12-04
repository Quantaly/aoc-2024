use anyhow::Result;
use aoc_2024::input_buf_read;
use std::io::BufRead;

type Movement = fn(usize, usize) -> Option<usize>;
static MOVEMENTS: &[(Movement, Movement)] = &[
    (usize::checked_sub, usize::checked_sub),
    (usize::checked_sub, no_move),
    (usize::checked_sub, usize::checked_add),
    (no_move, usize::checked_sub),
    // (no_move, no_move) is silly
    (no_move, usize::checked_add),
    (usize::checked_add, usize::checked_sub),
    (usize::checked_add, no_move),
    (usize::checked_add, usize::checked_add),
];

fn no_move(x: usize, _: usize) -> Option<usize> {
    Some(x)
}

fn get_at(board: &Vec<Vec<u8>>, (i, j): (usize, usize)) -> Option<u8> {
    board.get(i).and_then(|row| row.get(j)).copied()
}

static XMAS_LETTERS: &[u8] = &[b'X', b'M', b'A', b'S'];

fn check_xmas(
    board: &Vec<Vec<u8>>,
    remaining_letters: &[u8],
    (i, j): (usize, usize),
    &(move_i, move_j): &(Movement, Movement),
) -> bool {
    if remaining_letters.is_empty() {
        true
    } else if get_at(board, (i, j)) != Some(remaining_letters[0]) {
        false
    } else if remaining_letters.len() == 1 {
        true
    } else if let (Some(i), Some(j)) = (move_i(i, 1), move_j(j, 1)) {
        check_xmas(board, &remaining_letters[1..], (i, j), &(move_i, move_j))
    } else {
        false
    }
}

fn check_mas(
    board: &Vec<Vec<u8>>,
    (i, j): (usize, usize),
    &(move_i_back, move_j_back): &(Movement, Movement),
    &(move_i_forward, move_j_forward): &(Movement, Movement),
) -> bool {
    if let ((Some(i_back), Some(j_back)), (Some(i_forward), Some(j_forward))) = (
        (move_i_back(i, 1), move_j_back(j, 1)),
        (move_i_forward(i, 1), move_j_forward(j, 1)),
    ) {
        if let (Some(back), Some(at), Some(forward)) = (
            get_at(board, (i_back, j_back)),
            get_at(board, (i, j)),
            get_at(board, (i_forward, j_forward)),
        ) {
            at == b'A' && (back == b'M' && forward == b'S' || back == b'S' && forward == b'M')
        } else {
            false
        }
    } else {
        false
    }
}

fn main() -> Result<()> {
    let input = input_buf_read()?;
    let board: Vec<Vec<u8>> = input.split(b'\n').collect::<Result<_, _>>()?;

    let mut xmas_count = 0;
    let mut x_mas_count = 0;
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            for movement in MOVEMENTS {
                if check_xmas(&board, XMAS_LETTERS, (i, j), movement) {
                    xmas_count += 1;
                }
            }

            if check_mas(
                &board,
                (i, j),
                &(usize::checked_sub, usize::checked_sub),
                &(usize::checked_add, usize::checked_add),
            ) && check_mas(
                &board,
                (i, j),
                &(usize::checked_sub, usize::checked_add),
                &(usize::checked_add, usize::checked_sub),
            ) {
                x_mas_count += 1;
            }
        }
    }

    println!("{xmas_count}");
    println!("{x_mas_count}");

    Ok(())
}
