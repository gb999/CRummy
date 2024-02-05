use std::marker::PhantomData;

use deckofcards::*; 


struct Player {
    hand: Hand,
}

impl Player {
    fn new(hand: Hand) -> Player {
        Player {hand}
    }
}

struct ValidGame {
    deck: Deck,
    cards_on_board: Vec<Card>,
    players: Vec<Player>,
    current_player: Player
}

struct TurnInProgressGame {
    deck: Deck,
    cards_on_board: Vec<Card>,
    cards_to_be_placed: Vec<Card>,
    players: Vec<Player>,
    current_player: Player
}

enum GameState {
    Valid(ValidGame),
    TurnInProgress(TurnInProgressGame)
}

struct Game {
    deck: Deck,
    discard_pile: Vec<Card>, // Deck trait?
    turn_history: Vec<Move>,
    cards_on_board: Vec<Card>,
    
    players: Vec<Player>,
    current_player: Player
} 

enum Place {
    Deck,
    DiscardPile,
    Board,
    Hand
}
struct Move {
    from: Place,
    to: Place,
    card: Card
}

