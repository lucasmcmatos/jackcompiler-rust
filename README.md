# Jack Compiler (Rust)

## Autor (Grupo)

- **Nome:** Lucas Martins Campos Matos
- **Matrícula:** 20250013668

## Linguagem utilizada

O projeto foi desenvolvido em: **Rust**

Ferramentas utilizadas:
- `cargo` para compilação e execução;
- biblioteca padrão do Rust (`std`) para leitura e escrita de arquivos;
- `diff -w` para validação dos XMLs gerados.

## Descrição

Este repositório contém a implementação de um **compilador para a linguagem Jack**, desenvolvido como parte do estudo de construção de compiladores. O projeto segue uma arquitetura modular, onde cada etapa do processo de compilação é implementada de forma independente e evolutiva.

Atualmente, o projeto contempla dois módulos:

- **Analisador Léxico (`jack-lexical-analyzer`)**: lê arquivos `.jack`, identifica tokens e gera arquivos XML léxicos `*T.xml`.
- **Analisador Sintático (`jack-syntactic-analyzer`)**: lê os XMLs léxicos `*T.xml`, verifica a estrutura sintática da linguagem Jack e gera XMLs sintáticos `*P.xml`.

Fluxo atual do projeto:

```text
código Jack (.jack)
        -> Analisador Léxico
        -> XML de tokens (*T.xml)
        -> Analisador Sintático
        -> XML sintático (*P.xml)
```

## Estrutura do repositório

```text
jackcompiler-rust/
  jack-lexical-analyzer/     # Projeto Cargo do scanner/tokenizer
  jack-syntactic-analyzer/   # Projeto Cargo do parser
  square-tests/
    input/                   # Arquivos .jack usados como entrada
    output/                  # XMLs gerados pelos analisadores
    validate-square.sh       # Script de validação
  square-validation/         # Arquivos oficiais de referência
```

## Decisão de organização dos módulos

Os analisadores foram mantidos em projetos Cargo separados porque esta unidade do curso está sendo desenvolvida de forma incremental: primeiro o analisador léxico, depois o analisador sintático e, futuramente, as próximas etapas do compilador.

A ideia para as próximas unidades é que as pastas que hoje funcionam como projetos separados passem a representar apenas módulos internos do projeto total. Com isso, o repositório poderá evoluir para ter um ponto de entrada global do compilador. Assim, no futuro, um `main` principal poderá coordenar todo o fluxo:

```text
.jack -> scanner -> parser -> compilation engine -> VM/code generator
```

Essa separação ajuda a testar e validar cada etapa isoladamente agora, sem impedir que `jack-lexical-analyzer`, `jack-syntactic-analyzer` e os próximos componentes sejam integrados depois como partes de um único compilador completo.

## Execução do módulo "Analisador Léxico"

### Pré-requisitos

- Rust devidamente instalado e configurado na máquina. (https://rust-lang.org/tools/install/)
- Git configurado corretamente na máquina.

### 0. Clonar o repositório Git

```bash
git clone https://github.com/lucasmcmatos/jackcompiler-rust.git
cd jackcompiler-rust
```

### 1. Acessar o diretório do módulo léxico

```bash
cd jack-lexical-analyzer
```

### 2. Construção do ambiente do projeto

```bash
cargo build
```

### 3. Compilar e executar o analisador léxico

Garanta que os arquivos `.jack` estejam na pasta global:

```text
square-tests/input
```

Depois execute:

```bash
cargo run
```

### 4. Verificar os resultados léxicos

Os arquivos gerados pelo analisador léxico ficam em:

```text
square-tests/output
```

Exemplos:

```text
MainT.xml
SquareT.xml
SquareGameT.xml
```

## Execução do módulo "Analisador Sintático"

### 1. Acessar o diretório do módulo sintático

A partir da raiz do repositório:

```bash
cd jack-syntactic-analyzer
```

### 2. Construção do ambiente do projeto

```bash
cargo build
```

### 3. Compilar e executar o analisador sintático

O analisador sintático consome os arquivos `*T.xml` gerados pelo analisador léxico em:

```text
square-tests/output
```

Para executar com o caminho padrão:

```bash
cargo run
```

Também é possível informar um arquivo ou diretório específico:

```bash
cargo run -- ../square-tests/output/MainT.xml
cargo run -- ../square-tests/output
```

### 4. Verificar os resultados sintáticos

Os arquivos gerados pelo analisador sintático também ficam em:

```text
square-tests/output
```

Exemplos:

```text
MainP.xml
SquareP.xml
SquareGameP.xml
```

> O sufixo `T` indica saída do analisador léxico.
> O sufixo `P` indica saída do analisador sintático.

## Validação

Os arquivos oficiais usados para validação estão em:

```text
square-validation
```

Para validar os resultados gerados:

```bash
bash square-tests/validate-square.sh
```

O script compara:

- `MainT.xml`, `SquareT.xml`, `SquareGameT.xml` contra os XMLs léxicos oficiais;
- `MainP.xml`, `SquareP.xml`, `SquareGameP.xml` contra os XMLs sintáticos oficiais (`Main.xml`, `Square.xml`, `SquareGame.xml`).

Status atual da validação:

```text
OK: Main tokens
OK: Main parser
OK: Square tokens
OK: Square parser
OK: SquareGame tokens
OK: SquareGame parser
Validacao Square concluida com sucesso.
```

## Relato da unidade

Durante esta unidade, o principal desafio foi organizar a evolução do compilador em etapas pequenas, mantendo o analisador léxico funcionando enquanto o analisador sintático era desenvolvido. No parser, a maior atenção foi seguir a gramática oficial do nand2tetris e gerar uma hierarquia XML equivalente aos arquivos oficiais, especialmente nas regras de `expression`, `term`, chamadas de subrotina e blocos de comandos.

Outro ponto importante foi manter os dois módulos separados, mas integrados por meio dos arquivos `*T.xml`, facilitando a validação e preparando o repositório para as próximas etapas do compilador.

## Comandos úteis para demonstração

Executar o analisador léxico:

```bash
cd jack-lexical-analyzer
cargo run
```

Executar o analisador sintático:

```bash
cd ../jack-syntactic-analyzer
cargo run
```

Validar os resultados:

```bash
cd ..
bash square-tests/validate-square.sh
```
