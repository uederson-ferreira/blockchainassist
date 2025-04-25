# wallet_service.py
# Serviço de integração com funções de carteira do projeto blockchainassist

from typing import Dict
import uuid

# Simula um dicionário de saldos por endereço (substituir por lógica real depois)
saldos_simulados: Dict[str, float] = {
    "0x123": 10.5,
    "0xabc": 25.0,
    "0xdef": 0.0
}

def consultar_saldo(address: str) -> float:
    """
    Retorna o saldo associado ao endereço informado.
    Caso não exista, retorna 0.0.
    """
    return saldos_simulados.get(address.lower(), 0.0)

def enviar_tokens(from_address: str, to_address: str, amount: float) -> Dict:
    """
    Simula o envio de tokens entre carteiras.
    Valida se há saldo suficiente e realiza a "transação".
    """
    from_addr = from_address.lower()
    to_addr = to_address.lower()

    if from_addr not in saldos_simulados:
        return {"error": "Endereço de origem não encontrado."}

    if saldos_simulados[from_addr] < amount:
        return {"error": "Saldo insuficiente."}

    # Deduz o saldo e transfere
    saldos_simulados[from_addr] -= amount
    saldos_simulados[to_addr] = saldos_simulados.get(to_addr, 0.0) + amount

    # Gera um hash simulado da transação
    tx_hash = str(uuid.uuid4())

    return {
        "status": "sucesso",
        "de": from_addr,
        "para": to_addr,
        "valor": amount,
        "tx_hash": tx_hash
    }
