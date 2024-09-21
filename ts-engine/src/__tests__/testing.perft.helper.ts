import {BB64Long} from '../BB64Long';
import {BoardState} from '../BoardState';
import {START_POS} from '../Fen';
import {Perft} from '../Perft';
import {Move} from '../Move';

//const board = BoardState.fromFen(START_POS);
//const board = BoardState.fromFen("r1bqkbnr/pppppppp/n7/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 2");
let board = BoardState.fromFen("r3k2r/8/5Q2/8/8/3q4/8/R3K2R w KQkq - 0 1");
board = board.doMove(Move.fromUciString("e1f2", board));
board = board.doMove(Move.fromUciString("e8d7", board));
board = board.doMove(Move.fromUciString("f6b2", board));
// const board = BoardState.fromFen("r1bqkbnr/pppppppp/n7/8/8/3P4/PPP1PPPP/RNBQKBNR w KQkq - 0 2");
console.info(Perft.perftString(board, 1));
