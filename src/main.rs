use rand::seq::SliceRandom;
use rand::thread_rng;
use std::num::Wrapping;
mod tables;
mod deck;
use deck::Card;
use deck::Value;
use deck::Suit;
use tables::FLUSH_RANKINGS;
use tables::HC_STRAIGHT_RANKINGS;
use tables::HASH_ADJUST;
use tables::HASH_TABLE;

fn evaluate_hand(hand: &Vec::<&Card>) {
    let mut check_flush = 0xF000;
    let mut idx = 0;
    for card in hand {
        check_flush &= card.enc;
        idx |= card.enc;
    }
    let idx: usize = (idx >> 16).try_into().unwrap();
    if check_flush > 0 {
        return FLUSH_RANKINGS[idx];
    }
    
    if HC_STRAIGHT_RANKINGS[idx] > 0 {
        return HC_STRAIGHT_RANKINGS[idx];
    }

    let mut prime_key = Wrapping(1u32);
    for card in hand {
        prime_key *= card.enc & 0xFF;
    }
    prime_key += 0xe91aaa35;
    prime_key ^= prime_key >> 16;
    prime_key += prime_key << 8;
    prime_key ^= prime_key >> 4;
    let b: u32 = (prime_key.0 >> 8) & 0x1ff;
    let a: u32 = ((prime_key + (prime_key << 2)) >> 19).0;
    let b_usize: usize = b.try_into().unwrap();
    let hashed: u32 = HASH_ADJUST[b_usize];

    // Convert hashed to usize and calculate r
    let hashed_usize: usize = hashed.try_into().unwrap();
    let r: usize = a as usize ^ hashed_usize;
    let ranking: u32 = HASH_TABLE[r];
    return ranking
    /*let hand_rank =match ranking {
        6187..=7462 => println!("HIGH CARD!"),
        3326..=6186 => println!("ONE PAIR!"),
        2468..=3325 => println!("TWO PAIR!"),
        1610..=2467 => println!("THREE OF A KIND!"),
        1600..=1609 => println!("STRAIGHT!"),
        323..=1599 => println!("FLUSH!"),
        167..=322 => println!("FULL HOUSE!"),
        11..=166 => println!("FOUR OF A KIND!"),
        _ => println!("STRAIGHT FLUSH!"),
    }*/
}


fn main() {
    
    let _card = Card::new(Value::Jack, Suit::Spade);
    
    let mut deck: Vec<Card> = Vec::new();
    //init_deck(int *deck)


    // for (int i = 0; i < 4; i++, suit >>= 1)
    //     for (int j = 0; j < 13; j++, n++)
    //         deck[n] = primes[j] | (j << 8) | suit | (1 << (16+j));

    for s in &Suit::suits() {
        for v in &Value::values(){
            deck.push(Card::new(*v, *s));
        }
    }

    deck.shuffle(&mut thread_rng());
    let mut hand_one = Vec::with_capacity(2);
    let mut hand_two = Vec::with_capacity(2);
    let mut board = Vec::with_capacity(5);

    let mut hands = vec![&mut hand_one, &mut hand_two, &mut board];

    for hand in hands.iter_mut() {
        for _card in 0..hand.capacity() {
            hand.push(deck.pop().unwrap());
        }
    }

    
}