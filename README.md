# Trabalho Unidade 1 Grafos

## Estrutura do repositório em Rust 🦀

```bash
Unidade1-Grafos/
├── README.md
├── Cargo.lock
├── Cargo.toml
├── benches # Testes de benchmark
│   └── dfs_bench.rs
│   ...
├── examples # Programas exemplo para testar implementações
│   └── test.rs
├── latex # Código fonte do documento latex
│   ├── chapters
│   ├── CS_report.sty   # Definições e import de biblioteca
│   ├── figures         # Imagens usadas no documento
│   ├── main.tex        # Entry point do código fonte
│   ├── Makefile        # Para compilar o documento
│   └── references.bib  # Referências usadas no texto
└── src
    ├── adjacency_list.rs   # Implementação de um grafo como lista de adjacência
    ├── adjacency_matrix.rs # Implementação de um grafo como matriz de adjacência
    ├── incidence_matrix.rs # Implementação de um grafo como matriz de incidência
    ├── graph.rs            # Trait (Interface) de um grafo
    └── lib.rs              # Re-exportação dos items da crate
    ...
```

## Desenvolvimento

### Pré-requisitos

- [Cargo 1.90.0 (com rustc 1.90.0 stable)](https://rust-lang.org/learn/get-started/)
- [Texlive (full)](https://tug.org/texlive/) e Texlive-lang-portuguese. (pode ser encontrado nos gerenciadores de pacote)
- [Docker](https://www.docker.com/) (Para compilar o latex se não quiser instalar diretamente o texlive)

### Compilação e testes

#### Rust

```bash
# Compila o projeto
cargo b

# Executa binários na pasta examples/
cargo r --example test

# Executa testes unitários
cargo t

# Executa benchmarks
cargo bench

# Verifica o código usando o clippy
cargo clippy

# Formata o código
cargo fmt

# Compila documentação
cargo doc
```

#### $\LaTeX$

Na pasta `latex/`:

```bash
# Exibe receitas disponíveis
make help

# Compila pdf no diretório output/
make

# Limpa arquivos auxiliares
make clean

# Limpa todos os arquivos (incluindo pdf)
make distclean

# Limpa e compila novamente
make rebuild
```

#### $\LaTeX$ com Docker

```bash
# Cria a imagem docker
docker build -t latex-compiler latex/

# Compila a imagem e executa o container criando o pdf.
# --rm automaticamente deleta o container e o volume
docker run --rm latex-compiler > main.pdf
```

Existe uma imagem compilada em `vleema/latex-compiler` (não garanto que esteja atualizada). Podes substituir `docker build...` por

```bash
docker pull vleema/latex-compiler:latest
```

## TODOs

> [!NOTE]
>
> - Essa seção reflete parte das issues criadas no repositório, servindo como guideline.
> - As que não pedem a implementação específica em uma ou mais estruturas podem ser interpretadas como "a implementação em pelo menos uma estrutura". Por exemplo, na tarefa "(5) Função que calcula o grau de cada vértice" ficaria implícito que basta implementar uma função do trait `Graph` (nesse caso `neighbors`) para apenas uma das estruturas `AdjacencyList`, `AdjacencyMatrix` e `IncidenceMatrix`.
> - O checkbox aqui é opcional, acho que o importante é a criação e o fechamento das issues que são essenciais para o trabalho.

A. Para GRAFOS (as opcionais possuem a sigla OPC ao final da função)

- [x] (1) Criação do Grafo a partir da Lista de Adjacências.
- [x] (2) Criação do Grafo a partir da Matriz de Adjacências.
- [x] (3) Criação do Grafo a partir da Matriz de Incidência.
- [x] (4) Conversão de matriz de adjacência para lista de Adjacências e vice-versa.
- [ ] (5) Função que calcula o grau de cada vértice.
- [x] (6) Função que determina se dois vértices são adjacentes.
- [ ] (7) Função que determina o número total de vértices.
- [ ] (8) Função que determina o número total de arestas.
- [ ] (9) Inclusão de um novo vértice usando Lista de Adjacências e Matriz de Adjacências.
- [ ] (10) Exclusão de um vértice existente usando Lista de Adjacências e Matriz de Adjacências.
- [x] (11) Função que determina se um grafo é conexo ou não.
- [ ] (12) Determinar se um grafo é bipartido (OPC = 1,0 ponto).
- [ ] (13) Busca em Largura, a partir de um vértice específico.
- [x] (14) Busca em Profundidade, com determinação de arestas de retorno, a partir de um vértice em específico.
- [ ] (15) Determinação de articulações e blocos (biconectividade), utilizando obrigatoriamente a função lowpt.

B. Para DIGRAFOS (as opcionais possuem a sigla OPC ao final da função)

- [x] (16) Representação do Digrafo a partir da Matriz de Adjacências.
- [ ] (17) Representação do Digrafo a partir da Matriz de Incidência.
- [ ] (18) Determinação do Grafo subjacente (OPC= 0,5 ponto)
- [ ] (19) Busca em largura. (equivalente ao 13)
- [x] (20) Busca em profundidade, com determinação de profundidade de entrada e de saída de cada vértice, e arestas de árvore, retorno, avanço e cruzamento.

D. Relatório

- [ ] (30) Introdução: Sumário, objetivo do trabalho.
- [ ] (31) Definições: Definição de grafos, vértices, arestas, grau, grafos direcionados, etc.
- [ ] (32) Representações de Grafos: Explicação teórica, vantagens e desvantagens.
- [ ] (33) Algoritmos: Descrição dos algoritmos implementados.
- [ ] (34) Implementação: Linguagem de programação, estrutura do projeto, detalhes da implementação.
- [ ] (35) Testes: Testes elaborados, hardware e resultados dos testes.
- [ ] (36) Referências: Listar referências usadas no bibtex.

### Extra

C. Implementação de testes de benchmark para comparar performance das implementações dos grafos.

- [ ] (21) Comparação de performance de `has_edge()` entre as três implementações em grafos de diferentes densidades (esparso, médio, denso).
- [ ] (22) Comparação de performance de `neighbors()` entre as três implementações para nós com diferentes graus de conectividade.
- [ ] (23) Comparação de performance de `add_node()` e `remove_node()` entre as implementações com grafos de diferentes tamanhos.
- [ ] (24) Comparação de performance de `add_edge()` e `remove_edge()` entre as implementações.
- [ ] (25) Comparação da performance do BFS entre as implementações em grafos esparsos vs densos.
- [ ] (26) Comparação da performance do DFS entre as implementações em grafos esparsos vs densos.
- [ ] (27) Comparação da performance da detecção de biconectividade entre as implementações.
- [ ] (28) Comparação da performance da detecção de grafo bipartido entre as implementações.
- [ ] (29) Comparação da performance da verificação de conectividade entre as implementações.
