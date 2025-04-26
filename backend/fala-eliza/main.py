# Raiz do projeto
# Arquivo: main.py

import os
from captura_fala.ouvir import ouvir_microfone
from envio_eliza.enviar import enviar_para_eliza
from resposta_fala.falar import falar_texto

def main():
    #print("📂 Diretório atual (onde o main.py está rodando):")
    #print(os.getcwd())

    #agent_path = ".agent"  # 👈 só isso!
    #print(f"🔎 Caminho esperado do arquivo .agent: {os.path.abspath(agent_path)}")

    texto_usuario = ouvir_microfone()
    if texto_usuario:
        resposta_eliza = enviar_para_eliza(texto_usuario)
        if resposta_eliza:
            falar_texto(resposta_eliza)

if __name__ == "__main__":
    main()
