# import pyttsx3

# engine = pyttsx3.init()

# voices = engine.getProperty('voices')

# for index, voice in enumerate(voices):
#     print(f"Voz {index}: {voice.name} - {voice.id} - {voice.languages}")

# import pyttsx3

# # Inicializa o motor de fala
# engine = pyttsx3.init()

# # Pega todas as vozes disponíveis
# voices = engine.getProperty('voices')

# # Texto que será falado
# texto = "Olá, eu sou Anakin Brasileiro!"

# # Loop para cada voz
# for index, voice in enumerate(voices):
#     print(f"\n🔊 Testando voz {index}: {voice.name} - {voice.id} - Idiomas: {voice.languages}")

#     engine.setProperty('voice', voice.id)   # Define a voz atual
#     engine.setProperty('rate', 150)          # Define velocidade (pode ajustar se quiser)
#     engine.say(texto)
#     engine.runAndWait()                      # Fala o texto e espera terminar

import pyttsx3

# Inicializa o motor de fala
engine = pyttsx3.init()

# Pega todas as vozes disponíveis
voices = engine.getProperty('voices')

# Texto que será falado
texto = "Olá, eu sou Anakin Brasileiro!"

# Filtra vozes que são português do Brasil (pt-BR)
voices_ptbr = [voice for voice in voices if 'pt_BR' in str(voice.languages)]

if not voices_ptbr:
    print("⚠️ Nenhuma voz pt-BR encontrada. Verifique se há vozes brasileiras instaladas no sistema.")
else:
    for index, voice in enumerate(voices_ptbr):
        print(f"\n🔊 Testando voz {index}: {voice.name} - {voice.id} - Idiomas: {voice.languages}")

        engine.setProperty('voice', voice.id)   # Define a voz atual
        engine.setProperty('rate', 150)          # Define a velocidade
        engine.say(texto)
        engine.runAndWait()                      # Fala o texto e espera terminar
