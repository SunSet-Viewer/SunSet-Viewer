# Controle de Movimento & Salto Instantâneo (Dash) — SunSet Viewer

O **SunSet Viewer** conta com um sistema de controle de movimento e dash ultra-rápido projetado para mobilidade extrema, navegação ágil e combate no Second Life.

---

## 🚀 Como Acessar a Interface

Você pode abrir a janela flutuante **Movimento & Dash** de duas maneiras:
1. **Menu Principal**: Acesse `Mundo` › `Controle de Movimento & Dash`.
2. **Chat Local**: Digite o comando `/dash menu`.

---

## ⚡ Velocidade do Avatar (Multiplicador de Movimento)

O controle de velocidade substitui ou aprimora a movimentação nativa do avatar através de integração direta com o motor de física do simulador (**Aurora LSL Bridge v1.42**), permitindo que você corra e voe com velocidades até **100x** maiores que a velocidade padrão do Second Life.

### Controles Disponíveis:
- **Botões Rápidos**: `1x`, `2x`, `5x`, `10x`, `20x`, `50x`, `100x`.
- **Ajuste Fino (Custom)**: Campo com arraste decimal permitindo qualquer valor fracionário entre `0.1x` e `100.0x`.
- **Toggle Mestre (Sistema Ativado)**: Quando desmarcado, reverte instantaneamente toda a física, amortecimentos e velocidade ao comportamento nativo 1.0x do Second Life.

### 🛠️ Correções e Melhorias de Arquitetura (v0.9.7):
- **Correção da barreira de 5x**: Anteriormente, velocidades acima de 5x não aumentavam porque a telemetria do protocolo de rede do bridge (`Ack::parse`) rejeitava pacotes com alvos superiores a 2.565 cm/s (`5.13 m/s * 5x`). O parser foi expandido para suportar até 1.000.000 cm/s.
- **Aceleração e Empuxo Dinâmico (Havok)**: O script `bridge.lsl` (atualizado para v1.42) agora recalcula a aceleração máxima e rampa de força proporcionalmente à velocidade solicitada (de até 6.000 m/s²), permitindo vencer com facilidade o arrasto por atrito do chão imposto pela física de avatares do Havok.
- **Tolerância a Relevo e Desníveis**: Rampas de subida e pequenas elevações do terreno causavam interrupções bruscas (`clearRun`) por picos de velocidade no eixo Z. A tolerância de inclinação agora escala suavemente com a velocidade para evitar travamentos acidentais.
- **Suporte Expandido em Voo**: O multiplicador de voo assistido (`fly_multiplier`) agora atinge até 100x em paridade total com a velocidade terrestre.
- **Auto-Update do Bridge**: Ao logar com o viewer v0.9.7, o sistema detecta se o anexo do bridge é inferior à versão 1.42 e o atualiza silenciosamente no inventário.

---

## 💨 Salto Instantâneo (Dash)

Permite projetar seu avatar instantaneamente para a frente ou na direção mirada na câmera.

### Opções do Painel:
- **Distâncias Pré-definidas**: `25m`, `50m`, `100m`, `200m`, `500m`.
- **Distância Customizada**: Permite definir qualquer distância entre `1 m` e `8.192 m`.
- **Botão "Executar Salto Agora"**: Realiza o dash com a distância configurada na direção em que o avatar está virado.

### Comandos de Chat:
- `/dash` — Executa um dash com a distância atualmente selecionada no painel.
- `/dash <metros>` — Executa um dash com a distância especificada (exemplo: `/dash 75` ou `/dash 300`).

---

## 🎯 Atalhos em Modo Mouselook (Primeira Pessoa)

Quando você entra no modo Mouselook (primeira pessoa), recursos exclusivos de combate e navegação são ativados:
- **Tecla `E`**: Dispara um dash instantâneo na mira exata do crosshair do mouse.
- **Botão do Meio do Mouse (Scroll Click)**: Dispara um dash na direção da mira.
- Ambos os atalhos podem ser ativados ou desativados independentemente pelos checkboxes no rodapé do painel.

---

## 🔒 Segurança e Compatibilidade
- O sistema respeita as regras de assentamento (o dash e a velocidade turbo são desativados enquanto sentado).
- A proteção anti-teleporte inválido assegura que o avatar não seja projetado em regiões desconectadas.
- Todas as configurações e atalhos são salvos automaticamente no seu perfil (`settings.json`).

