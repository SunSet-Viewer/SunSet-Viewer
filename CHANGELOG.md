# SunSet Viewer — Registro de Mudanças (Changelog)

Todas as mudanças notáveis, novidades e correções de bugs do projeto **SunSet Viewer** estão documentadas neste arquivo.

---

## [0.9.9] — 2026-11-14

### 🌐 Localização Nativa em Espanhol (`es.ftl`)
- **Paridade Completa em Espanhol**: Catálogo completo `crates/aurora-i18n/locales/es.ftl` com **13.077 chaves** traduzidas e validadas em paridade absoluta com Inglês (`en.ftl`) e Português do Brasil (`pt-BR.ftl`).
- **Pacotes de Idioma e Sistema**: Atualizados os metadados de idioma para macOS (`es.lproj/InfoPlist.strings`) e Linux (`aurora-viewer.desktop`), além do verificador automático de catálogos (`aurora-i18n/tests/catalogs.rs`).

### 🎨 Temas e Esquemas de Skins SunSet (Preferências › Aparência)
- **Seletor de Família e Esquema ao Vivo**: Novo painel em **Preferências › Aparência** para alternar entre as famílias **Aurora** e **SunSet** com aplicação instantânea em tempo real e restauração rápida para Aurora.
- **6 Esquemas Exclusivos SunSet**: *Padrão (SunSet)*, *Rosa*, *Rosa Pastel*, *Violeta*, *Roxo* e *Lilás Claro*, integrados à paleta, fundos e widgets da interface.

### 💬 Textos Flutuantes (`llSetText`) e Combate 128 m
- **Distância e Desvanecimento de Textos Flutuantes**: Novos controles de alcance máximo (`0–128 m`) e faixa de desvanecimento suave (`1–64 m`) em **Preferências › Gráficos** e nas **Preferências Rápidas**.
- **Combate Até 128 m**: Alcance de exibição de hitboxes 3D e raycast de combate ampliado para até `128 m`.

---

## [0.9.8] — 2026-11-10

### 👗 Vitrine de Trajes, Retratos Automáticos e Foto HD (Aurora v0.9.6 Final)
- **Retratos Automáticos de Trajes (Auto Portrait)**: Enquadramento inteligente por ossos do avatar (corpo inteiro, busto e rosto), poses locais de estúdio e iluminação dedicada sem afetar a visão dos demais residentes.
- **Retratos em Lote (Batch Portraits)**: Geração sequencial automatizada de retratos para todos os trajes do inventário sem imagem a partir da Vitrine de Trajes.
- **Foto HD Opcional para Trajes**: Suporte a texturas em alta resolução (512×512 ou 1024×1024) salvas na pasta do traje além da miniatura gratuita (256×256).

### 📸 65 Looks de Foto Embutidos e Aprimoramentos no Poser
- **LUTs por Czar Fenris e Nanner Pudding**: 65 tabelas de cores 3D (20 do Czar Fenris e 45 do Nanner Pudding) integradas e comprimidas nativamente no executável.
- **Importação Múltipla de LUTs**: Suporte à seleção e importação de múltiplos arquivos `.cube` simultâneos.
- **Poser com Profundidade de Campo (DoF)**: O editor de poses agora preserva o desfoque de profundidade de campo configurado no modo de fotografia.

### 🎮 Renderização, GPU e Controles
- **Tipos de Renderização (Advanced › Rendering Types)**: Menu com alternância rápida para Terreno, Água, Céu, Nuvens, Objetos, Avatares, Animesh, Árvores, Grama e Partículas com atalhos de teclado dedicados no padrão do visualizador clássico.
- **Correção para AMD Radeon RX 9000 (RDNA 4)**: Eliminação de travamentos de GPU (TDR/device lost) nas sombras de contato através de shader sem dependência de workgroup memory.
- **Freio com Barra de Espaço**: Tecla `Espaço` interrompe o movimento do avatar imediatamente no solo ou no ar.
- **Confirmações de Inventário com Teclado**: Teclas `Enter` e `Tab` agora respondem diretamente aos diálogos de exclusão de itens e esvaziamento da lixeira.
- **Proteção de Região/Propriedade**: Diálogo de confirmação para alterações de acesso que enviem residentes de volta para casa.

### 📦 Instalador Windows MSI e Atualizador Automático
- **Diretório Customizável de Instalação no MSI**: O instalador oficial Windows agora permite selecionar qualquer pasta de destino (`WixUI_InstallDir`).
- **Elevação UAC Transparente via `%TEMP%`**: O atualizador executa a instalação a partir do diretório temporário do usuário, garantindo elevação segura de privilégios administrativos.

