use read_input::prelude::*;
use random_number::random;
use std::fs::File;
use std::io::prelude::*;
use std::time::Duration;
use std::thread::sleep;


static TIME: Duration = Duration::from_secs(1);


//Takes file path and prints file
fn print_ascii(file_path: String) {
=======
/*
CS 4080
Professor Raheja
5/14/22
Nate Clarke & Garrett Adams
*/

//takes file path and prints file
fn print_ascii(file_path: String){
  
    let mut file = File::open(file_path).unwrap();
    let mut art = String::new();
    file.read_to_string(&mut art).unwrap();
    println!("{}", art);
}

fn main() {

    let mut player = game_setup();
    let mut gold = 100;                      //begin with 100 gold  
    //let mut hero = Hero::init_hero();        //initialize a hero struct
    println!("You are the fiercest warrior known as Raheja the Centaur!");
    let file_path = String::from("ascii_art/centaur.txt");
    print_ascii(file_path);
    sleep(TIME);
    println!("You are on a quest to find and defeat the great dragon Placidusax");
    sleep(TIME);
    println!("First you decide to stop by the local trader");
    sleep(TIME);
    println!("...");
    sleep(TIME);
    println!("Hello Traveler!");
    sleep(TIME);
    println!("Please take a look at what I have to offer...\n\n");
    sleep(TIME);

    let mut choice = 0;
    while choice != 4 {
        println!("You currently have {} gold", gold);
        println!("(1) Greatsword(increase attack by 50) - 50 gold ");
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
        if gold < 10 {
            choice = 4; 
            println!("Thank you traveler... I bid you good luck!");
        }
        println!("\n\n");
    }
    dungeon_entrance(&mut hero);
   
}

//Game introduction, user enters their name and chooses their starting class
fn game_setup() -> Hero {

    println!("Greetings, adventurer...\nWelcome to the realm of (insert cool name here)!");
    println!("Before we begin your journey, let's get your name and your class.");
    let name = input::<String>().get();                         //Take user input and store into name variable
    println!("Hmmm, {}, not the name I would have chosen but whatever makes you happy!", name);
    sleep(TIME);
    println!("Now your class type.\n(1) Warrior\n(2) Archer\n(3) Mage");
    sleep(TIME);
    println!("The warrior is strongest in physical combat but weakest to magic.\nThe Archer is agile and strong with bows but weak against physical damage.\nThe mage is equipped with strong spells and incantations but is weak in a close combat fight.");

    //Take input and initialize Hero struct for the class they choose
    let mut class_choice = input::<u32>().get();
    let mut player;
    assert!(class_choice == 1 | 2 | 3);             //This should make it so only 1 2 or 3 are picked, not tested yet

    if class_choice == 1 {
        player = Hero::init_hero_warrior();
        player.name = name;
    }
    else if class_choice == 2 {
        player = Hero::init_hero_archer();
        player.name = name;
    }
    else if class_choice == 3 {
        player = Hero::init_hero_mage();
        player.name = name;
    }

    println!("Here are your starting stats:\n{:#?}", player);
    println!("Get ready {}, your quest begins!", player.name);
    player          //Return player::Hero struct into main()
}

fn dungeon_entrance(hero: &mut Hero) {
    println!("You crawl through the dungeon's opening and see two paths ahead of you");
    sleep(TIME);
    println!("Do you choose the path to the left(1) or to the right(2)?");

    loop {
        let path_input = input::<u32>().get();

        if path_input == 1 {
            dungeon_path_left(hero);
            break;
        }
        else if path_input == 2 {
            //dungeon_path_right(hero);
            break;
        }
        else {
            println!("Don't be scared! Adventure awaits!");
        }
    }
}

fn dungeon_path_left(hero: &mut Hero) {
    //let time = Duration::from_secs(1);
    println!("You walk through a dark and damp tunnel and can hear what sounds like a vicous beast close ahead!");
    println!("You enter a vast cavern and see the mighty dragon Placidusax feasting on the bones of past adventurers!");
    sleep(TIME);
    println!("Prepare for battle!\n");
    battle_sequence(hero);
}


// fn dungeon_path_right(hero: &Hero) {
    
// }

fn battle_sequence(hero: &mut Hero) {
    //let time = Duration::from_secs(1);
    let mut dragon = Boss::init_boss();
    let file_path = String::from("ascii_art/dragon.txt");
    print_ascii(file_path);
    
    let mut choice:u32; 
    while hero.health > 0 || dragon.health > 0 {
    
        println!("Choose your move: \n(1)attack \n(2)block");
        let mut boost = 0; 
        //Player attack phase
        choice = input::<u32>().get();
        if choice == 1 {
            let damage_done = hero.deal_damage(dragon.defense); //damage_done won't be mut because for now it will do the same amount everytime
            dragon.health -= damage_done;
            println!("You swing your greatsword at Placidusax, dealing {} damage!", damage_done);
            if dragon.health <= 0 {
                println!("Placidusax lets out one final roar before he collapses to the ground");
                sleep(TIME);
                break;
            }
        } 
        else if choice == 2 {
            boost = 10; 
            hero.def_up(boost);
            println!("You raise your shield for the incoming attack, temporarily boosting your defense by 10 points giving you {} defense!", hero.defense);
        }
        
        //Dragon attack phase
        if dragon.health >= 400 && dragon.health <= 525{
            println!("Placidusax rears back its head and breathes a blast of fire towards you!");
            sleep(TIME);
            if hero.dodge_chance() == true {
                //if true player dodges
                println!("Your quick reflexes are no match for the dragon and you quickly dash behind him to avoid the fire!");
            }
            else {
                //if false player takes damage
                let fire_damage = dragon.fire_breath(hero.defense);
                hero.health -= fire_damage;
                println!("The fire scorches you, dealing {} damage!", fire_damage);
            }
            if hero.health < 0 {
                hero.health = 0;
                break;
            }
        }
        else if dragon.health >= 100 && dragon.health < 400 {
            println!("The dragon swipes its ferocious claws at you!");
            sleep(TIME);
            if hero.dodge_chance() == true {
                println!("Right before he strikes, you leap over his claws!");
            }
            else {
                let claw_damage = dragon.claw_attack(hero.defense);
                hero.health -= claw_damage;
                println!("Placidusax' claws pierce your armor, dealing {} damage!", claw_damage);
            }
            if hero.health < 0 {
                hero.health = 0;
                break;
            }
        }
        else if dragon.health < 100 {
            println!("As the dragon draws closer to death it lashes out in a violent rage grabbing you and launching you into the air!");
            sleep(TIME);
            let fall_damage = dragon.grab_attack(hero.defense);
            hero.health -= fall_damage;
            println!("You hurtle towards the floor, suffering {} points of damage!", fall_damage);
            if hero.health < 0 {
                hero.health = 0;
                break;
            }
        }
        else if dragon.health <= 0 {
            println!("Placidusax lets out one final roar before he collapses to the ground");
            sleep(TIME);
            break;
        }

        //Display boss and hero stats at end of each phase
        sleep(TIME);
        hero.remove_boost(boost);
        
        println!("\n{:#?} \n {:#?}", hero, dragon);
    }

    if dragon.health <= 0 {
        println!("Congratulations! You have slayed the dragon!");
    }

    if hero.health <= 0 {
        println!("Oh dear, you are dead!");
        let file_path = String::from("ascii_art/death.txt");
        print_ascii(file_path);
    }

}


//Main character struct
#[derive(Debug)]
struct Hero {
    name: String, //It's your name
    health: i32, //Starting health
    melee_attack: i32, //Physical damage strength
    range_attack: i32, //Ranged damage strength
    magic_attack: i32, //Magical damage strength
    physical_defense: i32, //Physical damage absorption
    magical_defense: i32, //Magical damage absorption
    evade: f64, //Dodge chance
}

//Functions for the Hero struct
impl Hero {
    //Initialize Hero base stats (Going to change these depending on class type selected)
    // fn init_hero() -> Self {
    //     Self {health: 100, attack: 100, defense: 50, evade: 0.5}
    // }

    
    //Initialize Warrior Class
    fn init_hero_warrior() -> Self {
        Self {name: "".to_string(), health: 150, melee_attack: 100, range_attack: 35, magic_attack: 20, physical_defense: 50, magical_defense: 15, evade: 0.25}
    }

    //Initialize Archer Class
    fn init_hero_archer() -> Self {
        Self {name: "".to_string(), health: 100, melee_attack: 60, range_attack: 100, magic_attack: 25, physical_defense: 20, magical_defense: 45, evade: 0.6}
    }

    //Initialize Mage Class
    fn init_hero_mage() -> Self {
        Self {name: "".to_string(), health: 90, melee_attack: 30, range_attack: 40, magic_attack: 100, physical_defense: 20, magical_defense: 60, evade: 0.4}
    }
    

    //Character attribute increments/decrements
    
    //Heal character
    fn heal(&mut self, heal_amount: i32) {
        self.health += heal_amount;
    }
    
    //Increase characters physical damage output
    fn phys_att_up(&mut self, phys_up: i32) {
        self.melee_attack += phys_up;
    }

    //Increase characters ranged damage output
    fn range_att_up(&mut self, range_up: i32) {
        self.range_attack += range_up;
    }

    //Increase characters magical damage output
    fn magic_att_up(&mut self, magic_up: i32) {
        self.magic_attack += magic_up;
    }

    //Increase characters physical defense
    fn phys_def_up(&mut self, def_increase: i32) {
        self.physical_defense += def_increase;
    }

    //Increase characters magical defense
    fn mag_def_up(&mut self, def_increase: i32) {
        self.magical_defense += def_increase;
    }

    //Increase characters dodge chance
    fn evade_up(&mut self, evade_amt: f64) {
        self.evade += evade_amt;
    }

    //Reset characters physical attack to default
    fn phys_att_down(&mut self, boost: i32) {
        self.melee_attack;
    }

    // //Reset characters ranged attack to default
    // fn range_att_up(&mut self, range_up: i32) {
    //     self.range_attack += range_up;
    // }

    // //Increase characters magical damage output
    // fn magic_att_up(&mut self, magic_up: i32) {
    //     self.magic_attack += magic_up;
    // }

    //Remove physical defense boost
    fn remove_phys_boost(&mut self, boost: i32) {
        self.physical_defense -= boost;
    }

    //Remove magical defense boost
    fn remove_mag_boost(&mut self, boost: i32) {
        self.magical_defense -= boost;
    }

    

    //Deal damage to character (Not needed I don't think because just use damage functions to subtract from character's health)
    // fn take_damage(&mut self, loss: i32) -> i32 {
    //     self.health - loss
    // }
    
    //Subtract the target's physical defense, divided by 2, from the character's physical damage output and return the amount of damage dealt
    fn deal_phys_damage(&mut self, defense: i32) -> i32 {
        self.melee_attack - defense/2
    }

    //Subtract the target's physical defense, divided by 2, from the character's ranged damage output and return the amount of damage dealt
    fn deal_range_damage(&mut self, defense: i32) -> i32 {
        self.range_attack - defense/2
    }

    //Subtract the target's magical defense, divded by 2, from the character's magical damage output and return the amount of damage dealt
    fn deal_mag_damage(&mut self, defense: i32) -> i32 {
        self.magic_attack - defense/2
    }

    //Chance to dodge an attack
    fn dodge_chance(&mut self) -> bool {
        let rng: f64 = random!(); //Create a random float between 0.0 and 1.0
        let mut chance = (rng * 100.0).round() / 100.0; //Round the rng before putting moving it to chance variable

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

//Struct for bosses. Separate implementations for unique boss encounters
#[derive(Debug)]
struct Boss {
    health: i32, //Starting health
    melee_attack: i32, //Physical damage strength
    range_attack: i32, //Ranged damage strength
    magic_attack: i32, //Magical damage strength
    physical_defense: i32, //Physical damage absorption
    magical_defense: i32, //Magical damage absorption
}

//Functions for the Dragon Boss
impl Boss {
    
    fn init_boss() -> Self {
        Self {health: 525, melee_attack: 50, range_attack: 0, magic_attack: 30, physical_defense: 150, magical_defense: 150}
    }

    // fn def_up(&mut self, defense: i32) {
    //     self.defense += defense;
    // }

    fn fire_breath(&mut self, defense: i32) -> i32 {
        (defense - self.attack) + 25
    }

    fn claw_attack(&mut self, defense: i32) -> i32 {
        self.attack - defense/2
    }

    fn grab_attack(&mut self, defense: i32) -> i32 {
        self.attack - defense/4
    }
}

impl Boss {
    
}

//Struct for enemies. Separate implementations for different enemy types (Hopefully)
#[allow(dead_code)]
struct Enemy {
    health: i32, //Starting health
    melee_attack: i32, //Physical damage strength
    range_attack: i32, //Ranged damage strength (Applies physical)
    magic_attack: i32, //Magical damage strength
    physical_defense: i32, //Physical damage absorption
    magical_defense: i32, //Magical damage absorption
}
#[allow(dead_code)]
impl Enemy {
    
    //Deal damage to enemy
    fn take_damage(&self, loss: i32) -> i32 {
        self.health - loss
    }
    
}
