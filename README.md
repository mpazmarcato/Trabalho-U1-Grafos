# Trabalho Unidade 1 Grafos

Vídeo que explica o funcionamento do projeto: <https://drive.google.com/file/d/1dWUMXOR_W2P-UAwmElS3u4a4tlCy-9oE/view?usp=sharing>

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
│   └── cpp_api # Crate que implementa grafos em C++
│   └── graph-io # Crate que implementa operações de leitura/escrita de grafos em arquivos
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
    └── lib.rs              # Re-exportação dos items da crate
    ...
```

## Desenvolvimento

### Pré-requisitos

- [Cargo 1.90.0 (com rustc 1.90.0 stable)](https://rust-lang.org/learn/get-started/)
- [Texlive (full)](https://tug.org/texlive/) e Texlive-lang-portuguese: pode ser encontrado nos gerenciadores de pacote comuns.
- [Docker](https://www.docker.com/): Alternativa para compilar o $\LaTeX$, caso não queira instalar o `texlive`
- [Graphviz](https://www.graphviz.org/download/): Para converter os arquivos `.dot` em imagens `.png`
- [Valgrind](https://valgrind.org/): Para prover informações para o Gungraun.
- [Gungraun-runner](https://gungraun.github.io/gungraun/latest/html/installation/gungraun.html): Para executar os testes com o Gungraun.
- [Gnuplot](http://gnuplot.info/) (opcional): Para a geração de gráficos do Criterion-rs.
- [Clang](https://clang.llvm.org/): Compilador C++ para conseguir compilar API em C++.

### Compilação e testes

#### : Compilador C++ para conseguir compilar API de C++Rust

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

- `demo.rs`: arquivo com as principais operações descritas na proposta de implementação do trabalho. **Comece por aqui**.
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