### 🌐 Localização e Não-Regressão
- **Paridade Total em Português (`pt-BR.ftl`)**: 169 novas chaves traduzidas e validadas, com 100% de paridade em relação ao inglês.
- **12 Testes Unitários de Não-Regressão**: Suíte automatizada validando permanentemente todos os recursos exclusivos do SunSet e a base 0.9.6.

---

## [0.9.7] — 2026-11-10

### 🧪 Suíte de Testes Unitários de Não-Regressão SunSet
- **11 Testes Unitários Automatizados**: Implementada a suíte `crates/aurora-viewer/src/sunset_tests.rs` validando permanentemente contra regressões:
  - Alcance visual estendido até 8.192m e sanitização (`Settings::sanitized`).
  - Escalonamento progressivo do SpeedRez (`Stepping`) até 8.192m.
  - Controle de movimento e voo com multiplicadores de até 100x.
  - Janela de Dash instantâneo, atalhos de Mouselook (tecla `E` e botão do meio) e física do Aurora LSL Bridge v1.42.
  - Caixas de colisão 3D de combate (`CombatSettings`) e oclusão de raycast por terreno/objetos de cena.
  - Presença da página de preferências de Combate (`Page::Combat` em `Page::ALL`).
  - Precedência de cores individuais de residentes (`avatar_marks`) sobre Contact Sets no Minimapa e Hitboxes 3D.
  - Localização 100% nativa em Português do Brasil com paridade absoluta (0 chaves ausentes em relação ao `en.ftl`).
  - Integridade da identidade visual SunSet (logo `assets/branding/sunset-logo.png` e metadados).
  - Preservação da infraestrutura CI/CD (runner macOS ARM64 `macos-15` e símbolos PDB do Windows).

### 🌌 Sincronização Upstream Aurora-Viewer v0.9.5 & v0.9.6
- **Céu Noturno e Via Láctea (Milky Way)**: Renderização realista da Via Láctea integrada ao pipeline de renderização estelar com catálogo Tycho-2.
- **Vitrine de Trajes (Outfit Showcase)**: Nova janela e fluxo para inspecionar, pré-visualizar e gerenciar trajes e anexos diretamente pelo inventário.
- **Rastreador de Regiões (Region Tracker)**: Monitoramento em segundo plano do status operacional e lotação de regiões favoritas (`Mundo › Rastreador de região`).
- **Aprimoramentos de Renderização Alfa**: Nova ordenação de transparência alfa reduzindo artefatos em camadas de roupas, cabelos e vegetação.
- **Encaixe de Terreno (Terrain Snapping)**: Ferramenta aprimorada para alinhamento e ancoragem de prims ao relevo da parcela.
- **Chat e Emojis**: Suporte a emojis recentes, favoritos, atalho `Alt+Enter` para `(( OOC ))` e histórico de GIFs.
- **Gerenciamento de Memória de Áudio**: Implementado algoritmo LRU com liberação dinâmica de clipes de áudio não utilizados (`SoundClip` cache eviction).

### 🚀 Desbloqueio e Aceleração do Multiplicador de Velocidade (até 100x)
- **Correção da Barreira de 5x**: Resolvida a falha onde a velocidade do avatar não aumentava acima de 5x. O parser de telemetria do bridge (`Ack::parse`) rejeitava respostas com velocidade alvo superior a 2.565 cm/s (`5.13 m/s * 5`), o que causava perda de pacotes e desativação do motor pelo watchdog. O limite foi ampliado para suportar velocidades de até 1.000.000 cm/s.
- **Ampliação do Limite de 20x para 100x**: O multiplicador de velocidade agora permite valores de **1.0x até 100.0x**, com a adição dos botões de predefinição rápida **`50x`** e **`100x`** no painel `Movimento & Dash` e ajuste fino via slider/drag.
- **Atualização do Script Aurora LSL Bridge para v1.42**:
  - Rampa de aceleração dinâmica proporcional à velocidade solicitada, eliminando a lentidão para atingir altas velocidades.
  - Aumento da força máxima de empuxo (até 6.000 m/s²) para superar a resistência do arrasto físico imposta pelo Havok em solo.
  - Tolerância adaptativa a desníveis e relevo do terreno no eixo Z, evitando cancelamentos indevidos de corrida ao cruzar pequenas inclinações.
  - Suporte a curvas suaves com ângulo de até 45° sem zerar bruscamente a velocidade acumulada.
- **Paridade em Voo Assistido**: O multiplicador de velocidade de voo assistido (`fly_multiplier`) agora também escala até 100x tanto no painel `Movimento & Dash` quanto no floater de movimento nativo.

