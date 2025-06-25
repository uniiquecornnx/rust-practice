enum Color{
    Red,
    Yellow,
    Blue
}

fn print_color(my_color: Color){
    
    match my_color{
        Color::Red=>println!("Red"),
        Color::Yellow=>println!("Yellow"),
        Color::Blue=>println!("Blue")
    }
}

fn main(){
    let my_color=Color::Yellow;
    print_color(my_color);
}