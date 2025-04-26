# Pasta: envio_eliza
# Arquivo: enviar.py

import os
import requests

SERVER_PORT = 3000
AGENT_PATH = ".agent"  # 👈 só isso!

def carregar_agent_id():
    print("📂 Diretório atual (onde o enviar.py está rodando):")
    print(os.getcwd())

    print(f"🔎 Caminho absoluto esperado para o .agent: {os.path.abspath(AGENT_PATH)}")

    if not os.path.exists(AGENT_PATH):
        print("⚠️ Arquivo .agent não encontrado.")
        personagem = input("Digite o nome do personagem para usar: ").strip()
        with open(AGENT_PATH, "w", encoding="utf-8") as f:
            f.write(personagem)
        print(f"✅ Personagem '{personagem}' salvo no .agent.")

    with open(AGENT_PATH, "r", encoding="utf-8") as file:
        agent_id = file.read().strip()
        print(f"✅ Usando personagem: {agent_id}")
        return agent_id

def enviar_para_eliza(texto_usuario):
    agent_id = carregar_agent_id()
    try:
        url = f"http://localhost:{SERVER_PORT}/{agent_id}/message"
        print(f"🌐 Enviando requisição para: {url}")
        
        response = requests.post(
            url,
            headers={"Content-Type": "application/json"},
            json={"text": texto_usuario, "userId": "user", "userName": "User"},
            timeout=60
        )
        response.raise_for_status()
        data = response.json()

        if isinstance(data, list):
            resposta = " ".join([msg.get("text", "") for msg in data])
        else:
            resposta = data.get("text", "❓ Sem resposta da Eliza")

        print(f"🤖 Eliza respondeu: {resposta}")
        return resposta
    except Exception as e:
        print(f"❌ Erro ao enviar para Eliza: {e}")
        return None
