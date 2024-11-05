#!/bin/bash
# install_cli_response.sh

echo "Clonando o repositório..."
git clone https://github.com/nunniii/cli-response.git
cd cli-response || { echo "Falha ao acessar o diretório do projeto"; exit 1; }

echo "Compilando o projeto em modo release..."
cargo build --release

if [ ! -f "target/release/cli-response" ]; then
    echo "Falha ao compilar o projeto. Saindo..."
    exit 1
fi

echo "Movendo o binário para o diretório /data/data/com.termux/files/usr/bin..."
mv target/release/cli-response /data/data/com.termux/files/usr/bin/cli-response

echo "Configurando diretório de fontes em ~/.cli-response/fonts..."
mkdir -p ~/.cli-response/fonts
cp -r fonts/* ~/.cli-response/fonts/

if [[ ":$PATH:" != *":/data/data/com.termux/files/usr/bin:"* ]]; then
    echo 'export PATH=$PATH:/data/data/com.termux/files/usr/bin' >> ~/.bashrc
    source ~/.bashrc
fi

echo "Instalação concluída! 🎉"
