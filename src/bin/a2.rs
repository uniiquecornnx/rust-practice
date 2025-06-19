fn sum(a:i32,b:i32)->i32{
    a+b
}

fn display(z:i32){
    println!("{:?}",z);
}

fn main(){
    let z=sum(5,3);
    display(z);
}