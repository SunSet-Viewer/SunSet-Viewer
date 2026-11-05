# -*- coding: utf-8 -*-
"""
Script to generate pt-BR.ftl (Brazilian Portuguese) for SunSet Viewer with 100% Fluent key parity.
"""
import os
import re

def main():
    repo_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    locales_dir = os.path.join(repo_root, "crates", "aurora-i18n", "locales")
    en_path = os.path.join(locales_dir, "en.ftl")
    pt_path = os.path.join(locales_dir, "pt-BR.ftl")

    with open(en_path, "r", encoding="utf-8") as f:
        en_content = f.read()

    # Dictionary of exact key-to-Portuguese translations
    # Prioritizing primary user flows: General UI, Login, Top Menu, Toolbar, Preferences,
    # Navigation, Inventory, Chat, Groups, Building, Diagnostics, Status, Dates, Numbers.
    TRANSLATIONS = {
        # Core & Identifiers
        "language-name": "Português (Brasil)",
        "number-decimal-separator": ",",
        "number-thousands-separator": ".",
        "date-day-month": "{ $day } de { $month }",
        "date-day-month-year": "{ $day } de { $month } de { $year }",
        "ui-chat-date-at": "{ $date } às { $time }",
        
        # Months (full and short)
        "extra-jan": "Janeiro",
        "extra-feb": "Fevereiro",
        "extra-mar": "Março",
        "extra-apr": "Abril",
        "extra-may": "Maio",
        "extra-jun": "Junho",
        "extra-jul": "Julho",
        "extra-aug": "Agosto",
        "extra-sep": "Setembro",
        "extra-oct": "Outubro",
        "extra-nov": "Novembro",
        "extra-dec": "Dezembro",

        # Exit & Session Dialogs
        "exit-title": "Fechar o SunSet Viewer?",
        "exit-world-body": "Deseja sair do visualizador ou retornar à tela de login?",
        "exit-login-body": "Deseja fechar o SunSet Viewer?",
        "exit-waiting": "Desconectando…",
        "exit-logout": "Desconectar",
        "exit-logout-hint": "Encerra a sessão e retorna à tela de login.",
        "exit-quit": "Sair",
        "exit-cancel": "Cancelar",

        # Login Screen
        "ui-login-news": "Notícias",
        "ui-login-aditi-beta": "Aditi (beta)",
        "ui-login-other-grid": "Outro grid…",
        "ui-login-grid-link-title": "Trocar grid de login?",
        "ui-login-grid-link-switch": "Trocar grid",
        "ui-login-username": "Nome de usuário",
        "ui-login-password": "Senha",
        "ui-login-remember-password": "Lembrar senha",
        "ui-login-start-location": "Local de início",
        "ui-login-location-home": "Casa (Home)",
        "ui-login-location-last": "Última localização",
        "ui-login-location-type": "Digitar região…",
        "ui-login-login": "Entrar",
        "ui-login-logging-in": "Entrando…",
        "ui-login-cancel": "Cancelar",
        "ui-login-authenticating": "Autenticando…",
        "ui-login-connecting": "Conectando ao simulador…",
        "ui-login-loading-world": "Carregando o mundo…",
        "ui-login-offline-demo": "Modo de demonstração offline",
        "ui-login-first-account": "Bem-vindo! Digite os dados da sua conta para entrar.",

        # Top Bar & Main Menus
        "ui-bars-aurora-menu": "Menu SunSet",
        "ui-bars-quit-aurora-viewer": "Sair do SunSet Viewer",
        "ui-bars-wiki": "Wiki do SunSet Viewer",
        "ui-bars-about": "Sobre o SunSet Viewer",
        "ui-bars-show-huds": "Exibir HUDs",
        "ui-bars-zoom-in": "Aumentar zoom",
        "ui-bars-zoom-out": "Diminuir zoom",
        "ui-bars-appearance": "Aparência…",
        "ui-bars-stop-flying": "Parar de voar",
        "ui-bars-fly-override": "Forçar voo",
        "ui-bars-fly-override-on": "Forçar voo: ativado",
        "ui-bars-fly-override-off": "Forçar voo: desativado",
        "ui-bars-sitting": "Sentado",
        "ui-bars-stand-up": "Levantar",
        "ui-bars-no-previous-destination": "Nenhum destino anterior",
        "ui-bars-no-next-destination": "Nenhum próximo destino",
        "ui-bars-tp-list-heading": "Nesta sessão",
        "ui-bars-tp-list-here": "Você está aqui",
        "ui-bars-tp-list-unknown": "Local desconhecido",

        # Common Actions & Labels
        "common-ok": "OK",
        "common-cancel": "Cancelar",
        "common-apply": "Aplicar",
        "common-close": "Fechar",
        "common-save": "Salvar",
        "common-save-as": "Salvar como…",
        "common-edit": "Editar",
        "common-delete": "Excluir",
        "common-remove": "Remover",
        "common-copy": "Copiar",
        "common-paste": "Colar",
        "common-cut": "Recortar",
        "common-refresh": "Atualizar",
        "common-reload": "Recarregar",
        "common-help": "Ajuda",
        "common-about": "Sobre",
        "common-quit": "Sair",
        "common-enable": "Ativar",
        "common-disable": "Desativar",
        "common-enabled": "Ativado",
        "common-disabled": "Desativado",
        "common-add": "Adicionar",
        "common-clear": "Limpar",
        "common-reset": "Redefinir",
        "common-default": "Padrão",
        "common-defaults": "Padrões",
        "common-search": "Buscar",
        "common-find": "Localizar",
        "common-back": "Voltar",
        "common-next": "Avançar",
        "common-open": "Abrir",
        "common-show": "Exibir",
        "common-hide": "Ocultar",
        "common-display": "Visualização",
        "common-teleport": "Teleportar",
        "common-fly": "Voar",
        "common-sit": "Sentar",
        "common-stand": "Levantar",
        "common-walk": "Andar",
        "common-run": "Correr",
        "common-mouselook": "Visão em primeira pessoa",
        "common-mouselook-m": "Visão em primeira pessoa",
        "common-avatar": "Avatar",
        "common-object": "Objeto",
        "common-objects": "Objetos",
        "common-prim": "Primitiva",
        "common-prims": "Primitivas",
        "common-mesh": "Malha (Mesh)",
        "common-texture": "Textura",
        "common-textures": "Texturas",
        "common-animation": "Animação",
        "common-animations": "Animações",
        "common-gesture": "Gesto",
        "common-gestures": "Gestos",
        "common-sound": "Som",
        "common-sounds": "Sons",
        "common-script": "Script",
        "common-scripts": "Scripts",
        "common-landmark": "Ponto de referência",
        "common-landmarks": "Pontos de referência",
        "common-notecard": "Notecard",
        "common-notecards": "Notecards",
        "common-land": "Terreno",
        "common-parcel": "Parcela",
        "common-region": "Região",
        "common-estate": "Propriedade",
        "common-grid": "Grid",
        "common-buy": "Comprar",
        "common-buy-object": "Comprar objeto",
        "common-pay": "Pagar",
        "common-touch": "Tocar",
        "common-take": "Pegar",
        "common-take-copy": "Pegar cópia",
        "common-wear": "Vestir",
        "common-attach": "Anexar",
        "common-detach": "Desanexar",
        "common-drop": "Soltar",
        "common-rename": "Renomear",
        "common-properties": "Propriedades",
        "common-options": "Opções",
        "common-preferences": "Preferências",
        "common-settings": "Configurações",
        "common-conversations": "Conversas",
        "common-friends": "Amigos",
        "common-people": "Pessoas",
        "common-groups": "Grupos",
        "common-inventory": "Inventário",
        "common-places": "Lugares",
        "common-appearance": "Aparência",
        "common-world-map": "Mapa-múndi",
        "common-mini-map": "Minimapa",
        "common-performance": "Desempenho",
        "common-favorites": "Favoritos",
        "common-about-land": "Sobre o terreno",
        "common-build": "Construir",
        "common-go-home": "Ir para Casa",
        "common-adult": "Adulto",
        "common-moderate": "Moderado",
        "common-general": "Geral",
        "common-mature": "Maduro",
        "common-away": "Ausente (AFK)",
        "common-busy": "Ocupado (Não perturbe)",
        "common-unnamed": "(sem nome)",
        "common-abandon-land": "Abandonar terreno",
        "common-show-boundaries": "Exibir limites",
        "common-show-in-search": "Exibir na busca",
        "common-show-transparency": "Exibir transparência",
        "common-show-on-map": "Exibir no mapa",
        "common-add-to-favorites": "Adicionar aos favoritos",
        "common-add-to-contact-set": "Adicionar aos contatos",
        "common-marketplace-listings": "Anúncios do Marketplace",

        # Preferences Tabs
        "ui-options-pages-general": "Geral",
        "ui-options-pages-graphics": "Gráficos",
        "ui-options-pages-audio": "Áudio",
        "ui-options-pages-chat": "Bate-papo",
        "ui-options-pages-controls": "Controles",
        "ui-options-pages-privacy": "Privacidade",
        "ui-options-pages-system": "Sistema",
        "ui-options-pages-about": "Sobre",
        "ui-options-pages-skin": "Tema e Aparência",
        "ui-options-pages-camera": "Câmera",
        "ui-options-pages-voice": "Voz",
        "ui-options-pages-network": "Rede e Cache",
        "ui-options-pages-notifications": "Notificações",
        "ui-options-pages-accessibility": "Acessibilidade",

        # Joystick and 3D Mouse
        "joystick-title": "Joystick e Mouse 3D",
        "joystick-enabled": "Ativar entrada de joystick",
        "joystick-enable-hint": "Movimente o avatar ou a câmera livre com um joystick, controle ou SpaceMouse.",
        "joystick-device": "Dispositivo de entrada",
        "joystick-first-device": "Primeiro dispositivo conectado",
        "joystick-disconnected": "Nenhum dispositivo selecionado conectado",
        "joystick-monitor": "Entradas em tempo real",
        "joystick-gamepad": "Controle / Gamepad",
        "joystick-spacemouse": "SpaceMouse",
        "joystick-avatar-enabled": "Controlar o avatar",
        "joystick-flycam-enabled": "Permitir câmera livre",
        "joystick-flycam": "Câmera livre com joystick",
        "joystick-flycam-start": "Iniciar câmera livre",
        "joystick-flycam-stop": "Parar câmera livre",
        "joystick-jump-button": "Botão Pular / Aterrissar",
        "joystick-rise-button": "Botão Subir",
        "joystick-fall-button": "Botão Descer / Agachar",
        "joystick-button-assign": "Atribuir…",
        "joystick-button-press": "Pressione um botão…",
        "joystick-button-value": "Botão { $button }",
        "joystick-tuning": "Suavização de movimento",
        "joystick-axes": "Mapeamento e sensibilidade dos eixos",
        "joystick-forward": "Frente / Trás",
        "joystick-slide": "Deslizar Esquerda / Direita",
        "joystick-up": "Cima / Baixo",
        "joystick-roll": "Rolagem (Roll)",
        "joystick-pitch": "Inclinação (Pitch)",
        "joystick-yaw": "Guinada (Yaw)",
        "joystick-zoom": "Zoom (eixo analógico)",

        # Camera & View
        "controls-camera-title": "Controles de Câmera",
        "controls-movement-title": "Controles de Movimento",
        "camera-front-view": "Vista frontal",
        "camera-side-view": "Vista lateral",
        "camera-rear-view": "Vista traseira",
        "camera-free": "Câmera livre",
        "camera-orbit": "Orbitar",
        "camera-pan": "Deslocar (Pan)",
        "camera-zoom": "Zoom",
        "camera-reset": "Redefinir câmera",

        # Crash & Diagnostics
        "crash-title": "SunSet Viewer — Relatório de Falha",
        "crash-heading": "O SunSet Viewer parou inesperadamente",
        "crash-report-summary": "Falha do SunSet Viewer",
        "crash-report-actual": "O SunSet Viewer parou de funcionar de forma inesperada.",
        "app-graphics-start-failed-title": "O SunSet Viewer não pôde iniciar",
        "app-gpu-stall-title": "SunSet Viewer: a placa de vídeo não está respondendo",
        "app-gpu-lost-title": "SunSet Viewer: a placa de vídeo foi desconectada/perdida",

        # Toolbar
        "ui-skin-speak": "Falar",
        "search-title": "Buscar",
        "photo-title": "Foto / Snapshot",
        "gesture-title": "Gestos",
        "texture-library-title": "Biblioteca de Texturas",
        "report-abuse-title": "Denunciar Abuso",
        "toolbar-command-profile": "Perfil",
        "extra-sit-on-ground-stand-up": "Sentar no chão / Levantar",
        "extra-fly-land-f": "Voar / Pousar",
    }

    # Common phrase substitutions for automatic contextual translation
    PHRASE_SUBS = [
        (r"\bDo you want to quit\b", "Deseja sair do"),
        (r"\bClose SunSet Viewer\b", "Fechar o SunSet Viewer"),
        (r"\bSave changes\b", "Salvar alterações"),
        (r"\bCancel\b", "Cancelar"),
        (r"\bApply\b", "Aplicar"),
        (r"\bClose\b", "Fechar"),
        (r"\bSelect all\b", "Selecionar tudo"),
        (r"\bAre you sure\b", "Tem certeza"),
        (r"\bPlease wait\b", "Por favor aguarde"),
        (r"\bLoading\b", "Carregando"),
        (r"\bConnecting\b", "Conectando"),
        (r"\bDisconnecting\b", "Desconectando"),
        (r"\bDownloading\b", "Baixando"),
        (r"\bUploading\b", "Enviando"),
        (r"\bNot available\b", "Não disponível"),
        (r"\bEnabled\b", "Ativado"),
        (r"\bDisabled\b", "Desativado"),
        (r"\bEnable\b", "Ativar"),
        (r"\bDisable\b", "Desativar"),
        (r"\bNone\b", "Nenhum"),
        (r"\bAll\b", "Todos"),
        (r"\bDefault\b", "Padrão"),
        (r"\bSettings\b", "Configurações"),
        (r"\bPreferences\b", "Preferências"),
        (r"\bInventory\b", "Inventário"),
        (r"\bFriends\b", "Amigos"),
        (r"\bGroups\b", "Grupos"),
        (r"\bConversations\b", "Conversas"),
        (r"\bLocal chat\b", "Bate-papo local"),
        (r"\bLandmarks\b", "Pontos de referência"),
        (r"\bTeleport\b", "Teleportar"),
        (r"\bAppearance\b", "Aparência"),
        (r"\bWorld map\b", "Mapa-múndi"),
        (r"\bMini-map\b", "Minimapa"),
        (r"\bMinimap\b", "Minimapa"),
        (r"\bPerformance\b", "Desempenho"),
        (r"\bGraphics\b", "Gráficos"),
        (r"\bResolution\b", "Resolução"),
        (r"\bFullscreen\b", "Tela cheia"),
        (r"\bRefresh rate\b", "Taxa de atualização"),
        (r"\bDraw distance\b", "Distância de visão"),
        (r"\bShadows\b", "Sombras"),
        (r"\bWater reflections\b", "Reflexos da água"),
        (r"\bVolume\b", "Volume"),
        (r"\bMute\b", "Silenciar"),
        (r"\bVoice chat\b", "Bate-papo por voz"),
        (r"\bMicrophone\b", "Microfone"),
        (r"\bOutput device\b", "Dispositivo de saída"),
        (r"\bInput device\b", "Dispositivo de entrada"),
        (r"\bPassword\b", "Senha"),
        (r"\bUsername\b", "Nome de usuário"),
        (r"\bRemember password\b", "Lembrar senha"),
    ]

    # Process en.ftl line by line
    pt_lines = []
    lines = en_content.splitlines()
    translated_count = 0
    total_keys = 0

    # Pattern for variables: {$var} or { $var }
    var_re = re.compile(r'\{\s*\$[a-zA-Z0-9_-]+\s*\}')

    for line in lines:
        if line.startswith("#") or not line.strip():
            pt_lines.append(line)
            continue

        # Check if line defines a key: key = value
        if not line.startswith(" ") and "=" in line:
            parts = line.split("=", 1)
            key = parts[0].strip()
            val = parts[1]
            total_keys += 1

            if key in TRANSLATIONS:
                pt_lines.append(f"{key} = {TRANSLATIONS[key]}")
                translated_count += 1
            else:
                # Apply contextual substitutions while strictly preserving all variables
                new_val = val
                for pat, rep in PHRASE_SUBS:
                    new_val = re.sub(pat, rep, new_val, flags=re.IGNORECASE)

                # Verify variables did not change
                orig_vars = var_re.findall(val)
                new_vars = var_re.findall(new_val)
                if orig_vars == new_vars:
                    pt_lines.append(f"{key} ={new_val}")
                else:
                    # Fallback to original value if variable mismatch occurred
                    pt_lines.append(line)
        else:
            pt_lines.append(line)

    with open(pt_path, "w", encoding="utf-8") as f:
        f.write("\n".join(pt_lines) + "\n")

    print(f"Generated {pt_path} with {total_keys} keys (direct curated: {len(TRANSLATIONS)}, total: {len(pt_lines)} lines)")

if __name__ == "__main__":
    main()
