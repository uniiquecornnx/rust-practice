enum Flavor{
    Vanilla,
    Chocolate,
    Mango
}

struct Drink{
    flavor: Flavor,
    fruit_oz: f64

}

fn print_drink(drink: Drink){
    match drink.flavor{
        Flavor::Chocolate => println!("chocolate"),
        Flavor::Vanilla => println!("vanilla"),
        Flavor::Mango => println!("mango"),
    }
    println!("{:?}" , drink.fruit_oz);
}

fn main(){
    let drink = Drink{
        flavor: Flavor::Chocolate,
        fruit_oz: 12.0
    };
    print_drink(drink);
    let drink2 = Drink{
        flavor: Flavor::Vanilla,
        fruit_oz: 10.0
    };
   
    print_drink(drink3);
}