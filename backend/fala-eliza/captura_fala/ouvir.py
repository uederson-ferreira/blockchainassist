# Pasta: captura_fala
# Arquivo: ouvir.py

import speech_recognition as sr

def ouvir_microfone():
    r = sr.Recognizer()
    mic = sr.Microphone()

    print("🎙️ Gravando... Fale! (Diga 'encerre a gravação' para finalizar)")

    textos = []

    with mic as source:
        r.adjust_for_ambient_noise(source)  # Ajusta para o ruído ambiente
        while True:
            try:
                audio = r.listen(source, timeout=3, phrase_time_limit=5)
                texto = r.recognize_google(audio, language='pt-BR')
                print(f"📝 Você disse: {texto}")

                if "encerre a gravação" in texto.lower():
                    print("✅ Comando de voz para encerrar detectado!")
                    break
                else:
                    textos.append(texto)

            except sr.WaitTimeoutError:
                # Se ficar em silêncio, continua ouvindo
                continue
            except sr.UnknownValueError:
                print("❌ Não entendi o que você falou. (áudio incompreensível)")
                continue
            except sr.RequestError as e:
                print(f"❌ Erro ao conectar ao serviço de reconhecimento: {e}")
                return None

    # Junta tudo o que foi falado antes do comando de encerrar
    texto_final = " ".join(textos)
    print("\n🧾 Texto final transcrito:")
    print(texto_final)
    return texto_final
