//! サーチ☆（＾～＾）探索部とか言われてるやつだぜ☆（＾～＾）

use crate::look_and_model::{GameResult, Piece, Position, Search, BOARD_LEN, SQUARES_NUM};
use std::time::Instant;

impl Search {
    pub fn new(friend: Piece, start_pieces_num: usize, info_enable: bool) -> Self {
        Search {
            start_friend: friend,
            start_pieces_num: start_pieces_num,
            nodes: 0,
            stopwatch: Instant::now(),
            info_enable: info_enable,
        }
    }

    /// 最善の番地を返すぜ☆（＾～＾）
    pub fn go(&mut self, pos: &mut Position) -> (Option<u8>, GameResult) {
        if self.info_enable {
            self.info_header(pos);
        }
        self.node(pos)
    }

    /// 手番が来たぜ☆（＾～＾）いわゆる search だぜ☆（＾～＾）
    fn node(&mut self, pos: &mut Position) -> (Option<u8>, GameResult) {
        let mut best_addr = None;
        let mut best_result = GameResult::Lose;

        for addr in 1..BOARD_LEN {
            // 空きマスがあれば
            if let None = pos.board[addr] {
                // とりあえず置いてみようぜ☆（＾～＾）
                pos.do_move(addr);
                self.nodes += 1;

                // 前向き探索というのは、葉っぱの方に進んでるとき☆（＾～＾）
                // 後ろ向き探索というのは、根っこの方に戻ってるとき☆（＾～＾）
                //
                // 勝ったかどうか判定しようぜ☆（＾～＾）？
                if pos.is_opponent_win() {
                    // 勝ったなら☆（＾～＾）
                    // 前向き探索情報を出して、置いた石は戻して、後ろ向き探索情報を出して、探索終了だぜ☆（＾～＾）
                    if self.info_enable {
                        self.info_forward_leaf(
                            pos,
                            addr,
                            GameResult::Win,
                            Some("Hooray!".to_string()),
                        );
                    }
                    pos.undo_move();
                    if self.info_enable {
                        self.info_backward(pos, addr, GameResult::Win, None);
                    }
                    return (Some(addr as u8), GameResult::Win);
                } else if SQUARES_NUM <= pos.pieces_num {
                    // 勝っていなくて、深さ上限に達したら、〇×ゲームでは 他に置く場所もないから引き分け確定だぜ☆（＾～＾）
                    // 前向き探索情報を出して、置いた石は戻して、後ろ向き探索情報を出して、探索終了だぜ☆（＾～＾）
                    if self.info_enable {
                        self.info_forward_leaf(
                            pos,
                            addr,
                            GameResult::Draw,
                            Some("It's ok.".to_string()),
                        );
                    }
                    pos.undo_move();
                    if self.info_enable {
                        self.info_backward(pos, addr, GameResult::Draw, None);
                    }
                    return (Some(addr as u8), GameResult::Draw);
                } else {
                    // まだ続きがあるぜ☆（＾～＾）
                    if self.info_enable {
                        self.info_forward(pos, addr, Some("Search.".to_string()));
                    }
                }

                // 相手の番だぜ☆（＾～＾）
                let (_opponent_address, opponent_game_result) = self.node(pos);

                // 自分が置いたところを戻そうぜ☆（＾～＾）？
                pos.undo_move();

                match opponent_game_result {
                    // 相手の負けなら、この手で勝ちだぜ☆（＾～＾）後ろ向き探索情報を出して、探索終わり☆（＾～＾）
                    GameResult::Lose => {
                        if self.info_enable {
                            self.info_backward(pos, addr, GameResult::Win, Some("Ok.".to_string()));
                        }
                        return (Some(addr as u8), GameResult::Win);
                    }
                    // 勝ち負けがずっと見えてないなら☆（＾～＾）後ろ向き探索情報を出して、探索を続けるぜ☆（＾～＾）
                    GameResult::Draw => {
                        if self.info_enable {
                            self.info_backward(
                                pos,
                                addr,
                                GameResult::Draw,
                                Some("Fmmm.".to_string()),
                            );
                        }
                        match best_result {
                            GameResult::Lose => {
                                // 更新
                                best_addr = Some(addr as u8);
                                best_result = GameResult::Draw;
                            }
                            _ => {}
                        }
                    }
                    // 相手が勝つ手を選んではダメだぜ☆（＾～＾）後ろ向き探索情報を出して、探索を続けるぜ☆（＾～＾）
                    GameResult::Win => {
                        if self.info_enable {
                            self.info_backward(
                                pos,
                                addr,
                                GameResult::Lose,
                                Some("Resign.".to_string()),
                            );
                        }
                    }
                }
            }
        }

        (best_addr, best_result)
    }
}