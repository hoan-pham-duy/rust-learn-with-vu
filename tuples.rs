fn main(){
    let tup = ("hello", 100_000);
    let (_string, _integer) = tup;
    println!("{}", _string);
    println!("{}", _integer);
}