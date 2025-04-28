import speech_recognition as sr

r = sr.Recognizer()
mic = sr.Microphone()

print("Gravando... Fale! (Diga 'encerre a gravação' para parar)")

textos = []

with mic as source:
    r.adjust_for_ambient_noise(source)  # Ajusta para ruído ambiente
    while True:
        try:
            audio = r.listen(source, timeout=1, phrase_time_limit=5)
            texto = r.recognize_google(audio, language='pt-BR')
            print(f"Você disse: {texto}")

            if "gravação finalizada" in texto.lower():
                print("Comando de voz para encerrar detectado!")
                break
            else:
                textos.append(texto)

        except sr.WaitTimeoutError:
            pass  # Se não escutar nada, continua ouvindo
        except sr.UnknownValueError:
            print("Não entendi o que você falou.")
        except sr.RequestError as e:
            print(f"Erro ao se conectar ao serviço de reconhecimento: {e}")
            break

# Junta tudo em um único texto final
texto_final = " ".join(textos)
print("\nTexto final transcrito:")
print(texto_final)