use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use crate::bitboard::Bitboard;

use crate::engine::{Engine, UciMessage};
use crate::fen::START_POS;
// use crate::fen::{configure_command_line_options, START_POS};
// use crate::transposition_table::DEFAULT_SIZE_MB;

pub struct EngineThread {
    pub rx: Receiver<UciMessage>,
    pub engine: Engine,
}

impl EngineThread {
    pub fn new_from_fen(rx: Receiver<UciMessage>, fen: &str) -> Self {
        // let b_instance = Bitboard::new();
        // let bitboard: &'a Bitboard = &b_instance;
        let engine = Engine::new_from_fen(fen);
        EngineThread {
            rx,
            engine,
        }
    }

    pub fn new(rx: Receiver<UciMessage>) -> Self {
        EngineThread::new_from_fen(rx, START_POS)
    }

    fn start_loop(&mut self) {
        loop {
            match self.rx.recv() {
                Ok(msg) => {
                    if !self.handle_message(msg) {
                        return;
                    }
                }
                Err(err) => {
                    println!("Engine communication error: {:?}", err);
                    return;
                }
            }
        }
    }

    fn handle_message(&mut self, msg: UciMessage) -> bool {
        match msg {
            UciMessage::Stop => (),
            UciMessage::UciCommand(uci_command) => {
                // println!("UciCommand: {}", uci_command);
                let _result = self.engine.process_uci_command(uci_command);
                //println!("UciCommand execution result:\n{}", result);
            }
        }
        true
    }
}

pub fn spawn_engine_thread() -> Sender<UciMessage> {
    let (tx, rx) = mpsc::channel::<UciMessage>();
    let bitboard = Bitboard::new();

    thread::spawn(move || {
        let mut engine = EngineThread::new(rx);
        // configure_command_line_options(&mut engine.engine.board);
        engine.start_loop();
    });

    tx
}

