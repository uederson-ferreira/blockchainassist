# carteiras.py
# Rotas para listar e consultar carteiras armazenadas localmente em um arquivo JSON

from fastapi import APIRouter, HTTPException
import json
from pathlib import Path

router = APIRouter(prefix="/carteiras", tags=["Carteiras"])

# Caminho do arquivo onde as carteiras são armazenadas (simulado)
CARTEIRAS_JSON = Path("backend/data/carteiras.json")

@router.get("")
def listar_carteiras():
    """
    Lista todas as carteiras salvas no arquivo JSON.
    """
    if not CARTEIRAS_JSON.exists():
        return []
    with open(CARTEIRAS_JSON, "r") as f:
        return json.load(f)

@router.get("/{nome}")
def buscar_carteira(nome: str):
    """
    Busca uma carteira específica pelo nome.
    """
    if not CARTEIRAS_JSON.exists():
        raise HTTPException(status_code=404, detail="Arquivo de carteiras não encontrado.")

    with open(CARTEIRAS_JSON, "r") as f:
        carteiras = json.load(f)

    for c in carteiras:
        if c.get("nome") == nome:
            return c

    raise HTTPException(status_code=404, detail="Carteira não encontrada.")
