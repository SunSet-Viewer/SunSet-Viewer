<div align="center">

<img src="assets/branding/sunset-logo.svg" alt="SunSet Viewer" width="180">

# SunSet Viewer

**Um visualizador moderno para o Second Life, desenvolvido em Rust.**<br>
Renderizador Vulkan, interface limpa e o Second Life que você já conhece.

[ **English** ](README.md) · [ **Português (Brasil)** ](README.pt-BR.md)

[![Download](docs/readme/button-download.svg)](https://github.com/SunSet-Viewer/SunSet-Viewer/releases/latest)&nbsp;
[![Download para Linux](docs/readme/button-download-linux.svg)](docs/LINUX.md)&nbsp;
[![Download para macOS (experimental)](docs/readme/button-download-macos.svg)](docs/MACOS.md)&nbsp;

[![Release](https://img.shields.io/github/v/release/SunSet-Viewer/SunSet-Viewer?include_prereleases&color=8B5CF6)](https://github.com/SunSet-Viewer/SunSet-Viewer/releases)
[![Licença: GPLv3](https://img.shields.io/badge/license-GPLv3-4F46E5)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-5EEAD4?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![Plataforma](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-070B1F)](#compilação)

<sub>O SunSet Viewer é uma versão modificada e mantida de forma independente com base no <a href="https://github.com/Aurora-Viewer/Aurora-Viewer">Aurora Viewer</a>. Não é uma versão oficial do Aurora nem da Linden Lab, e nenhum dos dois projetos o endossa ou apoia diretamente.</sub>

[Por que o SunSet](#por-que-o-sunset) · [Destaques Exclusivos](#destaques-exclusivos-do-sunset-) · [Criadores](#ferramentas-para-criadores) · [Desempenho](#desempenho) · [Recursos](#o-que-funciona-hoje) · [Como Usar](#como-usar-o-sunset) · [Compilação](#compilação) · [Contribuindo](#contribuindo)

</div>

**Altas taxas de quadros e baixo frame time, teletransportes e carregamentos ultrarrápidos, graças a tecnologias modernas como Vulkan e Rust.**  
<img width="1920" height="1032" alt="Visualizador do Second Life em um jardim japonês, com janelas de Pessoas, Controles, Lugares, Inventário, Desempenho e Conversas abertas" src="docs/screenshots/world.webp" />  

**Todas as ferramentas que você precisa para aproveitar o Second Life da melhor forma: modo Foto profissional, Auto Hover Height e muita qualidade de vida!**  
<img width="1920" height="1029" alt="Construção no SunSet: painel Transformar ao lado de uma caixa selecionada, com janelas Sobre o terreno, Região/Propriedade, perfil e voz" src="docs/screenshots/build-land.webp" />  

**Interface moderna e botões de alternância para trazer de volta a interface Clássica nos módulos que você preferir!**  
<img width="1116" height="717" alt="Página SunSet das Preferências, com escolha entre SunSet ou Clássico para cada janela reformulada" src="docs/screenshots/preferences.webp" />

**Nova experiência de login com suporte a múltiplas contas de avatar.**  
<img width="1920" height="1032" alt="Tela de login do SunSet com contas de avatar lado a lado e notícias em tempo real" src="docs/screenshots/login.webp" />  

<details>
<summary><b>Mais capturas de tela</b>: conversas, notificações, inventário, lugares, pesquisa e monitor de recursos</summary>
<br>
<table>
  <tr>
    <td width="50%" valign="top">
      <img src="docs/screenshots/conversations.webp" alt="Janela de Conversas: chat privado com balões e oferta de teletransporte em formato de cartão"><br>
      <b>Conversas</b> estilo mensageiro: balões, fotos e ofertas de teletransporte ou presentes diretamente na conversa.
    </td>
    <td width="50%" valign="top">
      <img src="docs/screenshots/notification-center.webp" alt="Central de notificações desenrolada a partir do sino, com oferta de teletransporte e aviso do simulador"><br>
      <b>Central de Notificações</b> acessível pelo sino, mantendo o histórico de tudo o que você perdeu.
    </td>
  </tr>
  <tr>
    <td valign="top">
      <img src="docs/screenshots/inventory.webp" alt="Inventário expandido como um explorador de arquivos, com pastas, permissões e painel de detalhes"><br>
      <b>Inventário</b> que se expande em um explorador completo: pastas, detalhes, filtros e permissões de cada item em um relance.
    </td>
    <td valign="top">
      <img src="docs/screenshots/places.webp" alt="Janela de Lugares: favoritos e histórico em uma lista única, com pré-visualização do local selecionado"><br>
      <b>Lugares</b>: favoritos, marcos (landmarks) e histórico em uma lista só, com pré-visualização de cada local.
    </td>
  </tr>
  <tr>
    <td valign="top">
      <img src="docs/screenshots/search.webp" alt="Janela de Pesquisa: campo único, filtros rápidos e detalhes ao lado dos resultados"><br>
      <b>Pesquisa Universal</b> para residentes, lugares, eventos, grupos e classificados, com os detalhes ao lado dos resultados.
    </td>
    <td valign="top">
      <img src="docs/screenshots/performance.webp" alt="Monitor de recursos: processador, placa de vídeo, memória, rede e disco, com tempo de quadro dividido por tarefa"><br>
      <b>Monitor de Recursos</b> que mostra para onde vai cada milissegundo, megabyte e kilobit.
    </td>
  </tr>
</table>
</details>

O SunSet Viewer é um cliente modificado para Second Life baseado no [Aurora Viewer](https://github.com/Aurora-Viewer/Aurora-Viewer), equipado com um renderizador moderno de GPU (wgpu sobre Vulkan) e uma interface limpa. Também contém código derivado do visualizador oficial do Second Life e do Firestorm. A autoria original, licenças e a relação upstream estão documentadas em [NOTICE.md](NOTICE.md).

> **Status:** Desenvolvimento ativo (0.x). Perfeitamente utilizável para explorar, conversar, construir e testar.

---

## Por que o SunSet

<table>
  <tr>
    <td width="25%" align="center" valign="top">
      <h3>≈ 5×</h3>
      mais FPS do que um visualizador clássico em lojas movimentadas
    </td>
    <td width="25%" align="center" valign="top">
      <h3>≈ ½</h3>
      da memória de vídeo (VRAM) para as mesmas texturas
    </td>
    <td width="25%" align="center" valign="top">
      <h3>Instantâneo</h3>
      teletransportes dentro da mesma região, sem tela de carregamento
    </td>
    <td width="25%" align="center" valign="top">
      <h3>Vulkan</h3>
      renderizador de última geração projetado para placas de vídeo atuais
    </td>
  </tr>
</table>

### Destaques Exclusivos do SunSet <img src="docs/readme/pill-new.svg" alt="Novo" height="18" align="absmiddle">

<table>
  <tr>
    <td width="50%" valign="top">
      <h3>⚔️ Módulo de Combate Integrado</h3>
      Sistema completo de combate em tempo real (<code>scene::combat</code>) com raycasting a partir do centro da tela, hitboxes detalhadas por articulação do avatar, filtragem de colisões de armas e preferências exclusivas de combate.
    </td>
    <td width="50%" valign="top">
      <h3>⚡ Upscaling AMD FSR 3.1</h3>
      AMD FidelityFX Super Resolution 3.1 nativo utilizando compute shaders WGSL. Renderização nítida com alta taxa de quadros para telas exigentes em 1440p e 4K em qualquer placa de vídeo moderna.
    </td>
  </tr>
  <tr>
    <td valign="top">
      <h3>🇧🇷 Localização 100% Nativa em Português (pt-BR)</h3>
      Tradução integral e criteriosa de mais de 12.650 termos de interface, menus e mensagens de sistema, adotando o jargão consagrado do Second Life no Brasil (ex: <i>"Em uso"</i>, <i>"Desanexar de você"</i>, <i>"Sempre correr"</i>).
    </td>
    <td valign="top">
      <h3>🔄 Atualizações Automáticas em Um Clique</h3>
      Verificação e download atômico em segundo plano com verificação criptográfica SHA-256 e proteção de restauração, integrado diretamente aos lançamentos do repositório no GitHub.
    </td>
  </tr>
</table>

---

### Rápido e Fluido

- **Centenas de quadros por segundo em locais lotados.** Em lojas movimentadas com cerca de 16.000 objetos, o SunSet alcança em torno de 340 a 386 FPS onde visualizadores clássicos rodam a 68 FPS no mesmo computador e no mesmo local ([veja a medição](#desempenho)).
- **A imagem continua em movimento ao chegar.** Texturas e malhas (meshes) são decodificadas em threads secundárias e gravadas diretamente na memória da GPU, evitando congelamentos ao aterrissar em regiões movimentadas.
- **Lugares conhecidos carregam na hora.** As texturas ficam em cache no disco prontas para a placa de vídeo: voltando a um local já visitado, nada precisa ser decodificado novamente.
- **Aproximadamente metade da memória de vídeo para texturas**, sem perda perceptível aos olhos: cada textura comprimida em BC7 é verificada antes de ser mantida.
- **Teletransportes instantâneos dentro da região.** Teletransportes para outros simuladores exibem o último quadro desfocado com progresso real passo a passo.
- **A placa de vídeo finalmente faz o trabalho pesado.** A GPU decide por si mesma o que está visível (culling em compute shaders) e desenha milhares de objetos em pouquíssimas chamadas, enquanto uma thread dedicada de renderização mantém o fluxo constante de quadros.
- **Conexão resiliente**: instabilidades rápidas de rede não derrubam mais a sua sessão (o SunSet reconecta você exatamente de onde parou).
- **Inventário completo carregado em segundo plano**: desde o login, suas pastas já estão prontas quando você as abre.
- **Seleção automática da melhor GPU em notebooks**: em PCs com GPU integrada e placa dedicada, o SunSet escolhe automaticamente a dedicada.
- **AMD FSR 3.1 nativo**: reduza a resolução interna da cena de 67% até 33% e reconstrua a imagem nítida com anti-aliasing avançado para ganhar ainda mais FPS (Preferências › Gráficos › Anti-aliasing).

---

### Imagem Mais Nítida e Rica

- **Oclusão de Ambiente Moderna (GTAO)**: sombreamento suave e realista em cantos, sob móveis e ao redor dos avatares, impedindo reflexos em locais onde a luz não chega.
- **Sombras de Contato do Sol**: pequenas sombras que firmam pés, roupas e objetos no chão, onde os mapas de sombra tradicionais costumam falhar.
- **Anti-aliasing Especular**: superfícies brilhantes não cintilam mais conforme a câmera se desloca.
- **Movimento sem Borrões**: o TAA acompanha o vetor de movimento de cada pixel, garantindo nitidez total em avatares e objetos que se movem.
- **Céu Noturno Astronômico Real**: as 8.920 estrelas visíveis a olho nu posicionadas em seus locais e cores verdadeiros, com a Via Láctea girando de acordo com o horário do dia e a estação do ano.

---

### O Second Life que Você Conhece

- **Mesmo visual, compatibilidade total**: materiais legados (Blinn-Phong) e modernos (PBR), céus EEP, sombras, sondas de reflexão, espelhos, água, brilho e partículas.
- **Voz, Mensagens Instantâneas (IM), Grupos e Notificações**: histórico de conversas pesquisável no disco, compatível com os arquivos de chats de visualizadores anteriores. A voz inclui cancelamento de ruído, eliminação de eco e controle automático de ganho.
- **Edição de Aparência e Roupas**: 82 controles corporais e faciais com pré-visualização detalhada; ajuste automático de altura dos pés (Auto Hover Height) para manter o avatar perfeitamente apoiado no solo.
- **RLVa Completo**: compatibilidade com restrições e comandos utilizados por roupas, mobílias e coleiras roteirizadas (RLV 3.4.3 / RLVa 2.4.2), desativado por padrão em Preferências › RLVa.
- **Suporte a OpenSim e Hypergrid**: gerenciador de grades integrado, suporte a regiões grandes acima de 256m e compatibilidade com moedas locais.

---

### Modo Foto Profissional

<img width="1920" height="1025" alt="Visor de fotos profissional com regra dos terços, foco automático facial e controles manuais" src="docs/screenshots/viewfinder.webp" />

- **Visor de Câmera Completo (Viewfinder)**: sobreposição de câmera profissional em tela cheia com alternância entre níveis de detalhes pelo atalho `I`.
- **Foco Automático Facial**: modos AF-S, AF-C e foco manual. Em avatares, o ponto de foco trava nos olhos e acompanha os movimentos com confirmação sonora.
- **Anéis Virtuais de Lente**: controle de distância focal, abertura do diafragma (abertura) e exposição pela roda do mouse ou arrasto.
- **Profundidade de Campo Real (DoF)** com *focus peaking* manual para ver com precisão matemática o que está nítido.
- **Resoluções até 8K**: presets prontos em PNG, JPEG e BMP, para salvar em disco, enviar para o inventário ou publicar no feed.
- **Isolamento de Objeto / Avatar**: tire fotos do seu avatar ou de amigos com fundo totalmente transparente (canal alfa limpo) para criar cartazes e montagens sem esforço.
- **Filtros e LUTs 3D**: 30 filtros consagrados do visualizador clássico, 36 efeitos modernos embutidos (grão de filme, teal & orange, visual analógico, falha/glitch) e importação direta de arquivos `.cube` do Photoshop, Lightroom ou DaVinci Resolve.

---

### Ferramentas para Criadores

- **LiveMesh para Blender**: veja suas malhas do Blender no visualizador em tempo real enquanto modela, sem precisar enviar nenhum arquivo. Malhas com rigging acompanham o seu avatar; objetos estáticos viram modelos locais manipuláveis ([guia do LiveMesh](docs/BLENDER_LIVE.md)).
- **Poser Integrado**: crie poses para fotos para você, seus animeshes ou amigos que aceitarem o convite, articulação por articulação, com biblioteca de poses, exportação BVH e importação de animação ([guia do Poser](docs/POSER.md)).
- **Bancada de Trabalho IK**: manipulação de cinemática inversa com alvos fixos de contato para criar poses e animações precisas ([guia de IK](docs/IK_EDIT.md)).
- **Edição de Prims sem Complicação**: editor compacto, pré-visualização ao vivo do prim antes de criar, alinhamento magnético contra superfícies (tecla `P`) e busca por área (*Area Search*).
- **Editor de Partículas**: crie e ajuste sistemas de partículas visualmente no prim selecionado com geração automática do código LSL correspondente.

---

### Desempenho

<table>
  <tr>
    <td width="50%" align="center" valign="top">
      <img src="docs/screenshots/compare-firestorm.png" alt="Janela de estatísticas do Legacy Viewer em loja movimentada: 68 fps"><br>
      <b>Legacy Viewer</b> · 68 fps
    </td>
    <td width="50%" align="center" valign="top">
      <img src="docs/screenshots/compare-aurora.png" alt="Janela de estatísticas do Aurora upstream na mesma loja: 386 fps"><br>
      <b>Aurora upstream / SunSet base</b> · 386 fps
    </td>
  </tr>
</table>

Medição realizada em loja movimentada com cerca de 16.000 objetos no mesmo computador (RTX 4090) e mesmo local.

---

### O que Funciona Hoje

- **Módulo de Combate (Exclusivo SunSet)** — combate em tempo real (`scene::combat`), raycasting a partir da mira da tela, hitboxes detalhadas por articulação, testes locais de colisão e tela dedicada em Preferências.
- **Upscaling AMD FSR 3.1** — renderização de alta resolução com excelente desempenho utilizando compute shaders WGSL.
- **Localização 100% pt-BR** — tradução nativa em Português do Brasil com termos autênticos da comunidade Second Life.
- **Atualizador Automático** — verificação e instalação atômica diretamente dos lançamentos no GitHub.
- **Renderização Vulkan** — pipeline bindless, multi-draw indirect, GTAO, sombras de contato, materiais PBR e legados.
- **Avatares e Bento** — animações suaves, bakes de servidor, roupas em camadas, Quickjump e reset de esqueleto.
- **Modo Foto** — visor de alta precisão, foco automático nos olhos, profundidade de campo, exportação até 8K e suporte a LUTs.
- **Áudio e Voz WebRTC** — voz espacial próxima, cancelamento de ruído, equalização e chamadas de conferência.
- **Inventário em Explorador** — exibição em árvore ou explorador amplo com detalhes, direitos de permissão CMT e filtros rápidos.

---

## Como Usar o SunSet

### Contas e Login
A tela inicial exibe suas contas salvas em cartões com a foto do último local visitado e a grade escolhida. Basta clicar no cartão para entrar. O gerenciador de contas permite organizar a ordem dos cartões, esquecer senhas ou limpar autorizações 2FA.

### Idioma
O SunSet Viewer detecta automaticamente o idioma do seu sistema operacional. Para escolher manualmente:
1. Abra **Preferências › Interface › Idioma**.
2. Selecione **Português do Brasil (`pt-BR`)**, **Inglês (`en`)**, **Francês (`fr`)** ou **Japonês (`ja`)**.
3. A alteração é aplicada instantaneamente, sem necessidade de reiniciar.

### Ajuste de Altura do Avatar (Hover Height)
Abra **Aparência › Altura de flutuação (Hover Height)** para ajustar a altura dos pés em relação ao chão. A opção automática calcula a altura ideal a partir dos calçados anexados.

### Notícias na Tela de Login
O painel lateral de notícias traz os anúncios e notas de lançamento oficiais do SunSet Viewer diretamente da branch `main` do GitHub, mantendo você sempre informado sobre melhorias, novidades e atualizações.

---

## Compilação

**Requisitos:** Windows 10/11 (ou Linux/macOS), [Rust](https://rustup.rs) (versão estável configurada em `rust-toolchain.toml`) e placa de vídeo compatível com Vulkan.

### Compilando no Windows:

```powershell
git clone https://github.com/SunSet-Viewer/SunSet-Viewer
cd SunSet-Viewer
cargo build --release -p aurora-viewer
.\target\release\SunSet-Viewer.exe
```

Para gerar pacotes portáteis oficiais prontos para distribuição com metadados de instalação e versionamento:
```powershell
$env:AURORA_VERSION_TAG = "v0.9.4"
cargo build --release -p aurora-viewer
powershell -ExecutionPolicy Bypass -File "scripts\package.ps1" -Exe "target\release\SunSet-Viewer.exe" -Out "RELEASE\SunSet-Viewer" -Source "."
```

---

## Contribuindo

Ideias, sugestões e relatórios de problemas são muito bem-vindos na nossa página de [Issues no GitHub](https://github.com/SunSet-Viewer/SunSet-Viewer/issues).

Documentação técnica complementar:
- [MOVEMENT_DASH.md](docs/MOVEMENT_DASH.md) — Controle de Movimento e Dash Instantâneo (velocidades até 100x e atalhos).
- [DRAW_DISTANCE.md](docs/DRAW_DISTANCE.md) — Distância de Visão estendida (comando `dd 5000` até 8.192m, estilo Firestorm).
- [HITBOX_E_CORES.md](docs/HITBOX_E_CORES.md) — Caixas de colisão 3D (Hitboxes) e marcação de cores individuais para avatares.
- [GUIA_DE_LANCAMENTO_E_NOTICIAS.md](docs/GUIA_DE_LANCAMENTO_E_NOTICIAS.md) — Regras de publicação de releases e notícias da tela de login.
- [BRANDING.md](docs/BRANDING.md) — Paleta de cores, ícones e regras de interface.
- [BLENDER_LIVE.md](docs/BLENDER_LIVE.md) — Integração em tempo real com o Blender.
- [POSER.md](docs/POSER.md) — Guia do editor de poses para fotos.
- [RLVA.md](docs/RLVA.md) — Especificação e comandos RLVa.
- [TESTING.md](docs/TESTING.md) — Variáveis e cenários de testes offline.

---

## Licença e Atribuição Legal

O SunSet Viewer é um software livre distribuído sob a licença **[GNU General Public License v3.0 (GPL-3.0)](LICENSE)**.

- O SunSet Viewer é uma modificação mantida de forma independente, derivada do projeto de código aberto **Aurora Viewer**.
- Partes do código são originárias ou portadas do cliente oficial do **Second Life** e do **Firestorm** (originalmente sob LGPL 2.1). Todos os autores e licenças de componentes de terceiros estão devidamente creditados em [NOTICE.md](NOTICE.md).
- Este projeto não possui afiliação oficial com a Linden Lab, com a equipe do Firestorm nem com os mantenedores do Aurora Viewer.
- Second Life® é uma marca registrada da Linden Research, Inc.
