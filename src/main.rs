fn main() {
    println!("Por favor indica tu pais: ");

    let mut pais: String = String::new();
    std::io::stdin().read_line(&mut pais).unwrap();

    println!("Por favor introduce tu edad");
    let mut edad: String = String::new();
    std::io::stdin().read_line(&mut edad).unwrap();

    let edad_int: u8 = edad.trim().parse().unwrap();

    println!(
        "Hola, bienvenido usuario de {} de {} años de edad",
        pais, edad_int
    );
}
