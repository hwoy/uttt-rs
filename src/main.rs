extern crate ttt_rs;
use ttt_rs::prelude::*;

extern crate ttt_io_rs;

fn summary(number_of_game: usize, draw_count: usize, win_count: usize) {
    println!("\n\n====================== SUMMARY ======================");
    println!("ROUND: <{}>", number_of_game);
    println!("DRAW: <{}>", draw_count);
    println!("WIN/LOSE: <{}>", win_count);
}

fn printtable(
    player1: &ttt_sys::ox_player,
    player2: &ttt_sys::ox_player,
    ch: &[char; 2],
    blank_ch: char,
) {
    ttt_io_rs::printttable(
        player1.val,
        player2.val,
        ch[player1.id as usize],
        ch[player2.id as usize],
        blank_ch,
    );
}

fn main() {
    const CH: [char; 2] = ['O', 'X'];
    const BLANK_CH: char = ' ';

    let number_of_game = 100usize;

    let mut draw_count = 0usize;
    let mut win_count = 0usize;

    let mut game = ttt_rs::build_game();

    for n_game in 0..number_of_game {
        let mut n_turn = 0usize;
        let mut players = ttt_rs::build_players();
        printtable(&players[0], &players[1], &CH, BLANK_CH);

        loop {
            n_turn += 1;
            let player2 = players.pop().unwrap();
            let mut player1 = players.pop().unwrap();

            let r = ttt_rs::Ai::ai(&mut game, &player2, &player1);

            println!("\nGAME#{} TURN#{}\n", n_game, n_turn);

            println!(
                "Player{} sets <{}> => <{}>",
                player1.id + 1,
                CH[player1.id as usize],
                r
            );

            let gameid = game.gameplay(&player2, &mut player1, r as u32);
            println!("Game ID: {:?}\n", gameid);

            match gameid {
                ttt_sys::ox_gameid::ox_idgame => {}
                ttt_sys::ox_gameid::ox_idwin | ttt_sys::ox_gameid::ox_iddraw => {
                    match gameid {
                        ttt_sys::ox_gameid::ox_idwin => {
                            println!(
                                "Game over: Player{}[{}] wins!",
                                player1.id + 1,
                                CH[player1.id as usize]
                            );
                            win_count += 1;
                        }
                        _ => {
                            println!("Game over: Draw!");
                            draw_count += 1;
                        }
                    }

                    players.push(player1);
                    players.push(player2);
                    break;
                }
                _ => panic!("This is impossible!!!"),
            }

            printtable(&player1, &player2, &CH, BLANK_CH);

            players.push(player2);
            players.push(player1);
        }

        let player2 = players.pop().unwrap();
        let player1 = players.pop().unwrap();

        printtable(&player1, &player2, &CH, BLANK_CH);
    }

    summary(number_of_game, draw_count, win_count);
}
