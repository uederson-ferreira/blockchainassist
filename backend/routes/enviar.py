# envio.py
# Modelo de dados para envio de tokens entre carteiras

from pydantic import BaseModel

class EnvioRequest(BaseModel):
    from_address: str
    to_address: str
    amount: float
