# 📈 **TimeWise Regression – Análise de Séries Temporais com Regressão Linear Pura (Rust)**<br/>

Este projeto implementa uma ferramenta em Rust para análise de séries temporais com regressão linear pura, sem uso de crates externas. O foco é oferecer uma solução simples, eficiente e de fácil entendimento para análise e previsão de dados com tendência linear.

## 🚀 **Objetivo**<br/>
Desenvolver um módulo que:<br/>
- Importe dados de séries temporais (CSV);<br/>
- Realize análise exploratória (média, desvio padrão, etc.);<br/>
- Calcule uma regressão linear "pura" (sem bibliotecas externas);<br/>
- Avalie o modelo com métricas R² e MSE;<br/>
- Gere previsões para períodos futuros;<br/>
- Forneça saída amigável (terminal ou arquivo).<br/>

## 📌 **Justificativa**<br/>
A regressão linear é ideal para séries temporais com padrões lineares simples, como crescimento constante de valores (ex: temperatura, produção, vendas). É fácil de interpretar, rápida de executar e serve como base para modelos mais avançados.<br/>

Neste projeto, optei por uma implementação 100% em Rust para garantir:<br/>
- Performance e controle total;<br/>
- Aprendizado profundo dos fundamentos matemáticos;<br/>
- Evitar dependências externas.<br/>

## 📂 **Estrutura do Projeto**
**rust_portfolio/**<br/>
├── Cargo.toml<br/>
├── README.md<br/>
├── src/<br/>
│   ├── main.rs<br/>
│   ├── helpers/<br/>
│       ├── file_validation   # Validação dos arquivos<br/>
│       └── mod<br/>
│   ├── load_data             # Leitura de CSV<br/>
│   ├── regression            # Regressão linear e métricas<br/>
│   ├── prediction            # Função de previsão<br/>
│   └── lib                   # Arquivo de Testes<br/>
├── examples/<br/>
    └── mocked_data.csv       # Dados mockados<br/>

## 🛠️ **Como rodar o projeto**<br/>

**Pré-requisitos**<br/>
- Ter o Rust instalado: https://www.rust-lang.org/tools/install<br/>

**Passos:**<br/>

**1. Clone o projeto**<br/>
- `git clone https://github.com/Salutx/rust-portfolio.git`<br/>
- `cd rust-portfolio`<br/>

**2. Compile o projeto**<br/>
- `cargo build`<br/>

**3. Execute com os dados mockados**<br/>
- `cargo run`<br/>

## 📊 Exemplo de dados (`mocked_data.csv`)<br/>

day,value<br/>
1,15.0<br/>
2,15.5<br/>
3,16.1<br/>
4,16.8<br/>
5,17.2<br/>
6,17.6<br/>
7,18.3<br/>
8,18.9<br/>
9,19.4<br/>
10,20.0<br/>

## 🧪 Testes<br/>

Para rodar os testes unitários:
- `cargo test`

🎥 Vídeo Explicativo
📺 Acesse o vídeo explicando o sistema e as decisões de projeto:🔍 https://www.youtube.com/watch?v=3c7X11I5kqU

🧠 Autor
Desenvolvido por Salutx (Lucas Matos) 📧 lucas.costa.92648@a.fecaf.com.br

