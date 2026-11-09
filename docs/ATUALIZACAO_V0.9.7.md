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

## 🚀 2. Novidades Integradas do Aurora Viewer v0.9.4

### 🔧 Configurações de Depuração (`Debug Settings` — `Ctrl+Alt+Shift+S`)
- Nova janela dedicada acessível pela barra de ferramentas ou pelo atalho **`Ctrl+Alt+Shift+S`**.
- Permite pesquisar qualquer configuração interna pelo nome técnico (ou equivalente do visualizador clássico/Firestorm) e pela descrição.
- Exibe filtro de configurações modificadas e botão para restaurar qualquer chave individualmente ao valor padrão.

### 🏗️ Preferências de Construção (`Preferências › Construção`)
- Nova página em `Preferências (Ctrl+P)` › **`Construção`** para definir:
  - Dimensões padrão (`X`, `Y`, `Z`), material, textura e opções (**Físico**, **Temporário**, **Fantasma**) de novos prims criados.
  - Ponto de pivô no objeto raiz, destaque de prims selecionados, distância máxima de seleção e exibição de eixos na seleção.
  - Permissões padrão de criação e regras de duplo clique no inventário.

### 🌍 Mundo, Gráficos e Estabilidade de VRAM
- **Distância de Visão Progressiva (SpeedRez)**: inicia em `32m` ao chegar em uma região e dobra progressivamente até a sua distância configurada (até `8.192m`).
- **Descoberta Dinâmica de Regiões Vizinhas**: conecta até 3 anéis de regiões adjacentes conforme a distância de visão.
- **Proteção contra Superfícies Gigantes (`Large Surface Protection`)**: oculta automaticamente faces texturizadas descomunais que causariam queda brusca de FPS.
- **Detalhe de Árvores (`Trees Detail`) e Partículas de Anexos**: controle independente de LOD para vegetação e interruptor para ocultar partículas emitidas por objetos anexados a outros avatares.
- **Recuperação Automática de VRAM (`GPU Recovery`)**: reconstrói a cena 3D automaticamente caso a memória de vídeo fique cheia durante o login.
- **Correção de Frustum/Occlusion Culling**: objetos próximos à câmera não desaparecem mais em determinados ângulos de visão.

### 💬 Voz, Música e Interface
- **Chamadas de Voz Contínuas no Teleporte**: chamadas privadas de voz permanecem conectadas ao atravessar ou teletransportar entre regiões.
- **Música do Terreno sem Cortes**: o stream de áudio da parcela continua tocando normalmente ao mover a câmera livremente para fora do terreno.
- **Alternância Rápida de Conversas (`Ctrl+Tab` / `Ctrl+Shift+Tab`)**: navegue rapidamente entre abas da janela de Conversas.
- **Suavização de Câmera até `300`**: alcance ampliado para movimentos cinematográficos em vídeos e fotos.
- **Suporte Aprimorado a IME Japonês**: destaque visual do segmento de conversão ativo.

---

## 🛡️ 3. Resumo da Auditoria de Não-Regressão

Todos os pontos de interseção entre o código upstream (Aurora v0.9.4) e as customizações do SunSet Viewer foram auditados e validados por testes automatizados:

1. **`crates/aurora-viewer/src/settings.rs`**: `self.draw_distance = self.draw_distance.clamp(32.0, 8192.0)` preservado em `Settings::sanitize()`.
2. **`crates/aurora-viewer/src/ui/quick_prefs.rs` & `pages/graphics.rs`**: `Slot::DrawDistance` e o slider de Gráficos operam na faixa `32.0..=8192.0`.
3. **`crates/aurora-viewer/src/app/draw_distance_step.rs`**: `Stepping` respeita metas de até `8192.0m`.
4. **`crates/aurora-viewer/src/movement/mod.rs` & `ui/movement_dash.rs`**: multiplicador `100x`, salto até `8192m` e atalhos de Mouselook intactos.
5. **`crates/aurora-viewer/src/scene/combat.rs`, `ui/debug_overlay.rs` & `ui/minimap.rs`**: Hitboxes 3D, raycast com oclusão de cena e cores individuais (`avatar_marks`) operando sem regressão.

