
// debug - enable print debug
// clone & copy - ownership no longer passing however a copy (normally use for smaller in size)
#[derive(Debug, Clone, Copy)]
enum SkinColor {
    Dark,
    Darker,
    Pale,
}

impl SkinColor {
    fn print(&self) {
        match self {
            SkinColor::Dark => println!("{:?}", "Malay"),
            SkinColor::Darker => println!("{:?}", "Indian"),
            SkinColor::Pale => println!("{:?}", "Chinese"),
            //_ => println!("{:?}", "Alien"),
        }
    }
}

struct Human {
    name: String,
    height: i32,
    weight: i32,
    race: Race,
    skin_color: SkinColor,
}

// Structs always uses String instead of &str (because borrowed couldn't get disposed when struct's lifetime ends.)
struct Race {
    name: String,
}

impl Race {
    fn print_race(&self) {
        println!("{:?} Race", self.name);
    }
}

impl Human {
    fn new(name: String, height: i32, weight: i32, race: Race, skin_color: SkinColor) -> Self {
        Self {
            name,
            height,
            weight,
            race,
            skin_color,
        }
    }

    fn print(&self) {
        println!("{:?}", self.name);
        println!("{:?} Cm", self.height);
        println!("{:?} Kg", self.weight);
        self.race.print_race();
        self.skin_color.print();
    }
}

// String to be passed in function is always borrowed.
fn print_name(name: &str) {
    println!("{:?}", name);
}

fn main() {
    let adrian = Human::new(
        String::from("Adrian"),
        178,
        69,
        Race {
            name: String::from("Chinese"),
        },
        SkinColor::Pale,
    );
    let mohammed = Human::new(
        String::from("Mohammed"),
        178,
        69,
        Race {
            name: String::from("Malay"),
        },
        SkinColor::Dark,
    );

    // owned vector
    let human = vec![adrian, mohammed];
    for person in human {
        print_name(&person.name);
        person.print()
    }
}
