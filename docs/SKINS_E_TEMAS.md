# Skins e Conjuntos de Cores — SunSet Viewer

O **SunSet Viewer** possui um sistema completo de **Skins e Conjuntos de Cores** com **troca instantânea em tempo real** (sem precisar reiniciar o visualizador). A escolha altera de forma coesa janelas, barras superior e inferior, tela de login (`aurora_bg`), menus, botões, lista de Amigos/Pessoas, crachás flutuantes 3D (`nameplate`), mensagens de chat e a cor dos **textos digitados** em todos os campos de entrada.

Quem nunca alterar a skin mantém o visual original do **`Aurora (Padrão)`** (`Theme::default()`) preservado pixel a pixel.

---

## 1. Como Escolher uma Skin ou Tema

1. Pressione `Ctrl + P` para abrir as **Preferências** e acesse **Aparência › Skin** (ou abra **Aparência › Cores** e clique no atalho direto **Skin** no topo da página).
2. Escolha de duas formas:
   - **Chips Rápidos de 1 Clique**: Logo abaixo de **Conjunto de cores**, clique diretamente em qualquer um dos botões com prévia das 3 cores (**Tema SunSet (Pôr do Sol)**, **Rosa**, **Rosa Pastel (Claro)**, **Violeta**, **Roxo**, **Lilás**, **Lilás Claro** ou **Aurora (Padrão)**).
   - **Seletores Completos (`Skin` e `Conjunto de cores`)**: Selecione a família visual desejada (ex.: `Aurora (Padrão)`, `SunSet`, `Firestorm`, `AnsaStorm`, `StarLight`, `Vintage`, etc.) e o respectivo esquema de cores.
3. Confira o resultado no cartão **Pré-visualização ao vivo** (botão principal, campo de texto digitado e amigo online) e veja toda a interface atualizar imediatamente.
4. Para voltar ao visual clássico a qualquer momento, clique em **Restaurar padrão (Aurora)**.

---

## 2. Skins Nativas (`Aurora` & `SunSet`)

| Família | Esquema (`skin_theme`) | Descrição e Paleta Principal |
| :--- | :--- | :--- |
| **Aurora (Padrão)** | `Aurora (Padrão)` (`default`) | Visual original preservado integralmente (`Theme::default()`): `violet #8B5CF6`, `indigo #4F46E5`, `teal #5EEAD4`, `navy #070B1F`, `panel #252830`, `bar #1B1E26`, `field #181B23`, `raised #363B47`, `ink #E8EDFB` |
| **SunSet** | `Tema SunSet (Pôr do Sol)` (`sunset_default`) | Crepúsculo quente: fundo ameixa-noturno (`#1F1726`), acento laranja-coral pôr do sol (`#F97316`), secundário magenta crepúsculo (`#E11D48` / `#EC4899`), destaque âmbar dourado (`#FBBF24`), texto creme quente (`#FFF1E6`), digitado (`#FFE8D6`) |
| **SunSet** | `Rosa` (`rose`) | Fundo vinho/rosa escuro aveludado (`#2A1925` / campo `#1C1019`), acento rosa vibrante (`#EC4899` / `#F472B6`), botões malva (`#3D2535`), texto e texto digitado em branco-rosado suave (`#FCE7F3`) |
| **SunSet** | `Rosa Pastel (Claro)` (`rose_pastel`) | Tema claro elegante: fundo `#FCE7F3`, painéis `#F8D7EA`, acento framboesa (`#BE185D`), botões `#F3C4E0`, texto vinho escuro (`#3B0D26`) |
| **SunSet** | `Violeta` (`violet`) | Violeta elétrico intenso: painel `#1D192B`, acento `#7C3AED` / `#A855F7`, botões `#2E2747`, texto `#F3EFFE`, realces `#DDD6FE` |
| **SunSet** | `Roxo` (`purple`) | Púrpura profundo/real: painel berinjela escuro `#1C1226`, acento `#9333EA` / secundário `#6B21A8`, botões `#38234C`, texto `#F5EAFF`, digitado lavanda claro (`#EFE2FF`) |
| **SunSet** | `Lilás` (`lilac`) | Lilás suave escuro: painel `#261F33`, acento lilás `#C084FC` / `#A78BFA`, botões `#392F4D`, texto `#F6F0FF`, digitado `#EDE4FF` |
| **SunSet** | `Lilás Claro` (`lilac_light`) | Tema claro luminoso: fundo `#EFE6FA`, painéis `#F7F1FE`, acento roxo-lilás `#7E22CE`, botões `#E2D3F5`, texto `#231438` |

