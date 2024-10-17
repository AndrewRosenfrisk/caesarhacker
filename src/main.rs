use std::io::stdin;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let symbols = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let key_bound: isize = symbols.chars().count().try_into()?;

    println!("Enter the encrypted Casesar cipher message to hack.");

    let mut message: String = String::new();
    stdin().read_line(&mut message)?;

    symbols.char_indices().for_each(|symbol| {
        let mut translation = String::new();

        message.trim().to_uppercase().chars().for_each(|c| {
            if symbols.contains(&c.to_string()) {
                let i: isize = symbols
                    .find(c)
                    .unwrap()
                    .try_into()
                    .expect("Character outside bounds of symbol reference");

                let mut num = i - symbol.0 as isize;

                if num >= key_bound {
                    num -= key_bound
                } else if num < 0 {
                    num += key_bound
                }

                translation.push(
                    symbols
                        .chars()
                        .nth(
                            num.try_into().expect(
                                "Could not find character at nth position in symbol reference",
                            ),
                        )
                        .unwrap(),
                );
            } else {
                translation.push(c);
            }
        });
        println!("Key #{}: {}", symbol.0, translation);
    });
    Ok(())
}