### 👁️ Alcance Visual Desbloqueado até 8.192m e SpeedRez
- **Progressive Draw Distance (SpeedRez)**: Início dinâmico em 32m com duplicação gradual até a distância configurada, com teto de até 8.192m (`dd 5000+` no chat local e slider de preferências).

### 📚 Documentação Técnica Criada e Atualizada
- **Checklist Definitivo de Release**: Guia passo a passo em [`docs/CHECKLIST_RELEASE.md`](docs/CHECKLIST_RELEASE.md).
- **Controle de Movimento e Dash**: Guia completo em [`docs/MOVEMENT_DASH.md`](docs/MOVEMENT_DASH.md).
- **Distância de Visão Estendida**: Guia em [`docs/DRAW_DISTANCE.md`](docs/DRAW_DISTANCE.md).
- **Hitboxes 3D e Cores Individuais**: Guia em [`docs/HITBOX_E_CORES.md`](docs/HITBOX_E_CORES.md).
- **Arquitetura da Versão 0.9.7**: Detalhamento em [`docs/ATUALIZACAO_V0.9.7.md`](docs/ATUALIZACAO_V0.9.7.md).

---

## [0.9.6] — 2026-11-08

### 🎨 Marcador de Cores Individual para Contatos (Estilo Firestorm)
- **Menu de Contexto de Avatar**: Adicionado o submenu `Marcar cor` com paleta rápida de cores e seletor personalizado RGB/HSV completo para destacar qualquer avatar individualmente pelo botão direito.
- **Sobreposição e Precedência de Cores**: A cor individual marcada tem prioridade sobre as cores de predefinição herdadas de conjuntos de contatos (Contact Sets), aplicando-se aos hitboxes de combate 3D, pontos do minimapa e listas de pessoas/nomes.
- **Restauração Dinâmica**: Opção `Limpar cor` para restaurar imediatamente o avatar à cor predefinida do seu conjunto de contatos (ou padrão do sistema).
- **Persistência Entre Sessões**: As marcações individuais de cor de avatares são salvas automaticamente em `settings.json` e restauradas em cada inicialização.
- **Localização Multilíngue**: Suporte completo às novas opções nos idiomas Português do Brasil (`pt-BR`), Inglês (`en`), Francês (`fr`) e Japonês (`ja`).

---

## [0.9.5] — 2026-11-08

### ⚡ Novo Sistema Nativo de Movimento & Dash Instantâneo
- **Painel Flutuante Dedicado**: Adicionada a janela `Controle de Movimento & Dash`, acessível via menu superior (`Mundo` -> `Controle de Movimento & Dash`) ou pelo chat (`/dash menu`).
- **Salto Instantâneo (Dash)**: Permite saltar distâncias predefinidas (25m, 50m, 100m, 200m, 500m) ou qualquer metragem personalizada instantaneamente via comando `/dash <metros>` ou botão no painel.
- **Atalhos no Mouselook**: Pressione a tecla **`E`** ou dê um **clique com o botão do meio do mouse** na visão em primeira pessoa para disparar um dash em direção à mira.
- **Multiplicador de Velocidade de Movimento**: Controle preciso da velocidade de corrida e voo (1x até 20x), com ativação automática ao se mover e reset seguro instantâneo ao desligar o sistema ou retornar para 1x.

### 🌐 Correções de Localização (pt-BR)
- **Mouse**: Substituídas as ocorrências de "Rato" por **"Mouse"** em toda a interface de preferências, atalhos de controle e cabeçalhos de configuração ("Mouse e teclado", "Mouse para trás", seção "MOUSE").

### 👁️ Alcance Visual Desbloqueado (Draw Distance)
- **Extensão de Draw Distance**: Desbloqueado o limite máximo de alcance de renderização de até **8.192 metros** via comando `dd` no chat e no protocolo de rede `AgentUpdate`.

---

## [0.9.4] — 2026-11-07

