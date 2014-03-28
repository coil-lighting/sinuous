//! problem 54 from project euler
//!
//! In the card game poker, a hand consists of five cards and are ranked, from lowest to highest, in the following way:
//!
//! High Card: Highest value card.
//! One Pair: Two cards of the same value.
//! Two Pairs: Two different pairs.
//! Three of a Kind: Three cards of the same value.
//! Straight: All cards are consecutive values.
//! Flush: All cards of the same suit.
//! Full House: Three of a kind and a pair.
//! Four of a Kind: Four cards of the same value.
//! Straight Flush: All cards are consecutive values of same suit.
//! Royal Flush: Ten, Jack, Queen, King, Ace, in same suit.
//! The cards are valued in the order:
//! 2, 3, 4, 5, 6, 7, 8, 9, 10, Jack, Queen, King, Ace.
//!
//! If two players have the same ranked hands then the rank made up of the highest value wins; for example, a pair of eights beats a pair of fives (see example 1 below). But if two ranks tie, for example, both players have a pair of queens, then highest cards in each hand are compared (see example 4 below); if the highest cards tie then the next highest cards are compared, and so on.
//!
//! The file, poker.txt, contains one-thousand random hands dealt to two players. Each line of the file contains ten cards (separated by a single space): the first five are Player 1's cards and the last five are Player 2's cards. You can assume that all hands are valid (no invalid characters or repeated cards), each player's hand is in no specific order, and in each hand there is a clear winner.
//!
//! How many hands does Player 1 win?

use std::io::BufferedReader;
use std::io::File;

// Suits
// Need this compiler flag to enable equality operators
#[deriving(Eq)]
enum Suit {
	Hearts,
	Diamonds,
	Clubs,
	Spades
}

#[deriving(Eq, Ord)]
enum Rank {
	NoRank = 0,
	HighCard = 1,
	Pair = 2,
	TwoPair = 3,
	ThreeKind = 4,
	Straight = 5,
	Flush = 6,
	FullHouse = 7,
	FourKind = 8,
	StraightFlush = 9,
	RoyalFlush = 10
}

// A Card has a value 2-14 (ace is 14) and a Suit
struct Card {
	value: uint,
	suit: Suit
}

// implement traits necessary to sort a vector of Card
impl Eq for Card {
	fn eq(&self, other: &Card) -> bool {
		self.value == other.value
	}
}

impl TotalEq for Card {}

impl Ord for Card {
	fn lt(&self, other: &Card) -> bool {
		self.value < other.value
	}
}

impl TotalOrd for Card {
	fn cmp(&self, other: &Card) -> Ordering {
		if self.value < other.value {
			Less
		}
		else if self.value > other.value {
			Greater
		}
		else {
			Equal
		}
	}
}

// parse a hand from a string of value/suit pairs, returns sorted
fn hand_from_str(handstr: &str) -> Hand {
	let mut hand = Hand::new();
	for cardstr in handstr.words() {
		let val;
		// get the card value character
		let valchar = cardstr.char_at(0);

		let mut errval = ~"could not parse value character: ";
		errval.push_char(valchar);

		// try to convert it to a digit
		match std::char::to_digit(valchar,10) {
			// if it is a digit, set the value
			Some(num) => val = num,
			// if it is not a digit, parse the char
			None => {
				// match the possibilities
				match valchar {
					'T' => val = 10,
					'J' => val = 11,
					'Q' => val = 12,
					'K' => val = 13,
					'A' => val = 14,
					_ => fail!(errval)
				}
			}
		}

		let suit;
		let suitchar = cardstr.char_at(1);
		let mut errsuit = ~"could not parse suit character: ";
		errsuit.push_char(suitchar);
		match suitchar {
			'H' => suit = Hearts,
			'D' => suit = Diamonds,
			'C' => suit = Clubs,
			'S' => suit = Spades,
			_ => fail!(errsuit)
		}

		// now we know the value and the suit, make a card
		hand.add(Card{value: val, suit: suit});
	}

	hand.sort();

	hand
}

