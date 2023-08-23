enum fatherstate{

    Gone,
    NotGone
}

fn main() {
    println!("Hello, world");
    println!("amog");
    let my_dad = fatherstate::Gone;

    match my_dad  {
        fatherstate::Gone => println!("no milk?"),
        _=>()
    }
}
