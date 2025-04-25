import { useState } from "react";
import { Wand2, Wallet, Send } from "lucide-react";

export default function ElizaPanel() {
  const [address, setAddress] = useState("");
  const [command, setCommand] = useState("");

  return (
    <div className="min-h-screen flex items-center justify-center bg-gradient-to-tr from-indigo-100 via-purple-100 to-white px-4 py-8">
      <div className="w-full max-w-xl space-y-8 bg-white/80 rounded-2xl shadow-xl p-8">
        <h1 className="text-3xl md:text-4xl font-bold text-gray-900 text-center">
          Painel ElizaOS + Token ERC20
        </h1>

        {/* Mint Público */}
        <div className="space-y-2">
          <div className="flex items-center gap-2 text-lg font-semibold text-indigo-700">
            <Wand2 size={20} /> Mint Público
          </div>
          <button className="bg-indigo-600 hover:bg-indigo-700 text-white px-4 py-2 rounded-lg shadow">
            Mintar 10 Tokens Grátis
          </button>
        </div>

        {/* Consultar Saldo */}
        <div className="space-y-2">
          <div className="flex items-center gap-2 text-lg font-semibold text-indigo-700">
            <Wallet size={20} /> Consultar Saldo
          </div>
          <div className="flex gap-2">
            <input
              type="text"
              placeholder="Digite o endereço"
              value={address}
              onChange={(e) => setAddress(e.target.value)}
              className="flex-1 px-3 py-2 rounded border border-gray-300"
            />
            <button className="bg-indigo-500 hover:bg-indigo-600 text-white px-4 py-2 rounded shadow">
              Ver Saldo
            </button>
          </div>
        </div>

        {/* ElizaOS Command */}
        <div className="space-y-2">
          <div className="flex items-center gap-2 text-lg font-semibold text-indigo-700">
            <Send size={20} /> ElizaOS
          </div>
          <div className="flex gap-2">
            <input
              type="text"
              placeholder='Ex: "Envie 5 tokens para erd1..."'
              value={command}
              onChange={(e) => setCommand(e.target.value)}
              className="flex-1 px-3 py-2 rounded border border-gray-300"
            />
            <button className="bg-indigo-500 hover:bg-indigo-600 text-white px-4 py-2 rounded shadow">
              Enviar para Eliza
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}