fn main() {
	let path = Path::new("poker.txt");
	let mut file = BufferedReader::new(File::open(&path));

	let mut rank_tally: [int, ..11] = [0, ..11];

	let mut win_tally: [int, ..3] = [0, ..3];

	for line in file.lines() {

		let mut hand1 = hand_from_str(line.clone().unwrap().slice_to(14));
		let mut hand2 = hand_from_str(line.clone().unwrap().slice_from(14));

		let (winner, rank1, rank2) = fight(&mut hand1, &mut hand2);
		match winner {
			Player1 => {win_tally[0] += 1;},
			Player2 => {win_tally[1] += 1;},
			Tie => {win_tally[2] += 1}
		}
		rank_tally[rank1 as int] += 1;
		rank_tally[rank2 as int] += 1;
	}

	println!("{:?}",win_tally);
	println!("{:?}",rank_tally);


}

#[deriving(Eq)]
enum FightResult {
	Player1,
	Player2,
	Tie
}

// two hand fight.  a fight result and the two hand ranks
fn fight(player1: &mut Hand, player2: &mut Hand) -> (FightResult, Rank, Rank) {

	let (p1rank, p1tie) = player1.rank();
	let (p2rank, p2tie) = player2.rank();

	let victor: FightResult;

	if p1rank > p2rank {
		victor = Player1;
	}
	else if p2rank > p1rank {
		victor = Player2;
	}
	else {
		victor = tiebreak(p1tie,p2tie);
	}

	(victor, p1rank, p2rank)

}

fn tiebreak(p1: &[uint], p2: &[uint]) -> FightResult {
	for pair in p1.iter().zip(p2.iter()) {
		match pair {
			(p1v, p2v) if p1v > p2v => {return Player1;},
			(p1v, p2v) if p1v < p2v => {return Player2;},
			_ => {}
		}
	}
	return Tie;
}

/* compiler doesn't like FightResult used here without defining a trait
// function works for ints
#[test]
fn test_tiebreak(){

	assert_eq!( tiebreak(&[5,3,1], &[4,3,1]), Player1);
	assert_eq!( tiebreak(&[4,3,1], &[5,3,1]), Player2);
	assert_eq!( tiebreak(&[3,2,1], &[3,2,1]), Tie);
}
*/


// hand object
struct Hand {
	cards: ~[Card]
}

// methods on a Hand
impl Hand {
	// get a new hand
	fn new() -> Hand {
		Hand{cards: ~[]}
	}

	// add a card to the hand
	fn add(&mut self, card: Card) {
		self.cards.push(card);
	}

	// sort the hand
	fn sort(&mut self) {
		self.cards.sort();
	}

	fn get_suits(&self) -> ~[Suit] {
		self.cards.iter().map(|&card| card.suit).collect()
	}

	fn get_values(&self) -> ~[uint] {
		self.cards.iter().map(|&card| card.value).collect()
	}

	fn get_values_descend(&self) -> ~[uint] {
		self.cards.rev_iter().map(|&card| card.value).collect()
	}

	// rank the hand
	fn rank(&mut self) -> (Rank, ~[uint]) {

		let flush = is_flush_pattern(self);
		let (straight, wheel) = is_straight_pattern(self);

		// if this is "the wheel", fix the value of the high card and resort
		if wheel {
			self.cards[4].value = 1;
			self.sort();
		}

		// if this is a straight flush
		if straight && flush {
			// is this a royal flush? (YEAH BABY)
			if self.cards[4].value == 14 {
				return (RoyalFlush,~[0]);
			}
			else {
				return (StraightFlush,self.get_values_descend());
			}
		}

		let mut tiebreak: ~[uint];

		if match fourkind(self) {
			(is,tie) => {tiebreak = tie; is}
		} { return (FourKind,tiebreak); }

		else if match fullhouse(self) {
			(is,tie) => {tiebreak = tie; is},
		} { return (FullHouse,tiebreak); }

		else if flush {
			return (Flush,self.get_values_descend());
		}

		else if straight {
			return (Straight,self.get_values_descend());
		}

		else if match threekind(self) {
			(is,tie) => {tiebreak = tie; is},
		} { return (ThreeKind,tiebreak); }

		else if match twopair(self) {
			(is,tie) => {tiebreak = tie; is},
		} { return (TwoPair,tiebreak); }

		else if match pair(self) {
			(is,tie) => {tiebreak = tie; is},
		} { return (Pair,tiebreak); }

		else {
			return (HighCard,self.get_values_descend());
		}

	}
}
/*
	NoRank = 0,
	HighCard = 1,
	Pair = 2,
	TwoPair = 3,
	ThreeKind = 4,
	Straight = 5,
	Flush = 6,
	FullHouse = 7,
	FourKind = 8,
	StraightFlush = 9,
	RoyalFlush = 10
	*/

