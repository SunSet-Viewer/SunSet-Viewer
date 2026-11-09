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

## 🛠️ 3. Outras Formas de Alterar a Distância de Visão

Além do comando `/dd` no chat, você pode ajustar o alcance de visão visualmente:

1. **Preferências Rápidas (*Quick Prefs*)**:
   - Abra o menu de Preferências Rápidas na barra lateral ou inferior.
   - Utilize o controle deslizante **Distância de visão** (*Draw distance*) para ajustar em tempo real com pré-visualização imediata.
2. **Janela de Preferências**:
   - Pressione `Ctrl + P` › **Gráficos** › seção **Qualidade de renderização**.

---

## 💡 4. Dicas Táticas e Boas Práticas

- **Para Combate e PVP**: Mantenha entre `96m` e `192m` (`dd 128`). Isso foca a placa de vídeo em renderizar os hitboxes e avatares próximos com máxima fluidez e latência mínima.
- **Para Fotografia de Paisagens**: Use `dd 3000` ou `dd 5000` para capturar cordilheiras, mares e cidades distantes sem o efeito de névoa de corte artificial.
- **Ao se teletransportar para locais pesados**: Se a região demorar para firmar as texturas, envie rapidamente um `dd 64` e, após carregar o seu redor, volte para `dd 256`.

