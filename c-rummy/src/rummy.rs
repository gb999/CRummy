
enum Suit {
    Hearts,
    Tiles,
    Clover,
    Pikes
}

enum Card {
    Number(Suit, u8), // number between 2-10 TODO: make number type 
    Jack(Suit),
    Queen(Suit),
    King(Suit),
    Ace(Suit),
    Joker(Suit)
}

struct Deck {
    cards: Vec<Card>,
}

impl Default for Deck {
    /// Fills deck wit 2 decks of cards.
    fn default() -> Self {
        todo!(); 
    }
}

impl Deck {
    /// Draw a card 
    fn draw(&mut self) -> Card {
        todo!()
    }
}

struct Player {
    cards: Vec<Card>,
}

impl Player {
    fn new(cards: Vec<Card>) -> Player {
        Player {cards}
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

