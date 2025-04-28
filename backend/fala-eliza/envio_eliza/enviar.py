# Pasta: envio_eliza
# Arquivo: enviar.py

# Importa módulos para manipular o sistema de arquivos e enviar requisições HTTP
import os
import requests

# Define a porta onde o servidor local está rodando
SERVER_PORT = 3000

# Define o nome do arquivo que guarda o ID do personagem
AGENT_PATH = ".agent"  # 👈 Arquivo onde será salvo o nome do personagem usado para interagir com a Eliza

# Função para carregar o ID (nome) do agente/personagem
def carregar_agent_id():
    # Mostra o diretório atual para ajudar na depuração
    print("📂 Diretório atual (onde o enviar.py está rodando):")
    print(os.getcwd())
    print(f"🔎 Caminho absoluto esperado para o .agent: {os.path.abspath(AGENT_PATH)}")

    # Se o arquivo .agent não existir, solicita ao usuário o nome do personagem
    if not os.path.exists(AGENT_PATH):
        print("⚠️ Arquivo .agent não encontrado.")
        personagem = input("Digite o nome do personagem para usar: ").strip()

        # Salva o nome do personagem no arquivo .agent
        with open(AGENT_PATH, "w", encoding="utf-8") as f:
            f.write(personagem)
        print(f"✅ Personagem '{personagem}' salvo no .agent.")

    # Lê o nome do personagem já salvo
    with open(AGENT_PATH, "r", encoding="utf-8") as file:
        agent_id = file.read().strip()

    # Mostra o personagem que será usado
    print(f"✅ Usando personagem: {agent_id}")
    return agent_id

# Função para enviar uma mensagem do usuário para a Eliza
def enviar_para_eliza(texto_usuario):
    # Carrega o ID do personagem (nome)
    agent_id = carregar_agent_id()

    try:
        # Monta a URL da requisição com base no agent_id
        url = f"http://localhost:{SERVER_PORT}/{agent_id}/message"
        print(f"🌐 Enviando requisição para: {url}")

        # Envia a mensagem para o servidor local via POST
        response = requests.post(
            url,
            headers={"Content-Type": "application/json"},
            json={"text": texto_usuario, "userId": "user", "userName": "User"},
            timeout=60  # Limite de tempo de 60 segundos para resposta
        )

        # Verifica se houve algum erro HTTP
        response.raise_for_status()

        # Converte a resposta para JSON
        data = response.json()

        # Se a resposta for uma lista de mensagens, junta todas em uma única string
        if isinstance(data, list):
            resposta = " ".join([msg.get("text", "") for msg in data])
        else:
            # Caso contrário, tenta extrair o texto diretamente
            resposta = data.get("text", "❓ Sem resposta da Eliza")

        # Mostra a resposta recebida
        print(f"🤖 Eliza respondeu: {resposta}")
        return resposta

    except Exception as e:
        # Em caso de erro, mostra a mensagem de erro e retorna None
        print(f"❌ Erro ao enviar para Eliza: {e}")
        return None
