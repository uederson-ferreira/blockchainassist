#!/bin/zsh

# 📂 Define caminhos
CHARACTERS_PATH="backend/eliza-starter/characters"
AGENT_FILE="backend/fala-eliza/.agent"

# 1. Lista todos os personagens disponíveis
echo "🧩 Personagens disponíveis:"
select PERSONAGEM_JSON in $CHARACTERS_PATH/*.json; do
  if [ -n "$PERSONAGEM_JSON" ]; then
    break
  else
    echo "❌ Escolha inválida. Tente novamente."
  fi
done

# 2. Extrai o nome do personagem (remove caminho e extensão)
PERSONAGEM=$(basename "$PERSONAGEM_JSON" .json)

# 3. Caminho relativo para passar ao pnpm start
PERSONAGEM2="characters/$PERSONAGEM.json"

# 4. Grava o personagem no .agent
echo "$PERSONAGEM" > "$AGENT_FILE"
echo "✅ Gravado personagem $PERSONAGEM no arquivo .agent"

# 5. Inicia o servidor da Eliza Starter
echo "🚀 Iniciando Eliza Starter..."
cd backend/eliza-starter
pnpm start --character "$PERSONAGEM2"