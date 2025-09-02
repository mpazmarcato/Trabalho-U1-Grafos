# Trabalho Unidade 1 Grafos 

# Lista de atividades dos colaboradores

## 1. Estruturas de Dados Básicas

- [ ] 1. Crie um programa para criação e manipulação de um grafo simples, incluindo as funções de inserir vértice, excluir vértice e imprimir grafo. Execute a instância do primeiro exemplo desta aula.
- [ ] 2. Dada uma representação de um grafo não direcionado por matriz de adjacência, faça um algoritmo que represente o grafo em lista de adjacência.
- [ ] 3. Dada uma representação de um grafo direcionado por matriz de adjacência, faça um algoritmo que represente o grafo em matriz de incidência.
- [ ] 4. Dada uma representação de um grafo direcionado por matriz de adjacência, faça um algoritmo que represente o grafo em estrela direta.

Analise a complexidade dos algoritmos das questões 2 a 4.

## 2. Busca em Profundidade
- [ ] 1. Implemente o algoritmo para a busca em profundidade com matriz de adjacência e sem recorrência;
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

# Estrutura do repositório em Rust

Unidade1-Grafos/
│
├── README.md
├── .gitignore
└── src/
    ├── main.rs
    └── graphs /
        ├── mod.rs
        ├── adjacency_list.rs
        ├── ...
        └── adjacency_matrix.rs