> Nos esquemas claros (`Rosa Pastel (Claro)`, `Lilás Claro` e temas claros do `StarLight`), a tela de login substitui o céu noturno estrelado por um céu crepuscular pastel suave (`aurora_bg.rs`), os menus suavizam o destaque ao passar o mouse e as placas de nome 3D (`ui/nameplate/paint.rs`) ajustam automaticamente a opacidade e o contraste do texto para leitura nítida.

---

## 3. Skins Clássicas do Firestorm Incluídas (22 Esquemas)

Além da família nativa `SunSet`, o visualizador inclui **22 conjuntos de cores** portados das famílias tradicionais do **Firestorm**:

| Família (`skin`) | Conjuntos de Cores (`skin_theme`) |
| :--- | :--- |
| **Firestorm** (`firestorm`) | `Grey`, `Dark`, `Blue`, `High Contrast`, `CtrlAltStudio` |
| **AnsaStorm** (`ansastorm`) | `Bright Blue`, `Classic Brown`, `Ectoplasma`, `Blood` |
| **AnsaStorm Modern** (`ansastorm_modern`) | `Blue` |
| **MetaHarper Modern** (`metaharper_modern`) | `CoolOcean`, `AppalachianSky`, `BlackGlass` |
| **StarLight** (`starlight`) | `Original Orange`, `Original Teal`, `Mono Teal` *(Claro)*, `Nostalgia Blue`, `Silver Blue` *(Claro)*, `Silver Pink` *(Claro)* |
| **StarLight CUI** (`starlight_cui`) | `Custom Dark`, `Custom Light` *(Claro)* |
| **Vintage** (`vintage`) | `Classic` |

---

## 4. Customização Avançada, Acessibilidade e Persistência

- **Cores da Interface (`interface_colors.rs`)**: Na mesma aba **Aparência › Skin**, é possível ajustar manualmente apenas **Cor de destaque**, **Fundo da janela** e **Texto** sobre a skin ativa; o sistema recalcula automaticamente as variações em OKLCH/OKLab preservando contraste mínimo WCAG (`4.5:1` para texto e `3:1` para acento).
- **Cores Semânticas de Chat e Amigos (`ui/colors.rs`)**: Enquanto o usuário não personalizar cores individuais na aba **Aparência › Cores**, a paleta da skin fornece automaticamente tons legíveis para mensagens próprias, de amigos, URLs, menções e marcadores (`SkinChatColors`).
- **Alto Contraste e Daltonismo**: **Preferências › Acessibilidade** (`high_contrast.rs` e `colorblind.rs`) sobrepõem ou adaptam a paleta da skin mantendo todas as telas legíveis.
- **Pasta de Skins em Disco (`ui/skin.rs`)**: Sobrescritas avançadas via `%APPDATA%\Aurora\AuroraViewer\config\skins\<familia>\theme.json` continuam suportadas com recarregamento automático.
- **Persistência**: As preferências escolhidas são salvas automaticamente em `settings.json` nos campos `skin` e `skin_theme` (com sanitização automática em `Settings::sanitize()`).
- **Modo Demonstração / Testes**: É possível forçar uma skin na inicialização via variável de ambiente `AURORA_DEMO_SKIN=<familia>:<esquema>` (ex.: `sunset:rose`, `sunset:rose_pastel`, `firestorm:blue`).

---

## 5. Garantias Automatizadas

As propriedades de contrato e acessibilidade das skins são validadas continuamente nos testes unitários:
- `skin_presets::tests::aurora_default_matches_theme_default_exact` (garante paridade byte-a-byte com `Theme::default()`).
- `skin_presets::tests::sunset_default_is_distinct_twilight_palette`.
- `skin_presets::tests::all_skin_presets_meet_readability_ratios` (valida todas as 30 combinações quanto a contraste de texto, `typed_text` em campos e superfícies, rótulos secundários e contraste de botões).
- `sunset_tests::test_skin_defaults_and_presets_contract` e `sunset_tests::test_skin_i18n_keys_present_across_all_locales`.
- `ui::theme_guard` (impede regressões com cores ou raios fixos fora do sistema de tema).