### 🌐 Melhorias e Correções na Localização em Português do Brasil (pt-BR)
- **Menu de Anexos**: Corrigida a tradução de `Detach from yourself` de "Desapegue-se de você mesmo" para **"Desanexar de você"**.
- **Itens em Uso**: Corrigida a tradução de `Worn` de "Desgastado" para **"Em uso"** (aba do inventário, etiquetas e filtros) e botão `Take off` para **"Tirar"**.
- **Alternância de Corrida e Atalhos**: Corrigido `Always run` de "Always run: Ativado / Desativado" para **"Sempre correr: Ativado / Desativado"**.
- **Correções de Tradução Literal**:
  - Tecla Esc: Corrigido de "A fuga é cancelada" / "Fuga" para **"Pressione Esc para cancelar."** e **"Esc"**.
  - Contatos: Corrigido `Calling card` de "Cartão telefônico" para **"Cartão de visita"**.
  - Região/Terreno: Corrigido `Bake the terrain` de "Assar o terreno" para **"Fixar o terreno"**.
  - Contexto de Inventário: `Find all links` corrigido de "Find Todos links" para **"Localizar todos os links"**.
  - Mover pastas: Corrigido `Move to…` de "Mude para…" para **"Mover para…"**.
  - Câmera: `Cannot focus: too close` corrigido de "Cannot focus: too Fechar" para **"Não é possível focar: muito próximo"**.
- **Mensagens do Sistema e Chat**: Traduzidas notificações de transações L$, alertas de amigos online/offline, diálogos de terreno e mensagens de sistema que estavam em inglês.

---

## [0.9.3] — 2026-11-06

Esta versão marca a consolidação do **SunSet Viewer** como um projeto independente derivado do Aurora Viewer, incorporando as novidades do Aurora v0.9.3 (incluindo FSR 3.1), preservando o sistema proprietário de combate, introduzindo o novo sistema autônomo de auto-atualização e entregando 100% de suporte nativo ao Português do Brasil (pt-BR).

### 🚀 Principais Novidades (Features)

#### 1. Desvinculação e Nova Identidade (SunSet Viewer)
- **Repositório Independente**: Todas as URLs, feeds de atualização, issue trackers e links de documentação foram remapeados para `https://github.com/SunSet-Viewer/SunSet-Viewer`.
- **Ferramentas de Desenvolvimento**: Criado o script mestre `sunset-tools.cmd` no root do projeto (removido `aurora-tools.cmd`).
- **Conformidade de Licença**: Preservadas todas as licenças de código aberto (GPLv3) e atribuições de direitos autorais a Linden Lab e autores do Aurora em `NOTICE.md` e `LICENSE`.

#### 2. Sincronização Upstream (Aurora v0.9.3)
- **AMD FidelityFX Super Resolution (FSR 3.1)**:
  - Adicionado pipeline FSR 3.1 com shaders nativos em WGSL (`crates/aurora-render/src/fsr.rs` e `crates/aurora-render/src/shaders/fsr/`).
  - Upscaling de alta fidelidade e redução de carga na GPU em resoluções altas.
- **Aceleração e Otimização de Cena**:
  - `PickIndex`: Otimizada a seleção espacial excluindo malhas com rigging para evitar sobrecarga de CPU (`crates/aurora-viewer/src/scene/hover.rs`).
  - Cache de candidatos e nova lógica de cálculo de raio de hover.
- **Rede e Sessão**:
  - Reconciliados campos `owner_id` em diálogos de script (`NetEvent::ScriptDialog`), `critical_required` no login e novas permissões de grupo.

#### 3. Sistema de Combate SunSet Preservado
- **Módulo de Combate em Tempo Real**:
  - Código customizado preservado integralmente em `crates/aurora-viewer/src/scene/combat.rs`.
  - Detecção local de hitboxes e cálculo de raycast do centro da tela para armas e combate.
  - Opções dedicadas nas preferências do visualizador para diagnóstico de mira e combate.
  - Todos os testes de unidade de combate validados com sucesso (`scene::combat::tests`).

#### 4. Auto-Update Nativo e Resiliente
- **Feed do GitHub Releases**:
  - Configurado para consultar diretamente a API de releases de `SunSet-Viewer/SunSet-Viewer` (`crates/aurora-updater/src/feed.rs`).
- **Políticas e Metadados**:
  - Suporte a `sunset-update-policy.json` e `aurora-update-policy.json`.
  - Suporte a `sunset-install.json` e argumentos `--sunset-update-helper`.
- **Instalação Atômica**:
  - Swap de diretório com segurança transacional via helper externo, verificação estrita de hash SHA-256 e rollback automático em caso de erro.
  - Polling em segundo plano periódico e inteligente (com cabeçalhos `ETag` para economia de banda).

#### 5. Localização Completa em Português do Brasil (100% pt-BR)
- **12.659 Chaves e Seletores Traduzidos**:
  - Catálogo `crates/aurora-i18n/locales/pt-BR.ftl` com 100% de paridade de chaves em relação ao `en.ftl`.
  - Paridade estrita de variáveis Fluent (`{ $count }`, `{ $name }`, `{ $duration }`, etc.).