// Is the hand a flush?
// DEPRECATED FOR PATTERN MATCHING VERSION
fn is_flush(cards: &~[Card]) -> bool {

	// get an iterator over the cards
	let mut card_iter = cards.iter();

	// get the suit of the first card
	let suit = (card_iter.next().unwrap()).suit;

	let mut flush = true;

	// iterate over the rest of the hand and check the suit
	for card in card_iter {
		if card.suit != suit {
			flush = false;
			break;
		}
	}

	flush

}

#[test]
fn test_is_flush(){

	let hand1 = ~[Card{value: 2, suit: Hearts}, Card{value: 3, suit: Hearts}, Card{value: 5, suit: Hearts}, Card{value: 7, suit: Hearts} ];
	assert!(is_flush(&hand1));

	let hand2 = ~[Card{value: 2, suit: Hearts}, Card{value: 3, suit: Hearts}, Card{value: 8, suit: Hearts}, Card{value: 2, suit: Diamonds} ];
	assert!(!is_flush(&hand2));

}


fn is_flush_pattern(hand: &Hand) -> bool {
	let suits = hand.get_suits();
	match suits.slice_from(0) {
		[s1, s2, s3, s4, s5] if s1 == s2 && s1 == s3 && s1 == s4 && s1 == s5 => true,
		_ => false
	}
}

#[test]
fn test_is_flush_pattern(){
	let cards1 = ~[	Card{value: 2, suit: Hearts},
					Card{value: 3, suit: Hearts},
					Card{value: 5, suit: Hearts},
					Card{value: 7, suit: Hearts},
					Card{value: 8, suit: Hearts} ];
	let mut hand1 = Hand::new();
	for card in cards1.iter() {
		hand1.add(*card);
	}
	assert!(is_flush_pattern(&hand1));

	let cards2 = ~[ Card{value: 2, suit: Hearts},
					Card{value: 3, suit: Hearts},
					Card{value: 8, suit: Hearts},
					Card{value: 7, suit: Hearts},
					Card{value: 2, suit: Diamonds} ];
	let mut hand2 = Hand::new();
	for card in cards2.iter() {
		hand2.add(*card);
	}
	assert!(!is_flush_pattern(&hand2));
}


// Is the hand a straight?
// THIS ASSUMES THE HAND IS SORTED!
fn is_straight(cards: &~[Card]) -> (bool, bool) {

	// get two iterators over the cards
	let mut card_iter1 = cards.iter();
	let mut card_iter2 = cards.iter();

	// increment the second iterator to the second card
	card_iter2.next();

	let mut straight = true;

	// loop over the second card
	for card2 in card_iter2 {

		// get the first card
		let card1 = card_iter1.next();
		// if the second card isn't the first card + 1, not a straight
		if card2.value != card1.unwrap().value + 1 {
			straight = false;
			break;
		}
	}

	let mut vals = ~[];

	for card in cards.iter() {
		vals.push(card.value);
	}

	let mut wheel = false;

	// handle the edge case of the wheel (ace 2 3 4 5)
	if vals == ~[2,3,4,5,14] {
		straight = true;
		wheel = true;
	}

	(straight, wheel)
}


