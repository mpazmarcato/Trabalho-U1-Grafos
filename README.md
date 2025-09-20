# Trabalho Unidade 1 Grafos

TODO: Adicionar comando para compilar documento em $\LaTeX$.

# Lista de atividades dos colaboradores

## 1. Estruturas de Dados Básicas

- [ ] 1. Crie um programa para criação e manipulação de um grafo simples, incluindo as funções de inserir vértice, excluir vértice e imprimir grafo. Execute a instância do primeiro exemplo desta aula.
- [x] 2. Dada uma representação de um grafo não direcionado por matriz de adjacência, faça um algoritmo que represente o grafo em lista de adjacência.
- [x] 3. Dada uma representação de um grafo direcionado por matriz de adjacência, faça um algoritmo que represente o grafo em matriz de incidência.
- [ ] 4. Dada uma representação de um grafo direcionado por matriz de adjacência, faça um algoritmo que represente o grafo em estrela direta.

Analise a complexidade dos algoritmos das questões 2 a 4.

## 2. Busca em Profundidade

- [x] 1. Implemente o algoritmo para a busca em profundidade com matriz de adjacência e sem recorrência;
- [ ] 2. Implemente o algoritmo para a busca em profundidade com lista de adjacência e com recorrência;
- [ ] 3. Implemente o algoritmo para a busca em profundidade com lista de adjacência e com recorrência, salvando o predecessor;

Analise a complexidade dos algoritmos das questões 1 e 2.

## 3. Busca em Largura

- [ ] 1. Implemente o algoritmo para a busca em largura com matriz de adjacência e sem recorrência;
- [ ] 2. Implemente o algoritmo para a busca em largura com arestas de retorno e classificando as arestas como pai, tio, irmão ou primo;
- [ ] 3. Implemente o algoritmo para a busca em largura com lista de adjacência e com recorrência, salvando o predecessor;

## 4. Biconectividade

- [ ] 1. Implemente a Aplicação do algoritmo de percurso em profundidade para a determinação de blocos, usando a função Lowpt(w) e a árvore de profundidade. Seu algoritmo deve imprimir os vértices pertencentes a cada bloco.
- [ ] 2. Implementar uma função que determine se determinada aresta do grafo é uma ponte.

# Estrutura do repositório em Rust 🦀

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
