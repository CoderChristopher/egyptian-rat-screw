use std::fmt;
use rand::Rng;

#[derive(Copy,Clone)]
enum CardValue {
    Ace = 0,
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
    Six = 5,
    Seven = 6,
    Eight = 7,
    Nine = 8,
    Ten = 9,
    Jack = 10,
    Queen = 11,
    King = 12
}

impl CardValue {
    fn from_index( index: usize ) -> Result< CardValue, () > {
        match index {
            0 =>Ok(CardValue::Ace),
            1 =>Ok(CardValue::Two),
            2 =>Ok(CardValue::Three),
            3 =>Ok(CardValue::Four),
            4 =>Ok(CardValue::Five),
            5 =>Ok(CardValue::Six),
            6 =>Ok(CardValue::Seven),
            7 =>Ok(CardValue::Eight),
            8 =>Ok(CardValue::Nine),
            9 =>Ok(CardValue::Ten),
            10 =>Ok(CardValue::Jack),
            11 =>Ok(CardValue::Queen),
            12 =>Ok(CardValue::King),
            _ => Err( () )
        }
    }
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

#[derive(Copy,Clone)]
enum CardSuite {
    Spade = 0,
    Club = 1,
    Diamond = 2,
    Heart = 3
}

impl CardSuite {
    fn from_index( index: usize ) -> Result< CardSuite, () > {
        match index {
            0=>Ok(CardSuite::Spade),
            1=>Ok(CardSuite::Club),
            2=>Ok(CardSuite::Diamond),
            3=>Ok(CardSuite::Heart),
            _=>Err( () )
        }
    }
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

#[derive(Copy,Clone)]
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
    fn add_card_by_index( &mut self, index: usize ) -> Result< (), ()> {
        if index >= 52 {
            return Err( () );
        }

        let value = index%12;
        let suite = (index+1)/12;

        self.cards.push( Card { 
            suite: CardSuite::from_index( suite)?,
            value: CardValue::from_index( value)? 
        });
        Ok( () )
    }
}

impl fmt::Display for Deck {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        for card in self.cards.iter() {
            write!( f, "{}", card )?;
        }

        Ok( () )
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

impl fmt::Display for Human {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!( f, "Id: {}", self.id )?;
        write!( f, "Deck: {}", self.deck)?;
        write!( f, "Active: {}", self.active )
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

impl fmt::Display for Robot {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!( f, "Id: {}", self.id )?;
        write!( f, "Deck: {}", self.deck)?;
        write!( f, "Reaction Speed Milliseconds: {}", self.reaction_speed_milliseconds )?;
        write!( f, "Reaction Speed Variance: {}", self.reaction_speed_variance )?;
        write!( f, "Active: {}", self.active )
    }
}

enum PlayerType {
    Human{ attributes: Human },
    Robot{ attributes: Robot }
}

impl fmt::Display for PlayerType {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        match self {
            PlayerType::Human{ attributes } => write!( f, "{}", attributes ),
            PlayerType::Robot{ attributes } => write!( f, "{}", attributes ),
        }
    }
}

enum GameState {
    NotStarted,
    Shuffling,
    Dealing,
    Playing,
    Complete
}

impl fmt::Display for GameState {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        match self {
            GameState::NotStarted => write!( f, "Not Started"),
            GameState::Shuffling => write!( f, "Suffling"),
            GameState::Dealing => write!( f, "Dealing"),
            GameState::Playing => write!( f, "Playing"),
            GameState::Complete => write!( f, "Complete")
        }
    }
}


struct Game {
    turn: usize,
    player_count: usize,
    players: Vec<PlayerType>,
    table_deck: Deck,
    id: usize,
    state: GameState
}

impl Game {
    fn new( player_count:usize ) -> Game {
        Game {
            turn: 0,
            player_count,
            players: Vec::with_capacity( player_count ),
            table_deck: Deck::new(),
            id: player_count + 1,
            state: GameState::NotStarted
        }
    }

    fn shuffle_cards( &mut self ) {
        let mut deck_to_shuffle = Deck::new();

        let mut card_index:Vec<usize> = Vec::new();

        for index in 0..52 {
            card_index.push( index );
        }

        for index in 0..52 {
            let pull_index = rand::thread_rng().gen_range(0..(52-index));

            let pulled_index = card_index.remove( pull_index );

            deck_to_shuffle.add_card_by_index( pulled_index );
        }

        for player_index in 0..self.player_count {
            self.players.push( PlayerType::Robot{ attributes: Robot::new( player_index, 500, 10 ) } ) 
        }

        let mut turn = 0;

        for card in deck_to_shuffle.cards.iter_mut() {
            match &mut self.players[turn] {
                PlayerType::Human{ attributes } => {
                    panic!( "I CANNOT HANDLE HUMAN INTERACTION!");
                },
                PlayerType::Robot{ attributes } => {
                    attributes.deck.cards.push( *card );
                }
            }

            turn += 1;

            if turn == self.player_count {
                turn = 0;
            }
        }
    }
}

impl fmt::Display for Game {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!( f, "Turn: {}", self.turn );
        write!( f, "Player Count: {}", self.player_count );
        write!( f, "ID: {}", self.id );
        write!( f, "Game State: {}", self.state );
        write!( f, "Table Deck: {}", self.table_deck );
        for player in self.players.iter() {
            write!( f, "Table Deck: {}", player );
        }
        Ok( () )
    }
}

fn main() {
    let mut game = Game::new(4);    

    println!( "{}", game );

    game.shuffle_cards(); 

    println!( "{}", game );
}
