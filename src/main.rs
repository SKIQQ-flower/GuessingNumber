
use colored::Colorize;
use rand::Rng;
use dialoguer::{theme::ColorfulTheme, Input, Select};

fn main() {
    println!("{}", "Adivinhe um número!".green().bold().underline());
    let options = vec!["Maricas (intervalo de 5 números)".green(), "Neurotípico (intervalo de 10 números)".yellow(), "MULEQUE DOIDO VIDA LOKA (intervalo de 30 numeros)".red()];
    let difficulty = Select::new()
        .with_prompt("Escole a dificuldade logo, porra")
        .items(&options)
        .interact()
        .unwrap();

    let mut rng = rand::thread_rng();
    let mut number_to_be_guessed: i32 = 0;
    let mut range: i32 = 0;
    let mut tentativas: i32 = 1;
    match difficulty {
        0 => { number_to_be_guessed = rng.gen_range(1..=5); range = 5},
        1 => { number_to_be_guessed = rng.gen_range(1..=10); range = 10},
        2 => { number_to_be_guessed = rng.gen_range(1..=30); range = 30 },
        _ => println!("Opção inválida"),
    }

    println!("{} {}", "Advinhe o número entre 1 e".yellow(), range.to_string().yellow());
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Resposta")
        .validate_with({
            |input: &String| -> Result<(), &str> {
                if input == &number_to_be_guessed.to_string() {
                    Ok(())
                } else {
                    tentativas += 1;
                    Err("Errou!")
                }
            }
        })
        .interact_text()
        .unwrap();

    println!("Parabéns seu lixo humano, você acertou com {tentativas} tentativa{}", if tentativas > 1 { "s" } else { "" });
    
}