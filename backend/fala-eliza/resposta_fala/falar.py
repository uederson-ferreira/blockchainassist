# Pasta: resposta_fala
# Arquivo: falar.py

#E para sintetizar texto em fala com pyttsx3:
import pyttsx3

def falar_texto(texto):
    # Inicializa o motor de fala
    engine = pyttsx3.init()

    # Define a taxa de fala (opcional)
    engine.setProperty('rate', 150)  # Palavras por minuto

    # Define a voz (opcional, pode variar dependendo do sistema)
    voices = engine.getProperty('voices')
    #engine.setProperty('voice', voices[0].id)  # Voz masculina
    #engine.setProperty('voice', voices[1].id)  # Voz feminina

    # Fala o texto
    engine.say(texto)
    engine.runAndWait()