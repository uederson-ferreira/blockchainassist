# saldo.py
# Rota para consulta de saldo de carteiras

from fastapi import APIRouter
from services.wallet_service import consultar_saldo

router = APIRouter(prefix="/saldo", tags=["Saldo"])

@router.get("/{address}")
def get_saldo(address: str):
    """
    Consulta o saldo de uma carteira informada pelo endereço.
    """
    saldo = consultar_saldo(address)
    return {"address": address, "saldo": saldo}