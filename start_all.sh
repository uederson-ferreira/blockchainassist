#!/bin/zsh

# 📂 Define caminhos
CHARACTERS_PATH="ia/eliza-starter/characters"
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

PERSONAGEM2="characters/$PERSONAGEM.json"
# 3. Grava o personagem no .agent
echo "$PERSONAGEM" > "$AGENT_FILE"
echo "✅ Gravado personagem $PERSONAGEM no arquivo .agent"

# 4. Inicia o servidor da Eliza Starter
echo "🚀 Iniciando Eliza Starter..."
cd ia/eliza-starter
pnpm start --character "$PERSONAGEM2"

# #!/bin/zsh

# # 📝 Define o personagem
# PERSONAGEM="Eliza"
# PERSONAGEM_JSON="eliza.character.json"

# # 📂 Define caminhos
# CHARACTERS_PATH="characters"
# AGENT_FILE="backend/fala-eliza/.agent"

# # 1. Grava o personagem no .agent
# echo "$PERSONAGEM" > "$AGENT_FILE"
# echo "✅ Gravado personagem $PERSONAGEM no arquivo .agent"

# # 2. Inicia o servidor da Eliza Starter no terminal atual
# echo "🚀 Iniciando Eliza Starter..."
# cd ia/eliza-starter
# pnpm start --character "$CHARACTERS_PATH/$PERSONAGEM_JSON"

