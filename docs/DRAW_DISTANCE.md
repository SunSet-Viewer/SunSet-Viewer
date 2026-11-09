# Distância de Visão (Draw Distance) e Comando `dd` — SunSet Viewer

O **SunSet Viewer** conta com um sistema de alcance visual de renderização estendido, permitindo ajustar a distância de visão (*Draw Distance*) de **32 metros** até **8.192 metros**, com suporte a comandos rápidos de chat idênticos aos do Firestorm e controle dinâmico de desempenho.

---

## ⚡ 1. O Comando de Chat `dd` (Estilo Firestorm)

Assim como no Firestorm, você pode alterar a distância de visão instantaneamente digitando diretamente na barra de **Chat Local**:

```text
dd <distância>
```

### Exemplos Práticos:

| Comando | Efeito | Cenário Recomendado |
| :--- | :--- | :--- |
| **`dd 128`** | Define o alcance para **128 metros** | Ambientes fechados, lojas ou eventos lotados com máximo FPS. |
| **`dd 256`** | Define o alcance para **256 metros** | Padrão equilibrado para exploração urbana e navegação geral. |
| **`dd 512`** | Define o alcance para **512 metros** | Exploração de grandes ilhas e navegação marítima. |
| **`dd 1024`** | Define o alcance para **1.024 metros** (1 km) | Visão ampla de múltiplos sims vizinhos. |
| **`dd 5000`** | Define o alcance para **5.000 metros** (5 km) | Fotografia panorâmica de grandes extensões e voo em alta altitude. |
| **`dd 8192`** | Define o alcance para **8.192 metros** (8 km) | **Limite máximo desbloqueado** para vistas de continentes inteiros. |

### Ajustes Relativos (Incremento / Decremento):
Você também pode somar ou subtrair metros à sua distância atual sem precisar digitar o número absoluto:
- **`dd +100`**: Aumenta a distância atual em 100 metros.
- **`dd +500`**: Aumenta a distância atual em 500 metros.
- **`dd -50`**: Reduz a distância atual em 50 metros.
- **`dd -200`**: Reduz a distância atual em 200 metros.

---

## 🚀 2. Por que o SunSet Viewer permite distâncias como 5000m ou 8192m?

Nos visualizadores clássicos baseados em OpenGL, definir distâncias superiores a 512m ou 1024m causava congelamentos severos de tela (*freezes*), queda brusca de FPS ou até falhas de falta de memória (Out Of Memory).

No **SunSet Viewer**, o pipeline de renderização foi construído em **Rust e Vulkan**, trazendo vantagens exclusivas:
1. **Decodificação em Threads Assíncronas**: Texturas e malhas de regiões distantes são baixadas e decodificadas em paralelo sem travar a thread gráfica principal.
2. **Occlusion Culling Geométrico**: Objetos escondidos atrás de colinas ou construções não sobrecarregam a placa de vídeo, mesmo com o horizonte a 5 km de distância.
3. **Protocolo `AgentUpdate` Otimizado**: O viewer envia a solicitação da nova distância ao simulador do Second Life sem sobrecarregar a largura de banda de rede configurada.

---

## 🛠️ 3. SpeedRez (Distância de Visão Progressiva — Novo na v0.9.7)

A partir da versão **v0.9.7** (integração com Aurora v0.9.4), o **SunSet Viewer** conta com o sistema **SpeedRez** (*Progressive Draw Distance / `FSRenderFarClipStepping`*), totalmente harmonizado com o teto estendido de **8.192 metros**:

1. **Como funciona**:
   - No login ou ao se teletransportar entre regiões, o visualizador inicia o alcance de visão em **32 metros** para carregar imediatamente os objetos e avatares ao seu redor sem travamentos.
   - Em seguida, a distância de visão **dobra progressivamente** (`32m → 64m → 128m → 256m → 512m...`) a cada intervalo de tempo até atingir exatamente a sua distância alvo configurada — inclusive alvos de **5.000m** (`dd 5000`) ou **8.192m** (`dd 8192`).
2. **Onde configurar**:
   - **Ativar/Desativar**: `Preferências (Ctrl+P)` › `Gráficos` › seção `Distância e detalhe` › **`Distância de visão progressiva`**.
   - **Intervalo por passo**: Nas **Configurações de Depuração** (`Ctrl+Alt+Shift+S`), procure por `draw_distance_step_interval` (padrão: `20` segundos por passo, ajustável de `1` a `60` segundos).
3. **Descoberta Dinâmica de Regiões Vizinhas**:
   - O visualizador descobre e mantém conectadas até **3 anéis de regiões vizinhas** de acordo com o seu alcance de visão (`draw_distance`), garantindo que horizontes distantes apareçam completos.

---

## 🎚️ 4. Outras Formas de Alterar a Distância de Visão

Além do comando `dd` no chat, você pode ajustar o alcance de visão visualmente até **8.192 metros** sem que o valor seja truncado para 512m:

1. **Preferências Rápidas (*Quick Prefs*)**:
   - Abra o menu de Preferências Rápidas na barra de ferramentas.
   - Utilize o controle deslizante **Distância de visão** (*Draw distance*, de `32 m` a `8.192 m`) para ajustar em tempo real.
2. **Janela de Preferências**:
   - Pressione `Ctrl + P` › **Gráficos** › seção **Distância e detalhe** (`32 m` a `8.192 m`).

---

## 💡 5. Dicas Táticas e Boas Práticas

- **Para Combate e PVP**: Mantenha entre `96m` e `192m` (`dd 128`). Isso foca a placa de vídeo em renderizar os hitboxes e avatares próximos com máxima fluidez e latência mínima.
- **Para Fotografia de Paisagens**: Use `dd 3000` ou `dd 5000` para capturar cordilheiras, mares e cidades distantes sem o efeito de névoa de corte artificial.
- **Ao se teletransportar para locais pesados**: Se a região demorar para firmar as texturas, envie rapidamente um `dd 64` e, após carregar o seu redor, volte para `dd 256`.

