# Guia de Atualização e Arquitetura — SunSet Viewer v0.9.7

Este documento descreve todas as funcionalidades, melhorias de engenharia e garantias de não-regressão entregues no **SunSet Viewer v0.9.7**, que unifica a base oficial do **Aurora Viewer v0.9.4** com os recursos exclusivos de combate, mobilidade, alcance visual e localização do **SunSet Viewer**.

---

## 🌟 1. Funcionalidades Exclusivas do SunSet Viewer (Preservadas e Auditadas)

| Funcionalidade | Descrição Técnica e Uso | Documentação Detalhada |
| :--- | :--- | :--- |
| **Distância de Visão Estendida (`dd 5000+` até `8.192m`) + SpeedRez** | Suporte a até **8.192 metros** via comando `dd <metros>` no chat local, Preferências Rápidas e Preferências › Gráficos (`Settings::sanitize` em `32.0..=8192.0`), harmonizado com a nova **Distância de Visão Progressiva (SpeedRez)**. | [DRAW_DISTANCE.md](DRAW_DISTANCE.md) |
| **Controle de Movimento (100x) & Salto Instantâneo (Dash)** | Multiplicador de velocidade de solo e voo até **100x** via **Aurora LSL Bridge v1.42**, salto instantâneo (`Dash`) de até **8.192m** (`/dash <metros>`) e atalhos em Mouselook (**Tecla `E`** e **Botão do Meio do Mouse**). | [MOVEMENT_DASH.md](MOVEMENT_DASH.md) |
| **Hitboxes 3D de Combate & Raycast Local** | Renderização em tempo real dos cubos de colisão 3D reportados pelo simulador, detecção local de impacto por articulação do esqueleto, linha de diagnóstico de raycast e oclusão por objetos/terreno (`scene_objects_block_rays`). | [HITBOX_E_CORES.md](HITBOX_E_CORES.md) |
| **Cores Individuais de Hitbox e Minimapa (`avatar_marks`)** | Marcação individual de cor por residente via menu de contexto (`Marcar cor` › Paleta rápida ou RGB/HSV), com precedência máxima sobre Contact Sets tanto no **Minimapa** quanto no **Hitbox 3D** do mundo. | [HITBOX_E_CORES.md](HITBOX_E_CORES.md) |
| **Localização 100% Nativa em Português (`pt-BR`)** | Mais de 12.770 chaves traduzidas em `crates/aurora-i18n/locales/pt-BR.ftl`, incluindo todas as 120 novas strings introduzidas na base 0.9.4 e todas as chaves exclusivas do SunSet Viewer. | [TRANSLATING.md](TRANSLATING.md) |

---

## 🚀 2. Novidades Integradas do Aurora Viewer v0.9.5 & v0.9.6

### 🌌 Céu Estelar e Renderização da Via Láctea (Milky Way)
- Renderização panorâmica realista da Via Láctea integrada ao pipeline de céu diurno/noturno.
- Integração refinada com o catálogo de estrelas Tycho-2 para brilho e densidade celestial precisa.

### 👗 Vitrine de Trajes (Outfit Showcase)
- Nova funcionalidade para inspeção e gerenciamento visual de trajes no inventário.
- Permite pré-visualizar itens anexados e camadas com facilidade.

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

Para blindar o visualizador contra quebras acidentais em futuras sincronizações upstream, foi desenvolvida uma suíte de testes unitários contendo **11 testes automatizados**, cobrindo rigorosamente todos os 7 pilares customizados do SunSet Viewer:

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

