# Guia de Atualização e Arquitetura — SunSet Viewer v0.9.7

Este documento descreve todas as funcionalidades, melhorias de engenharia e garantias de não-regressão entregues no **SunSet Viewer v0.9.7**, que unifica a base oficial do **Aurora Viewer v0.9.4** com os recursos exclusivos de combate, mobilidade, alcance visual e localização do **SunSet Viewer**.

---

## 🌟 1. Funcionalidades Exclusivas do SunSet Viewer (Preservadas e Auditadas)

| Funcionalidade | Descrição Técnica e Uso | Documentação Detalhada |
| :--- | :--- | :--- |
| **Distância de Visão Estendida (`dd 5000+` até `8.192m`) + SpeedRez** | Suporte a até **8.192 metros** via comando `dd <metros>` no chat local, Preferências Rápidas e Preferências › Gráficos (`Settings::sanitize` em `32.0..=8192.0`), harmonizado com a nova **Distância de Visão Progressiva (SpeedRez)**. | [DRAW_DISTANCE.md](DRAW_DISTANCE.md) |
| **Controle de Movimento (100x) & Salto Instantâneo (Dash)** | Multiplicador de velocidade de solo e voo até **100x** via **Aurora LSL Bridge v1.42**, salto instantâneo (`Dash`) de até **8.192m** (`/dash <metros>`) e atalhos em Mouselook (**Tecla `E`** e **Botão do Meio do Mouse**). | [MOVEMENT_DASH.md](MOVEMENT_DASH.md) |
| **Hitboxes 3D de Combate & Raycast Local** | Renderização em tempo real dos cubos de colisão 3D reportados pelo simulador, detecção local de impacto por articulação do esqueleto, linha de diagnóstico de raycast e oclusão por objetos/terreno (`scene_objects_block_rays`). | [HITBOX_E_CORES.md](HITBOX_E_CORES.md) |
| **Alcance do Texto Flutuante / Meter de Combate (`floating_text_fade_distance` & `floating_text_fade_range`)** | Controle da distância de início (`0..=64m`, padrão `8m`) e alcance de transição (`0.25..=32m`, padrão `4m`) do texto flutuante 3D (`llSetText`), permitindo enxergar *meters* de combate a até **96 metros** em **Preferências › Combate** ou **Preferências › Layout › Dicas de ferramentas**. | [HITBOX_E_CORES.md](HITBOX_E_CORES.md) |
| **Cores Individuais de Hitbox e Minimapa (`avatar_marks`)** | Marcação individual de cor por residente via menu de contexto (`Marcar cor` › Paleta rápida ou RGB/HSV), com precedência máxima sobre Contact Sets tanto no **Minimapa** quanto no **Hitbox 3D** do mundo. | [HITBOX_E_CORES.md](HITBOX_E_CORES.md) |
| **Localização 100% Nativa em Português (`pt-BR`) e Espanhol (`es`)** | Mais de 13.100 chaves traduzidas em `crates/aurora-i18n/locales/pt-BR.ftl` e `crates/aurora-i18n/locales/es.ftl`, com zero chaves faltantes em relação ao catálogo em inglês e terminologia autêntica do Second Life. | [TRANSLATING.md](TRANSLATING.md) |
| **Sistema de Skins & Conjuntos de Cores (`Aurora`, `SunSet` & `Firestorm`)** | Troca ao vivo em **Preferências › Aparência › Skin** com chips rápidos de 1 clique, preservando `Aurora (Padrão)` idêntico ao `Theme::default()` e oferecendo 7 temas exclusivos **SunSet** (Crepúsculo, Rosa, Rosa Pastel Claro, Violeta, Roxo, Lilás e Lilás Claro) + 22 esquemas portados do **Firestorm**, colorindo interfaces, menus, botões, amigos, crachás 3D e textos digitados. | [SKINS_E_TEMAS.md](SKINS_E_TEMAS.md) |
| **Instalador MSI Customizável & Auto-Update com Elevação UAC** | Escolha livre de diretório de instalação no MSI (`WixUI_InstallDir`) e atualização automática resiliente via `%TEMP%` com elevação UAC sob demanda. | [UPDATES.md](UPDATES.md) |

---

## 🚀 2. Novidades Integradas do Aurora Viewer v0.9.5 & Release Final v0.9.6

### 🌌 Céu Estelar, Via Láctea e Tipos de Renderização (`Rendering Types`)
- Renderização panorâmica realista da Via Láctea integrada ao pipeline de céu diurno/noturno e catálogo Tycho-2.
- Novo menu `Avançado › Tipos de renderização` (Terreno, Água, Céu, Nuvens, Objetos, Avatares, Animesh, Árvores, Grama e Partículas) com atalhos de teclado dedicados.
- Correção de sombras de contato sem *workgroup memory* para placas **AMD Radeon RX 9000 (RDNA 4)** e aplicação única de nitidez (*FidelityFX CAS*) no upscale.

### 👗 Vitrine de Trajes, Retratos Automáticos, Lote e Foto HD
- **Retrato Automático e Modo Foto Quadrada (`#3226`)**: Enquadramento automático (corpo inteiro, busto ou rosto) com fundo e iluminação de estúdio locais.
- **Retratos em Lote (`#3235`)**: Fotografe automaticamente múltiplos trajes sem foto direto pela Vitrine de Trajes.
- **Foto HD Opcional (`#3237`)**: Opção de salvar textura HD (`512×512` ou `1024×1024`) na pasta do traje além da miniatura gratuita (`256×256`).

