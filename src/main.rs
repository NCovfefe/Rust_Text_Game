use read_input::prelude::*;
use random_number::random;
use std::fs::File;
use std::io::prelude::*;
use std::time::Duration;
use std::thread::sleep; 

fn print_ascii(file_path: String){
    let mut file = File::open(file_path).unwrap();
    let mut art = String::new();
    file.read_to_string(&mut art).unwrap();
    println!("{}", art);
}
fn main(){
   let mut gold = 100;          
   let mut hero = Hero::init_hero();        //initialize a hero struct
   let time = Duration::from_secs(2);
   println!("You are the fiercest warrior known as Raheja the Centaur!");
   let file_path = String::from("ascii_art/centaur.txt");
   print_ascii(file_path);
   //sleep(time);
   println!("You are on a quest to find and defeat the great dragon Placidusax");
   //sleep(time);
   println!("First you decide to stop by the local trader");
   //sleep(time);
   println!("...");
   //sleep(time);
   println!("Hello Traveler!");
   //sleep(time);
   println!("Please take a look at what I have to offer...\n\n");
   //sleep(time);

   let mut choice = 0;
   while choice != 4 {
       println!("You currently have {} gold", gold);
       println!("(1) Greatsword(increase attack by 30) - 50 gold ");
       let file_path = String::from("ascii_art/sword.txt");
       print_ascii(file_path);

       println!("(2) Enchanted shield(increases defense by 20) -30 gold");
       let file_path = String::from("ascii_art/shield.txt");
       print_ascii(file_path);

       println!("(3) Dodge Potion(Increase dodge probability by 15%) -10 gold ");
       let file_path = String::from("ascii_art/potion.txt");
       print_ascii(file_path);
       
       println!("(4) Goodbye");

       choice = input::<u32>().get();
       
       if choice == 1 {
           gold -= 50;
           hero.attack_up(50);
       } else if choice == 2 {
           gold -= 30;
           hero.def_up(20);
       } else if choice == 3 {
           gold -= 10;
           hero.evade_up(0.15);
       } else if choice == 4 {
           choice = 4; 
           println!("Thank you traveler... I bid you good luck!");
       } 
       if(gold <= 10){
           choice = 4; 
       }
       println!("\n\n");
   }
   dungeon_entrance(&mut hero);
   
}
//    let mut sword = Weapon {
//        damage:20
//    };
//    let mut me = Hero::init_hero();
//    me.attack_up(sword.damage);
//    me.attack = 30;

fn dungeon_entrance(hero: &mut Hero) {
    let time = Duration::from_secs(2);
    println!("You crawl through the dungeon's opening and see two paths ahead of you");
    //sleep(time);
    println!("Do you choose the path to the left(1) or to the right(2)?");

    loop {
        let path_input = input::<u32>().get();

        if path_input == 1 {
            dungeon_path_left(&hero)
        }
        else if path_input == 2 {
            dungeon_path_right(&hero)
        }
        else {
            println!("Don't be scared! Adventure awaits!");
        }
    }
}

fn dungeon_path_left(mut hero: &Hero) {
    let time = Duration::from_secs(2);
    println!("You walk through a dark and damp tunnel and can hear what sounds like a vicous beast close ahead!");
    println!("You enter a vast cavern and see the mighty dragon Placidusax feasting on the bones of past adventurers!");
    //sleep(time);
    println!("Prepare for battle!\n");
    battle_sequence(&mut hero);
}


fn dungeon_path_right(hero: &Hero) {
    
}

fn battle_sequence(hero: &mut Hero) {
    let mut dragon = Boss::init_boss();
    let file_path = String::from("ascii_art/dragon.txt");
    print_ascii(file_path);
    //println!("{:#?}", hero);
    let mut choice:u32; 
    while hero.health > 0 || dragon.health > 0 {
        println!("Choose your move: \n (1)attack \n(2)block \n (3)evade");
        choice = input::<u32>().get();
        if choice == 1 {
            dragon.health -= hero.deal_damage(dragon.defense);
        } else if choice == 2 {
            //hero.
        }
    }

}
/*
attack = 100

enemy defense = 10

damage = attack - enemy.defense


*/
//Main character struct
#[derive(Debug)]
struct Hero {
    health: u32, //Damage able to take
    attack: u32, //Damage dealt
    defense: u32, //Damage absorption
    evade: f64, //Dodge chance
}

//Functions for the Hero struct
impl Hero {
    //Initialize Hero base stats
    fn init_hero() -> Hero {
        Hero {health: 100, attack: 100, defense: 50, evade: 0.5}
    }
    
    //Heal character
    fn heal(&mut self, amount: u32) -> u32 {
        self.health + amount
    }
    
    //Increase characters damage output
    fn attack_up(&mut self, damage: u32) {
        self.attack += damage;
    }

    //Increase characters defense
    fn def_up(&mut self, defense: u32) {
        self.defense += defense;
    }

    //Increase characters dodge chance
    fn evade_up(&mut self, evade: f64) {
        self.evade += evade;
    }

    //Deal damage to character
    fn take_damage(&mut self, loss: u32) {
        self.health -= loss;
    }
    
    //How much damage is dealt
    fn deal_damage(&mut self, defense: u32) -> u32 {
        self.attack - defense
    }

    //Chance to dodge an attack
    fn dodge_chance(&mut self) -> bool {
        let mut chance: f64 = random!();

        chance *= self.evade;
        if chance < 0.333 {
            false //Character hit
        }
        else {
            true //Character dodges attack
        }

    }
}

#[allow(dead_code)]
struct Weapon {
    damage: u32,
}

#[allow(dead_code)]
struct Item {
    
}

struct Boss {
    health: u32,
    attack: u32,
    defense: u32,
}

impl Boss {
    
    fn init_boss() -> Boss {
        Boss {health: 1000, attack: 50, defense: 150}
    }

    fn def_up(&mut self, defense: u32) {
        self.defense += defense;
    }

    fn deal_damage(&mut self, defense: u32){
        self.attack -= defense;
    }

    fn take_damage(&mut self, loss: u32) {
        self.health -= loss;
    } 
}

#[allow(dead_code)]
struct Enemy {
    health: u32,
    attack: u32,
    defense: u32,
}
#[allow(dead_code)]
impl Enemy {
    
    //Deal damage to enemy
    fn take_damage(&self, loss: u32) -> u32 {
        self.health - loss
    }
    
}