#[test]
fn test_is_straight(){
	let hand1 = ~[	Card{value: 4, suit: Hearts}, Card{value: 5, suit: Hearts},
					Card{value: 6, suit: Hearts}, Card{value: 7, suit: Hearts},
					Card{value: 8, suit: Hearts}];

	match is_straight(&hand1) {
		(s,w) => {assert!(s); assert!(!w);}
	}

	let hand2 = ~[	Card{value: 4, suit: Hearts}, Card{value: 5, suit: Hearts},
					Card{value: 7, suit: Hearts}, Card{value: 7, suit: Hearts},
					Card{value: 8, suit: Hearts}];

	match is_straight(&hand2) {
		(s,w) => {assert!(!s); assert!(!w);}
	}

	let hand3 = ~[	Card{value: 2, suit: Hearts}, Card{value: 3, suit: Hearts},
				Card{value: 4, suit: Hearts}, Card{value: 5, suit: Hearts},
				Card{value: 14, suit: Hearts}];

	match is_straight(&hand3) {
		(s,w) => {assert!(s); assert!(w);}
	}
}

fn is_straight_pattern(hand: &Hand) -> (bool, bool) {
	let values = hand.get_values();
	match values.slice_from(0) {
		// if we have the wheel
		[2, 3, 4, 5, 14] => (true, true),
		[v1, v2, v3, v4, v5] if (v2 == v1+1) && (v3 == v2+1) && (v4 == v3+1) && (v5 == v4+1) => (true,false),
		_ => (false, false)
	}
}

#[test]
fn test_is_straight_pattern(){
	let cards1 = &"4H 5H 6H 7H 8H";

	let hand1 = hand_from_str(cards1);

	match is_straight_pattern(&hand1) {
		(s,w) => {assert!(s); assert!(!w);}
	}

	let cards2 = &"4H 5H 7H 7H 8H";
	let hand2 = hand_from_str(cards2);

	match is_straight_pattern(&hand2) {
		(s,w) => {assert!(!s); assert!(!w);}
	}

	let cards3 = &"2H 3H 4H 5H AH";
	let hand3 = hand_from_str(cards3);

	match is_straight_pattern(&hand3) {
		(s,w) => {assert!(s); assert!(w);}
	}
}

// quick helper function to check if every element of a vector is equal
fn vec_eq<T: Eq>(vector: &[T]) -> bool {
	let mut iter = vector.iter();
	let first = iter.next().unwrap();
	let mut iseq = true;
	for val in iter {
		if val != first {
			iseq = false;
			break;
		}
	}
	iseq
}

#[test]
fn test_vec_eq_int(){
	let v1 = [1,1,1,1,1,1];
	assert!(vec_eq(v1));

	let v2 = [1,2,3,4,5,6];
	assert!(!vec_eq(v2));

	let v3 = [1];
	assert!(vec_eq(v3));
}

// is the hand four of a kind?  returns value of the four as well
fn fourkind(hand: &Hand) -> (bool, ~[uint]) {
	let values = hand.get_values();
	let mut fourkind = false;
	let mut value = 0;
	for fours in values.windows(4) {
		if vec_eq(fours) {
			fourkind = true;
			value = fours[0];
			break;
		}
	}

	(fourkind, ~[value])
}

#[test]
fn test_fourkind(){
	let hand1 = hand_from_str(&"3H 3C 3D 3S 5D");
	match fourkind(&hand1) {
		(f,h) => {assert!(f); assert_eq!(h,~[3]); }
	}


	let hand2 = hand_from_str(&"2H KC KS KD KH");
	match fourkind(&hand2) {
		(f,h) => {assert!(f); assert_eq!(h,~[13]); }
	}

	let hand3 = hand_from_str(&"2H 2D 2C 3S 3D");
	match fourkind(&hand3) {
		(f,h) => {assert!(!f); assert_eq!(h,~[0]); }
	}
}

