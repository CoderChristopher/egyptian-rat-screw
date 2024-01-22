use std::fmt;

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
    King
}

impl fmt::Display for CardValue {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        match self {
            CardValue::Ace =>   write!( f, " A" ),
            CardValue::Two =>   write!( f, " 2" ),
            CardValue::Three => write!( f, " 3" ),
            CardValue::Four =>  write!( f, " 4" ),
            CardValue::Five =>  write!( f, " 5" ),
            CardValue::Six =>   write!( f, " 6" ),
            CardValue::Seven => write!( f, " 7" ),
            CardValue::Eight => write!( f, " 8" ),
            CardValue::Nine =>  write!( f, " 9" ),
            CardValue::Ten =>   write!( f, "10" ),
            CardValue::Jack =>  write!( f, " J" ),
            CardValue::Queen => write!( f, " Q" ),
            CardValue::King =>  write!( f, " K" ),
        }
    }
}

enum CardSuite {
    Spade,
    Club,
    Diamond,
    Heart
}

impl fmt::Display for CardSuite {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        match self {
            CardSuite::Spade =>   write!( f, "S" ),
            CardSuite::Club =>    write!( f, "C" ),
            CardSuite::Diamond => write!( f, "D" ),
            CardSuite::Heart =>   write!( f, "H" ),
        }
    }
}

struct Card {
    suite: CardSuite,
    value: CardValue
}

impl fmt::Display for Card {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!( f, "{}{}", self.value, self.suite)
    }
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
    active: bool,
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
    players: Vec<PlayerType>,
    table_deck: Deck,
    id: usize,
}

impl Game {
    fn new( player_count:usize ) -> Game {
        Game {
            turn: 0,
            players: Vec::with_capacity( player_count ),
            table_deck: Deck::new(),
            id: player_count + 1,
        }
    }
}

fn main() {
    let mut game = Game::new(4);    


}