### 📸 65 LUTs Embutidos, Poser e Construção
- **65 Looks de Foto Embutidos (`#3205`, `#3224`)**: 20 LUTs por **Czar Fenris** e 45 LUTs por **Nanner Pudding** comprimidos no executável, além de importação de vários `.cube` de uma só vez.
- **Poser + DoF (`#3230`)**: O Poser preserva a profundidade de campo da janela de fotografia.
- **Inventário, Gizmo e Região**: Favoritos sem pastas pai por padrão (`#3212`), confirmação com `Enter`/`Tab` ao excluir (`#2979`), freio de movimento com `Espaço` (`#2417`) e confirmação de restrições de Região/Propriedade que enviam residentes para casa (`#3243`).

### 🧭 Rastreador de Regiões (Region Tracker)
- Disponível em `Mundo › Rastreador de região`.
- Permite adicionar e monitorar regiões favoritas em tempo real (status online/offline e contagem de avatares presentes) sem abrir o mapa-múndi.

### 🎨 Ordenação Alfa e Snap de Terreno
- **Ordenação Alfa Aprimorada**: Minimização de artefatos de renderização em sobreposição de geometrias transparentes (como cabelos e vegetação).
- **Snap de Terreno**: Alinhamento inteligente de objetos à malha de altura da parcela durante o modo de construção/edição.

### 💬 Chat, Emojis e Gestão de Cache de Áudio
- **Emojis & GIFs**: Linha de emojis recentes, aba de favoritos e seletor rápido no chat.
- **Cache de Sons (LRU Eviction)**: Gerenciamento inteligente de clips de som em memória (`SoundClip` memory budget) com liberação automática de clips inativos.

---

## 🛡️ 3. Suíte de Testes Unitários de Não-Regressão (`crates/aurora-viewer/src/sunset_tests.rs`)

Para blindar o visualizador contra quebras acidentais em futuras sincronizações upstream, foi desenvolvida uma suíte de testes unitários contendo **16 testes automatizados**, cobrindo rigorosamente todos os pilares customizados do SunSet Viewer, a localização em Espanhol, as skins e a integração da `v0.9.6`:

1. **`test_draw_distance_sanitize_clamps_to_8192`**: Garante que `Settings::sanitize()` nunca rejeite ou resete distâncias de visão até `8192.0m`.
2. **`test_quick_prefs_slider_draw_distance_range_allows_8192`**: Garante que o slider de Preferências Rápidas permita deslizar até `8192.0m`.
3. **`test_speedrez_draw_distance_stepping_supports_8192m`**: Valida a progressão exponencial de SpeedRez partindo de 32m até 8192m.
4. **`test_movement_dash_settings_and_multiplier_up_to_100x`**: Valida o clamping de dash até 8192m e o multiplicador de velocidade até 100x.
5. **`test_movement_preferences_sanitize_run_and_fly_100x`**: Testa a sanitização de preferências de corrida e voo assistido até 100.0x.
6. **`test_combat_settings_and_preferences_page_presence`**: Assegura a presença da página `Page::Combat` na lista global de opções `Page::ALL`.
7. **`test_combat_hitbox_and_scene_blocking_behavior`**: Valida a lógica de cálculo de hitboxes 3D, detecção de membros e bloqueio por obstáculos do cenário (`scene_objects_block_rays`).
8. **`test_avatar_marks_individual_colors_precedence`**: Valida a prioridade de marcação individual de cor de avatar sobre Contact Sets no Minimapa e no Combate.
9. **`test_localization_pt_br_has_zero_missing_keys_compared_to_en`**: Valida programaticamente que o arquivo `pt-BR.ftl` possui **zero** chaves faltantes em relação ao `en.ftl`.
10. **`test_sunset_branding_logo_and_assets_validity`**: Verifica os magic bytes de imagem PNG e a integridade de `assets/branding/sunset-logo.png`.
11. **`test_ci_cd_workflows_and_build_scripts_integrity`**: Valida a presença do runner `macos-15` em `release.yml`, a emissão de símbolos PDB para Windows e o suporte a builds locais no script `check.ps1`.
12. **`test_aurora_v096_final_features_integrated_without_regression`**: Valida os 65 LUTs embutidos, os 10 controles de `Rendering types` e a convivência de `outfit_portrait` com `draw_distance` de `8192m`.
13. **`test_localization_es_has_zero_missing_keys_compared_to_en`**: Valida programaticamente que o catálogo `es.ftl` possui **zero** chaves faltantes em relação ao `en.ftl`.
14. **`test_skin_defaults_and_presets_contract`**: Garante preservação exata do padrão `Aurora (Padrão)` (`aurora/default`), presença das 9 famílias (incluindo os 7 esquemas `SunSet` e polaridade clara de `Rose Pastel` / `Lilac Light`) e sanitização de aliases em `Settings::sanitized()`.
15. **`test_skin_i18n_keys_present_across_all_locales`**: Assegura que todas as 21 chaves de UI e nomes de skins estejam traduzidas em `en.ftl`, `pt-BR.ftl`, `es.ftl`, `fr.ftl` e `ja.ftl`.
16. **`test_floating_text_fade_settings_sanitized_and_extended_combat_range`**: Valida os valores padrão (`8.0m` e `4.0m`), a sanitização (`0.0..=64.0m` e `0.25..=32.0m`) e o cálculo de opacidade do texto flutuante / meter de combate a longas distâncias (até `96.0m`).

