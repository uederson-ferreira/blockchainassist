# main.py
# Arquivo principal da API FastAPI que integra o projeto blockchainassist com ElizaOS

from fastapi import FastAPI
from routes import saldo, enviar, carteiras
import uvicorn

# Instância do app FastAPI
app = FastAPI(title="Eliza Blockchain Backend")

# Incluindo os módulos de rota
app.include_router(saldo.router)
app.include_router(enviar.router)
app.include_router(carteiras.router)

# Rota raiz para verificação da API
@app.get("/")
def root():
    return {"message": "ElizaOS BlockchainAssist API ativa"}

# Inicializador do servidor local
if __name__ == "__main__":
    uvicorn.run("main:app", host="0.0.0.0", port=8000, reload=True)