- **Terminologia Second Life Padronizada**:
  - Ajustados todos os jargões para o vocabulário oficial do Second Life:
    - Câmera: *Panorâmica* (corrigido de "Panela"), *Rolagem* (corrigido de "Role"), *Órbita*, *Mouselook*, *Objeto*.
    - Navegação: *Dock* (corrigido de "banco dos réus"), *Lixeira*, *Inventário*, *Pontos de referência (Landmarks)*.
    - Marca: *SunSet* (corrigido de "Pôr do sol" na aba de preferências e telas de login).
    - Conteúdo: *Scripts* (corrigido de "roteiros"), *Malhas/Mesh*, *Rigged*, *Prims*, *Rez*, *L$*.
  - Eliminação completa de frases híbridas em inglês (como *"Only Amigos and Grupos"*, *"Changes Aplicar"*, *"Modified Configurações"*).
- **Ajustes de Layout e Interface**:
  - Aba de configurações modificadas renomeada para **Modificadas** para evitar truncamento no menu lateral.
  - Rótulos dos botões da câmera simplificados para **Objeto** e **Mouselook**, eliminando colisão visual de texto.

---

### 🐛 Correções de Bugs (Bug Fixes)

1. **Sobreposição de Letras e Textos nos Botões da Câmera**:
   - **Problema**: Os botões "Visualização de objeto" e "Visão em primeira pessoa" excediam a largura de ~90px do `ButtonGrid`, fazendo seus textos colidirem e ficarem ilegíveis.
   - **Correção**:
     - Simplificados os textos para os termos padrão do Second Life: `Objeto` e `Mouselook`.
     - Adicionado recorte estrito (`with_clip_rect(rect)`) no renderizador `action_button_keyed` em `crates/aurora-viewer/src/ui/widgets.rs`, impedindo qualquer texto de vazar para botões adjacentes.

2. **Termos Traduzidos Erroneamente por Scripts Legados**:
   - **Problema**: O termo `controls-pan` (pan da câmera) foi traduzido como "Panela", `ui-options-aurora` traduzido como "Pôr do sol", e o dock traduzido como "banco dos réus".
   - **Correção**: Corrigidos para `Panorâmica`, `SunSet` e `Dock`.

3. **Frases Híbridas Inglês/Português nas Preferências**:
   - **Problema**: Substituições ingênuas de palavras isoladas geraram frases como *"Only Amigos and Grupos know I'm online"*, *"Open email notification Configurações"* e *"Changes Aplicar immediately"*.
   - **Correção**: Reescritas e retraduzidas 976 linhas para português natural brasileiro em conformidade com as regras gramaticais e de contexto.

4. **Duplicação de Seletores Fluent em Malhas e Animações**:
   - **Problema**: As linhas `import-mesh-info` e `import-animation-info` continham seletores duplicados (`{ $models -> { $models ->`), quebrando o parser do `fluent-syntax`.
   - **Correção**: Sintaxe Fluent restabelecida perfeitamente em ambas as entradas.

5. **Falha de Escape `\u` em Regex de Tradução**:
   - **Problema**: Strings contendo quebras de linha (`\u000A`) disparavam erro de template no `re.sub` do Python.
   - **Correção**: Substituído por chamada via função lambda `patt.sub(lambda _, p=ph: p, restored)`.

6. **Esgotamento de Memória (OOM) Durante a Compilação**:
   - **Problema**: Compilações paralelas de crates pesadas (`aurora-viewer`, `wgpu`, shaders) podiam estourar a RAM do host (`STATUS_STACK_BUFFER_OVERRUN`).
   - **Correção**: Scripts de release e ferramentas configurados para limitar estritamente a concorrência a 2 threads (`cargo build -j 2`).

7. **Suporte a Builds Locais nos Scripts de Empacotamento**:
   - **Problema**: O script `scripts/build-release.ps1` buscava sempre `origin/main` remoto, impedindo compilar commits locais antes do push.
   - **Correção**: Adicionada a flag `-Local` ao `scripts/build-release.ps1`.

---

### 📦 Artefatos de Release Gerados

- **Executável Standalone**: `RELEASE/SunSet-Viewer/SunSet-Viewer.exe` (177 MB, otimizado para produção).
- **Pacote Portátil**: `RELEASE/SunSet-Viewer-v0.9.3-windows-x64.zip` (73.7 MB, pronto para distribuição).
- **Símbolos de Depuração**: `RELEASE/src/target/release/SunSet_Viewer.pdb` (411 MB, para triagem de crashes).
- **Metadados de Instalação**: `sunset-install.json` e `version.txt` gerados e validados.

