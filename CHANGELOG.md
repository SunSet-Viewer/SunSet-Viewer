# SunSet Viewer — Registro de Mudanças (Changelog)

Todas as mudanças notáveis, novidades e correções de bugs do projeto **SunSet Viewer** estão documentadas neste arquivo.

---

## [0.9.3] — 2026-11-06

Esta versão marca a consolidação do **SunSet Viewer** como um projeto independente derivado do Aurora Viewer, incorporando as novidades do Aurora v0.9.3 (incluindo FSR 3.1), preservando o sistema proprietário de combate, introduzindo o novo sistema autônomo de auto-atualização e entregando 100% de suporte nativo ao Português do Brasil (pt-BR).

### 🚀 Principais Novidades (Features)

#### 1. Desvinculação e Nova Identidade (SunSet Viewer)
- **Repositório Independente**: Todas as URLs, feeds de atualização, issue trackers e links de documentação foram remapeados para `https://github.com/Adriano-Severino/SunSet-Viewer`.
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
  - Configurado para consultar diretamente a API de releases de `Adriano-Severino/SunSet-Viewer` (`crates/aurora-updater/src/feed.rs`).
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
