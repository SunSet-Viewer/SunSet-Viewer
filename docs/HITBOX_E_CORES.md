# Hitboxes 3D e Cores Individuais de Avatares — SunSet Viewer

O **SunSet Viewer** conta com um sistema de caixas de colisão 3D (**Hitboxes**) e marcação visual individual de avatares com suporte a cores personalizadas, projetado tanto para combate competitivo (PVP/RP) quanto para identificação tática rápida no Second Life.

---

## 🎯 1. O que são os Hitboxes 3D?

Os **Hitboxes** são cubos delimitadores 3D renderizados em tempo real ao redor de cada avatar no alcance visual (*draw distance*). Eles representam com exatidão matemática o volume de colisão relatado pelo simulador para detecção de projéteis e impacto de armas.

### Como Ativar:
1. **Via Preferências**:
   - Pressione `Ctrl + P` para abrir as **Preferências**.
   - Navegue até a aba **Combate**.
   - Ative a opção **`Exibir caixas de colisão de avatares`** (*Show hitboxes*).
2. **Via Menu Superior / Barra de Ferramentas**:
   - No menu superior ou nos botões de alternância da barra de ferramentas, clique em **`Caixas de colisão`** para ligar/desligar com um clique.

---

## 🔫 2. Recursos Avançados de Combate e Mira

Na mesma página de **Preferências › Combate**, você encontra ferramentas integradas de precisão de disparo:

- **Detecção Local de Acertos (*Local Hit Detection*)**:
  - Testa em tempo real os volumes de colisão do esqueleto do avatar ao longo da linha de visão do centro da tela.
  - Desenha um marcador ciano brilhante no ponto exato de impacto onde o tiro atingiu o avatar.
- **Objetos da Cena Bloqueiam Raios (*Scene Objects Block Rays*)**:
  - Quando ativado, paredes, portas, veículos e construções interceptam o raio de tiro, evitando falsos positivos através de obstáculos sólidos.
- **Exibir Raio de Combate (*Show Raycast*)**:
  - Desenha uma linha vetorial tridimensional contínua partindo da câmera até o ponto de impacto ou alcance máximo.

---

## 🎨 3. Cores dos Hitboxes e Marcação Individual de Avatares

No SunSet Viewer, os **Hitboxes 3D não têm cores genéricas estáticas**. Eles herdam dinamicamente as cores de identificação do avatar, permitindo distinguir instantaneamente amigos, inimigos e alvos prioritários no calor da batalha.

### 🌟 Hierarquia e Precedência de Cores:
1. 🥇 **Cor Individual Marcada (*Avatar Mark*) — PRIORIDADE MÁXIMA**:
   - Se você definir uma cor específica para um avatar, o hitbox 3D dele será desenhado com essa exata cor, sobrepondo qualquer outra regra.
2. 🥈 **Conjunto de Contatos (*Contact Sets*)**:
   - Se o avatar não possuir uma cor individual, ele usará a cor do seu grupo de contatos (ex: *Aliados*, *Facção*, *Inimigos*).
3. 🥉 **Cor Padrão do Sistema**:
   - Se não pertencer a nenhum grupo, utiliza o esquema padrão do sistema (amigos em verde/azul, residentes desconhecidos, donos do terreno, Lindens, etc.).

---

## 🖱️ 4. Como Marcar uma Cor Individual para um Avatar

Você pode marcar ou trocar a cor de qualquer avatar em poucos cliques:

1. **Clique com o botão direito** diretamente no corpo do avatar no mundo 3D (ou no nome dele na janela de **Pessoas** ou no **Minimapa**).
2. Acesse o submenu **`Marcar cor`** (ícone de etiqueta preenchida).
3. Escolha uma das opções:
   - **Paleta Rápida**: Clique em qualquer um dos blocos de cores predefinidas (Vermelho, Verde, Azul, Amarelo, Laranja, Roxo, Ciano, Branco, etc.).
   - **Seletor Personalizado (RGB/HSV)**: Clique no botão de cor para abrir a paleta completa e selecionar tonalidades personalizadas com ajuste de saturação e brilho.
4. O **Hitbox 3D**, o ponto do **Minimapa** e o identificador do avatar passarão a exibir a nova cor imediatamente.

### Como Limpar a Marcação:
- **Individualmente**: No mesmo menu do avatar, clique em `Marcar cor` › **`Limpar cor`** (o avatar retorna à cor do seu Contact Set ou padrão).
- **Em Massa**: Clique com o botão direito em qualquer residente e selecione **`Limpar todas as marcas`** para restaurar todo o sistema às cores originais.

---

## 💾 5. Persistência Automática

Todas as cores individuais marcadas são salvas automaticamente no arquivo de configuração do seu perfil local (`settings.json`). Mesmo que você reinicie o SunSet Viewer ou viaje entre simuladores diferentes, suas cores marcadas permanecerão intactas.

