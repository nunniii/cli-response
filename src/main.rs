use std::process::{Command, Stdio};
use std::io::{self, Write}; // Importando Write

fn main() {
    // Caminho para a fonte mono9.tlf no seu projeto
    let font_path = "mono9.tlf";

    // Usar o comando figlet com largura de saída e encadear com lolcat
    let output = Command::new("figlet")
        .arg("-f")
        .arg(font_path)
        .arg("-w") // Adiciona a opção para definir a largura
        .arg("200") // Define a largura como 100 caracteres
        .arg("Hello, Nunniii ><")
        .stdout(Stdio::piped())
        .output()
        .expect("Falha ao executar figlet");

    let figlet_output = String::from_utf8_lossy(&output.stdout);

    // Passar a saída do figlet para o lolcat
    let mut lolcat = Command::new("lolcat")
        .stdin(Stdio::piped())
        .spawn()
        .expect("Falha ao iniciar lolcat");

    // Escrever a saída do figlet no stdin do lolcat
    lolcat.stdin.as_mut().expect("Falha ao obter stdin do lolcat")
        .write_all(figlet_output.as_bytes())
        .expect("Falha ao escrever no stdin do lolcat");

    lolcat.wait().expect("lolcat não conseguiu processar a saída");
}
