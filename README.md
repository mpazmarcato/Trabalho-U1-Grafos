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
├── crates # Crates auxiliares
│   └── cpp_api 
├── examples # Programas de exemplo para testar implementações
│   └── simple_tests.rs
│   ...
│   └── data/ # Arquivos .txt com grafos
│   └── dot/ # Diagramas de grafos em .dot
│   └── output/ # Imagens de grafos geradas através dos .dot
├── latex # Código fonte do documento latex
│   ├── chapters/
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
    ├── graph.rs            # Trait (Interface) de operações de leitura/escrita de grafos em arquivos
    ├── utils.rs            # Funções de utilidade usadas nos examples
    └── lib.rs              # Re-exportação dos items da crate
    ...
```

## Desenvolvimento

### Pré-requisitos

- [Cargo 1.90.0 (com rustc 1.90.0 stable)](https://rust-lang.org/learn/get-started/)
- [Texlive (full)](https://tug.org/texlive/) e Texlive-lang-portuguese: pode ser encontrado nos gerenciadores de pacote comuns.
- [Docker](https://www.docker.com/): Alternativa para compilar o $\LaTeX$, caso não queira instalar o `texlive`
- [Graphviz](https://www.graphviz.org/download/): Para converter os arquivos `.dot` em imagens `.png`
- TODO

### Compilação e testes

#### Rust

```bash
# Compila o projeto
cargo b

# Executa binários na pasta examples/
cargo r --example [example]

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

Existe uma imagem compilada em `vleema/latex-compiler` (não garantimos que esteja atualizada). Podes substituir `docker build...` por

```bash
docker pull vleema/latex-compiler:latest
```

## Exemplos

  No diretório `examples/` estão presentes diversos scripts que demonstram os usos da biblioteca principal. Estes são:
- `adjacency_list_operations.rs`: cria grafos em uma Lista de Adjacência e adiciona/exclui vértices e arestas;
- `adjacency_matrix_operations.rs`: cria grafos em uma Matriz de Adjacência e adiciona/exclui vértices e arestas;
- `classify_edges_graph.rs`: executa a DFS em um digrafo, salvando a árvore resultante no diretório `dot/classify_edges`
- `classify_edges_undirected_graph.rs`: executa a DFS em um grafo não orientado, salvando a árvore resultante no diretório `dot/classify_edges`
- `graph_bfs.rs`: executa a BFS em grafos orientados e não orientados, salvando a árvore resultante no diretório `dot/bfs`;
- `graph_creation.rs`: importa alguns arquivos `.txt` e cria ou não o grafo correspondente;
- `graph_exports.rs`: cria grafos, executa operações diversas e salva os resultados em `dot/graph_exports_example`;
- `node_checks.rs`: cria grafos e calcula a ordem, tamanho e grau de cada vértice;
- `underlying_graphs.rs`: cria digrafos e encontra seus grafos subjacentes;
- `simple_tests.rs`: outras operações simples com grafos.

Para executá-los, veja a seção anterior.

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
- [x] (5) Função que calcula o grau de cada vértice.
- [x] (6) Função que determina se dois vértices são adjacentes.
- [x] (7) Função que determina o número total de vértices.
- [x] (8) Função que determina o número total de arestas.
- [x] (9) Inclusão de um novo vértice usando Lista de Adjacências e Matriz de Adjacências.
- [x] (10) Exclusão de um vértice existente usando Lista de Adjacências e Matriz de Adjacências.
- [x] (11) Função que determina se um grafo é conexo ou não.
- [ ] (12) Determinar se um grafo é bipartido (OPC = 1,0 ponto).
- [x] (13) Busca em Largura, a partir de um vértice específico.
- [x] (14) Busca em Profundidade, com determinação de arestas de retorno, a partir de um vértice em específico.
- [x] (15) Determinação de articulações e blocos (biconectividade), utilizando obrigatoriamente a função lowpt.

B. Para DIGRAFOS (as opcionais possuem a sigla OPC ao final da função)

- [x] (16) Representação do Digrafo a partir da Matriz de Adjacências.
- [x] (17) Representação do Digrafo a partir da Matriz de Incidência.
- [x] (18) Determinação do Grafo subjacente (OPC= 0,5 ponto)
- [x] (19) Busca em largura. (equivalente ao 13)
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

C. Implementação de testes de benchmark para comparar performance da implementação em Rust com versão clássica de C++.

- [ ] (21) Comparação de performance de `neighbors()` para lista de adjacência e matriz de adjacência.
- [ ] (22) Micro-benchmark da busca em profundidade (DFS).
- [ ] (23) Macro-benchmark da busca em profundidade (DFS).
- [ ] (24) Micro-benchmark da busca em largura (BFS).
- [ ] (25) Macro-benchmark da busca em largura (BFS).
- [ ] (26) Micro-benchmark da identificação de componentes biconexos.
- [ ] (27) Macro-benchmark da identificação de componentes biconexos.
- [ ] (28) Macro-benchmark da determinação de grafo bipartido.
- [ ] (29) Macro-benchmark da classificação de arestas.
