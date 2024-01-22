enum CardValue {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    Diamond
}

enum CardSuite {
    Spade,
    Club,
    Diamond,
    Heart
}

struct Card {
    suite: CardSuite,
    value: CardValue
}

struct Deck {
    cards: Vec<Card>
}

impl Deck {
    fn new() -> Deck {
        Deck {
            cards: Vec::new(),
        }
    }
}

struct Human {
    id: usize,
    deck: Deck,
    active: bool,
}

impl Human {
    fn new( id: usize ) -> Human {
        Human {
            id,
            deck: Deck::new(),
            active: true,
        }
    }
}

struct Robot {
    id: usize,
    deck: Deck,
    reaction_speed_milliseconds: usize,
    reaction_speed_variance: usize,
    active: false,
}

impl Robot {
    fn new( id: usize,reaction_speed_milliseconds: usize, reaction_speed_variance: usize ) -> Robot {
        Robot {
            id,
            deck: Deck::new(),
            reaction_speed_milliseconds,
            reaction_speed_variance,
            active: true,
        }
    }
}

enum PlayerType {
    Human{ attributes: Human },
    Robot{ attributes: Robot }
}


struct Game {
    turn: usize,
    players: Vec<PlayerType>
}

impl Game {
    fn new( player_count:usize ) -> Game {
        Game {
            turn: 0,
            players: Vec::with_capacity( player_count )
        }
    }
}

fn main() {

}
