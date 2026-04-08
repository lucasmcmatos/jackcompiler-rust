# Jack Compiler (Rust)

## Autor (Grupo)

- **Nome:** Lucas Martins Campos Matos  
- **Matrícula:** 20250013668  

## Linguagem utilizada

O projeto foi desenvolvido em: **Rust**

Ferramentas utilizadas:
- `cargo` para compilação e execução;
- biblioteca padrão do Rust (`std`) para leitura e escrita de arquivos.

## Descrição

Este repositório contém a implementação de um **compilador para a linguagem Jack**, desenvolvido como parte do estudo de construção de compiladores. O projeto segue uma arquitetura modular, onde cada etapa do processo de compilação é implementada de forma independente e evolutiva. Atualmente, o projeto contempla o módulo de **análise léxica (tokenizer)**, responsável por:

- Ler arquivos `.jack`;
- Identificar e classificar os elementos léxicos da linguagem (tokens);
- Gerar arquivos intermediários no formato **XML (`T.xml`)**, conforme especificação do projeto.

Este módulo constitui a primeira etapa do compilador e serve de base para as próximas fases, como análise sintática e geração de código. Fluxo atual do projeto:

```text
.código Jack -> Tokenizer (Análise Léxica) -> Arquivo XML (T.xml)
```

## Execução do módulo "Analisador Léxico"

### Pré-requisitos
- Rust devidamente instalado e configurado na máquina. (https://rust-lang.org/tools/install/)

### 0. Clonar o repositório Git

A etapa 0 do processo para execução do projeto é o clone do repositório git (consideramos que você ja tem o git configurado corretamente em sua máquina).

```bash
git clone https://github.com/lucasmcmatos/jackcompiler-rust.git
```

### 1. Acessar o diretório do módulo 

O analisador léxico foi implementado como um projeto Rust independente dentro do repositório:

```bash
cd jack-lexical-analyzer
```

### 2. Construção do ambiente do projeto

O gerenciador de pacotes do Rust automaticamente gera o ambiente configurado para os projetos. Para isso: 

```bash
cargo build
```

### 3. Compilar e Executar do projeto 

Garanta que os arquivos ".jack" que deseja compilar estejam na pasta:

```bash
tests/Square/Inputs
```

Por fim basta roda o compilar e executar o código usando o gerenciador de pacotes:

```bash
cargo run
```

### 4. Verificar os resultados

Os resultados podem ser verificados acessando os arquivos T.xml gerados na pasta:

```bash
tests/Square/Outputs
```

> [!WARNING]
> **Aviso Importante:**
> Como ainda estou aprendendo a linguagem rust, por algum motivo o código roda e depois de alguns poucos segundos que os resultados aparecem na pasta, dentro do tempo para resolução da tarefa ainda não consegui corrir esse problema mas para os futuros módulos pretendo ja ter resolvido.

