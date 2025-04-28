# Pasta: captura_fala
# Arquivo: ouvir.py

# Importa a biblioteca de reconhecimento de fala
import speech_recognition as sr

# Função que ouve o microfone e transcreve o que foi falado
def ouvir_microfone():
    # Cria um reconhecedor de fala
    r = sr.Recognizer()
    # Define o microfone como fonte de áudio
    mic = sr.Microphone()

    # Mensagem para o usuário saber que está sendo gravado
    print("🎙️ Gravando... Fale! (Diga 'encerre a gravação' para finalizar)")

    # Lista para armazenar os trechos de texto reconhecidos
    textos = []

    # Abre o microfone como fonte de áudio
    with mic as source:
        # Ajusta o reconhecedor para ignorar ruídos do ambiente
        r.adjust_for_ambient_noise(source)

        # Entra em um loop para ficar ouvindo continuamente
        while True:
            try:
                # Escuta o áudio do microfone até o usuário parar de falar naturalmente
                audio = r.listen(source, timeout=4)
                # Escuta o áudio do microfone (timeout de 3 segundos e limite de 5 segundos por frase)
                #audio = r.listen(source, timeout=3, phrase_time_limit=5)
                
                # Usa o serviço da Google para reconhecer o que foi falado
                texto = r.recognize_google(audio, language='pt-BR')
                print(f"📝 Você disse: {texto}")

                # Se o usuário disser "enviar para você", o programa entende que deve encerrar a gravação
                if "pode responder" in texto.lower():
                    print("✅ Comando de voz para encerrar detectado!")
                    break
                else:
                    # Se não for o comando de encerrar, adiciona o texto reconhecido à lista
                    textos.append(texto)

            except sr.WaitTimeoutError:
                # Se o usuário ficar em silêncio, apenas continua ouvindo (não dá erro)
                continue
            except sr.UnknownValueError:
                # Caso o áudio não seja compreendido, avisa o usuário e continua ouvindo
                print("❌ Não entendi o que você falou. (áudio incompreensível)")
                continue
            except sr.RequestError as e:
                # Se houver erro ao se conectar ao serviço de reconhecimento, exibe o erro e finaliza
                print(f"❌ Erro ao conectar ao serviço de reconhecimento: {e}")
                return None

    # Junta todos os trechos de texto falados em uma única string
    texto_final = " ".join(textos)

    # Exibe o texto final transcrito
    print("\n🧾 Texto final transcrito:")
    print(texto_final)

    # Retorna o texto final
    return texto_final