// is a full house?  also returns high card
fn fullhouse(hand: &Hand) -> (bool,~[uint]) {
	let values = hand.get_values();
	if vec_eq(values.slice(0,3)) && vec_eq(values.slice(3,5)) {
		(true,~[values[0]])
	}
	else if vec_eq(values.slice(0,2)) && vec_eq(values.slice(2,5)) {
		(true,~[values[2]])
	}
	else {
		(false, ~[0])
	}
}

#[test]
fn test_fullhouse(){
	let hand1 = hand_from_str(&"3D 3C 3S AD AS");
	match fullhouse(&hand1) {
		(f,h) => {assert!(f); assert_eq!(h,~[3]); }
	}

	let hand2 = hand_from_str(&"7D 7C 5D 5S 5H");
	match fullhouse(&hand2) {
		(f,h) => {assert!(f); assert_eq!(h,~[5]); }
	}

	let hand3 = hand_from_str(&"6D 7C 5D 5S 5H");
	match fullhouse(&hand3) {
		(f,h) => {assert!(!f); assert_eq!(h,~[0]); }
	}

}

fn threekind(hand: &Hand) -> (bool, ~[uint]) {
	let values = hand.get_values();
	let mut threekind = false;
	let mut value = 0;
	for threes in values.windows(3) {
		if vec_eq(threes) {
			threekind = true;
			value = threes[0];
			break;
		}
	}

	(threekind, ~[value])
}

// is two pair?  returns high pair, low pair, and high card
fn twopair(hand: &Hand) -> (bool, ~[uint]) {
	let values = hand.get_values();
	let mut onepair = false;
	let mut twopair = false;

	let mut hc: ~[uint] = ~[];
	let mut lowpair = 0;
	let mut highpair = 0;

	let mut pair_iter = values.windows(2);
	for pairs in pair_iter {
		if vec_eq(pairs) {
			onepair = true;
			lowpair = pairs[0];
			break;
		}
	}
	if onepair {
		pair_iter.next();
		for morepairs in pair_iter {
			if vec_eq(morepairs) {
				twopair = true;
				highpair = morepairs[0];
				break;
			}
		}
	}

	hc.push(highpair);
	hc.push(lowpair);

	for card in values.iter() {
		if *card != lowpair && *card != highpair {
			hc.push(*card);
		}
	}

	(twopair,hc)

}

#[test]
fn test_twopair(){
	let hand1 = hand_from_str(&"3D 3C 2S 2D AS");
	match twopair(&hand1) {
		(f,hc) => {assert!(f); assert_eq!(hc,~[3, 2,14])}
	}

	let hand2 = hand_from_str(&"2D 2C TS TD 7S");
	match twopair(&hand2) {
		(f,hc) => {assert!(f); assert_eq!(hc,~[10,2,7])}
	}

	let hand3 = hand_from_str(&"6D 3C 2S 2D AS");
	match twopair(&hand3) {
		(f,_) => {assert!(!f);}
	}
}

fn pair(hand: &Hand) -> (bool, ~[uint]) {
	let values = hand.get_values();
	let mut pair = false;
	let mut hc: ~[uint] = ~[];
	for pairs in values.windows(2) {
		if vec_eq(pairs) {
			pair = true;
			hc.push(pairs[0]);
			break;
		}
	}

	if pair {
		for card in values.rev_iter() {
			if *card != hc[0] {
				hc.push(*card);
			}
		}
	}

	(pair, hc)
}


#[test]
fn test_pair(){
	let hand1 = hand_from_str(&"2S 2D 3D 4C AS");
	match pair(&hand1) {
		(f,hc) => {assert!(f); assert_eq!(hc,~[2,14,4,3]); }
	}

	let hand2 = hand_from_str(&"2S 3D 5D AC AS");
	match pair(&hand2) {
		(f,hc) => {assert!(f); assert_eq!(hc,~[14,5,3,2]); }
	}

	let hand3 = hand_from_str(&"2S 3D 5D KC AS");
	match pair(&hand3) {
		(f,_) => {assert!(!f); }
	}
}
