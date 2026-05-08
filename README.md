# 📋 Manual de Execução de Jobs — DIOT · Dataprev

> Guia técnico interativo para operação no **Workload**, desenvolvido para novos colaboradores da equipe DIOT da Dataprev.

---

## 🖥️ Sobre o Projeto

Este projeto é um **manual técnico em formato HTML estático**, moderno e intuitivo, que documenta os procedimentos operacionais para execução de jobs no sistema Workload da DIOT (Dataprev).

O material foi criado para facilitar o onboarding de novos funcionários, centralizando em um único lugar todos os fluxos de execução com passo a passo visual, modelos de mensagem para o ISTM e alertas de boas práticas.

---

## ✨ Funcionalidades

- 🔍 **Navegação por tipo de job** com menu fixo no topo
- 🃏 **Cards recolhíveis por fase** — clique para expandir ou ocultar
- 📋 **Modelos de mensagem ISTM** com botão de cópia rápida
- 🔶 **Indicadores visuais** de status (Hold, Sucesso, Erro)
- 💡 **Alertas e observações** de boas práticas em cada etapa
- 📱 **Layout responsivo** para desktop e mobile
- 🌙 **Dark mode** com design moderno

---

## 📂 Tipos de Execução Documentados

| # | Tipo | Status |
|---|------|--------|
| 01 | ⚡ Job Avulso | ✅ Documentado |
| 02 | 🔁 Job Eventual | ✅ Documentado |
| 03 | ⚙️ Job Procedure | 🚧 Em elaboração |

---

## 🗂️ Estrutura do Projeto

```
📁 manual-diot/
├── 📄 manual-diot.html        # Arquivo principal — abrir no navegador
└── 📄 README.md               # Este arquivo
```

---

## 🚀 Como Usar

### Opção 1 — Abrir localmente

1. Faça o download ou clone o repositório:
   ```bash
   git clone https://github.com/seu-usuario/manual-diot.git
   ```
2. Navegue até a pasta do projeto:
   ```bash
   cd manual-diot
   ```
3. Abra o arquivo `manual-diot.html` diretamente no navegador.

> Não requer servidor, build ou dependências externas. Funciona offline exceto pelas fontes do Google Fonts.

### Opção 2 — GitHub Pages

O manual pode ser publicado via **GitHub Pages** para acesso online pela equipe:

1. Vá em **Settings → Pages** no repositório
2. Selecione a branch `main` e a pasta `/root`
3. Clique em **Save** — o link estará disponível em instantes

---

## 📝 Fluxo de Cada Tipo de Job

### ⚡ Job Avulso
```
Monitor (Subscribe with filter)
    ↓
Define (Download → Trigger → Submit on hold)
    ↓
Monitor (Reset Definitions → Release Application)
    ↓
ISTM (Resolvido + comentário padrão)
```

### 🔁 Job Eventual
```
Monitor (Subscribe with filter + Last # of generations)
    ↓
Define (Download → Trigger → Submit on hold)
    ↓
Monitor (Verificar jobs → Release Application)
    ↓
ISTM (Resolvido + comentário padrão)
```

---

## 💬 Modelos de Mensagem ISTM

O manual inclui 3 modelos prontos com botão de cópia para cada tipo de execução:

| Situação | Código de Fechamento |
|----------|---------------------|
| 🔵 Processo em execução | `Processo em execução` |
| ✅ Finalizado sem erros | `Com sucesso` |
| ❌ Falha na execução | `Processamento com erro` |

---

## ⚠️ Observações Importantes

- Sempre marque **"Submit application on hold"** antes de acionar qualquer trigger
- No Job Avulso, preencha o **Reset Definitions** em todos os jobs antes do Release
- Em caso de dúvida ou requisição fora do padrão, **consulte o líder ou gerente da equipe** antes de prosseguir

---

## 🛠️ Tecnologias Utilizadas

- **HTML5** + **CSS3** puro — sem frameworks
- **JavaScript** vanilla — sem dependências
- Fontes via [Google Fonts](https://fonts.google.com): `Syne`, `DM Mono`, `DM Sans`

---

## 📌 Versionamento

| Versão | Descrição |
|--------|-----------|
| v1.0 | Job Avulso + modelos ISTM (Em execução, Com sucesso, Com erro) |
| v1.1 | Job Eventual implementado |
| v1.2 | Observação de dúvidas com líder/gerente adicionada em todas as seções |

---

## 👥 Equipe

**DIOT — Dataprev**
Divisão de Infraestrutura e Operações de TI

---

> 📌 *Em caso de dúvidas sobre este manual ou sobre os procedimentos descritos, procure o líder ou gerente da equipe.*
