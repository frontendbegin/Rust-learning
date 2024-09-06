use core::fmt;
use std::fmt::write;

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum IpAddrType {
    V4(String),
    V6(String),
}
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {}
}
fn plus_1(x: &Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

enum Dice_Sides {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
}
impl fmt::Display for Dice_Sides{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let side  = match self {
            Dice_Sides::One => "One",
            Dice_Sides::Two => "Two",
            Dice_Sides::Three => "Three",
            Dice_Sides::Four => "Four",
            Dice_Sides::Five => "Five",
            Dice_Sides::Six => "Six",
            Dice_Sides::Seven => "Seven",
            Dice_Sides::Eight => "Eight",
            Dice_Sides::Nine => "Nine",
            Dice_Sides::Ten => "Ten",
        };
        write!(f, "Roll number: {}", side)
    }
}
fn dice_roll_action(die_face: &Dice_Sides) -> u8 {
    match die_face {
        Dice_Sides::Three => {
            add_fancy_hat();
            3
        }
        Dice_Sides::Seven => {
            remove_fancy_hat();
            7
        }
        other => move_player(other),
    }
}
/*This is already in std librarry so we do not need to redefine it
enum Option<T>{
    None,
    Some(T),
}*/
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("A lucky penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quater from {state:?}!");
            25
        }
    }
}
struct IpAddr {
    kind: IpAddrType,
    address: String,
}

fn main() {
    let four = IpAddrType::V4;
    let six = IpAddrType::V6;

    let home = IpAddrType::V4(String::from("127.0.0.1"));
    let loopback = IpAddrType::V6(String::from("::1"));
    let home = IpAddrKind::V4((127), (0), (0), (1));
    let loopback2 = IpAddrKind::V6(String::from("::1"));

    let exit = Message::Quit;
    let direction = Message::Move { x: (70), y: (60) };

    let some_variable = Some(5);
    let some_char = Some('c');
    let some_num = Some('c');
    let none_none: Option<i32> = None;

    let coin = Coin::Penny;
    let coin_value = value_in_cents(&coin);
    let coin2 = Coin::Quarter((UsState::Alaska));
    let coin2_value = value_in_cents(&coin2);
    println!("{coin2_value}");
    let mut count = 0;
    match &coin{
        Coin::Quarter(states) => println!("This is a quater from the {states:?} state"),
        _=> count += 1,
    }
    match &coin2{
        Coin::Quarter(states) => println!("This is a quater from the {states:?} state"),
        _=> count += 1,
    }
    if let Coin::Quarter(state) = &coin{
        println!("This is a quater from the {state:?} state");
    } 
    else{
        count += 1;
    }
    println!("{count}");

    let some_number = Some(5);
    let nothing: Option<i32> = None;
    let some_number = plus_1(&some_number);
    let nothing: Option<i32> = plus_1(&nothing);
    println!("{nothing:?}");
    println!("{some_number:?}");

    let three = Dice_Sides::Three;
    let seven = Dice_Sides::Seven;
    let three = dice_roll_action(&three);
    let seven = dice_roll_action(&seven);
    let eight = Dice_Sides::Eight;
     println!("{seven}");

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configrued to be {}", max),
        _ => (),
    }
    if let Some(max) = config_max{
        println!("The msximum is configured to be {max}");
    }

}
fn route(ipkind: IpAddrType) {}
fn add_fancy_hat() {
    println!("**Acquired Fancy Hat**")
}
fn remove_fancy_hat() {
    println!("**Lost Fancy Hat**")
}

fn move_player(die_face: &Dice_Sides) -> u8 {
    match die_face {
        Dice_Sides::One => 1,
        Dice_Sides::Two => 2,
        Dice_Sides::Four => 4,
        Dice_Sides::Five => 5,
        Dice_Sides::Six => 6,
        Dice_Sides::Eight => 8,
        Dice_Sides::Nine => 9,
        Dice_Sides::Ten => 10,
        _=>0,
    }
}
