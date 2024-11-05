use std::process::{Command, Stdio};
use std::io::{self, Write};

fn main() {
    // Caminho absoluto para a fonte mono9.tlf
    let font_path = "mono9.tlf";

    // Executar o comando figlet com a fonte e largura desejadas, e capturar a saída
    let output = Command::new("figlet")
        .arg("-f")
        .arg(font_path)
        .arg("-w")
        .arg("200") // Define a largura como 200 caracteres
        .arg("Hello, Nunniii ><")
        .stdout(Stdio::piped())
        .output()
        .expect("Falha ao executar figlet");

    // Converter a saída do figlet para string
    let figlet_output = String::from_utf8_lossy(&output.stdout);

    // Passar a saída do figlet para o lolcat
    let mut lolcat = Command::new("lolcat")
        .stdin(Stdio::piped())
        .spawn()
        .expect("Falha ao iniciar lolcat");

    // Escrever a saída do figlet no stdin do lolcat
    lolcat.stdin.as_mut()
        .expect("Falha ao obter stdin do lolcat")
        .write_all(figlet_output.as_bytes())
        .expect("Falha ao escrever no stdin do lolcat");

    // Esperar o lolcat terminar
    lolcat.wait().expect("lolcat não conseguiu processar a saída");
}
