use graphs_algorithms::graphs::{AdjacencyList, AdjacencyMatrix, IncidenceMatrix};
use graphs_algorithms::utils::print_incidence_matrix;
use graphs_algorithms::{Graph, UndirectedGraph, UndirectedGraphIO, print_list, print_matrix};
use std::io::Error;

fn main() {
    println!("Bem vindo/a à demo! Aqui, vamos passar por cada questão do trabalho de grafos.");
    println!("SEÇÃO DE GRAFOS NÃO ORIENTADOS!");

    println!("1 - Criação do grafo a partir da Lista de Adjacências");
    println!("Importando do arquivo GRAFO_2.txt...");

    let res1: Result<AdjacencyList, Error> =
        UndirectedGraphIO::import_undirected_from_file("examples/data/GRAFO_2.txt".to_owned());
    print_list(&res1.unwrap());

    println!("2 - Criação do grafo a partir da Matriz de Adjacências");
    println!("Importando do arquivo GRAFO_2.txt...");

    let res2: Result<AdjacencyMatrix, Error> =
        UndirectedGraphIO::import_undirected_from_file("examples/data/GRAFO_2.txt".to_owned());
    let mut m2 = res2.unwrap();
    print_matrix(&m2);

    println!("3 - Criação do grafo a partir da Matriz de Incidência");
    println!("Importando do arquivo GRAFO_2.txt...");

    let im = IncidenceMatrix::from_adjacency_matrix(&m2);
    print_incidence_matrix(&im);

    println!("4 - Conversão entre Matriz de Adjacência e Lista de Adjacência");
    let res4: Result<AdjacencyMatrix, Error> =
        UndirectedGraphIO::import_undirected_from_file("examples/data/GRAFO_2.txt".to_owned());
    println!("4.1 - Matriz -> Lista");
    let mut m4 = res4.unwrap();
    println!("- Temos a seguinte matriz:");
    print_matrix(&m4);

    println!("- Convertendo para lista..");
    let l4 = AdjacencyList::from_adjacency_matrix(&m4);
    println!("- Temos a seguinte lista:");
    print_list(&l4);

    println!("4.2 - Lista -> Matriz");
    println!("- Convertendo a lista anterior para matriz...");
    m4 = AdjacencyMatrix::from_adjacency_list(&l4);
    println!("- Temos a seguinte matriz:");
    print_matrix(&m4);

    println!("5 - Função que calcula o grau de cada vértice");
    println!("- Para a matriz da questão 04, temos os seguintes graus:");
    for i in 0..m4.order() {
        println!("Vértice {} tem grau {}.", i, m4.undirected_node_degree(i));
    }

    println!("6 - Função que determina a adjacência entre dois vértices");
    println!(
        "- Para a matriz da questão 04, verifiquemos se há arestas entre os seguintes vértices:"
    );
    println!("Aresta (0, 1): {} ", m4.has_edge(0, 1));
    println!("Aresta (5, 0): {} ", m4.has_edge(5, 0));
    println!("Aresta (1, 3): {} ", m4.has_edge(1, 3));

    println!("7 - Função que determina o número total de vértices");
    println!(
        "- Para a matriz da questão 04, temos que a ordem do grafo é {}",
        m4.order()
    );

    println!("8 - Função que determina o número total de arestas");
    println!(
        "- Para a matriz da questão 04, temos que o tamanho do grafo é {}",
        m4.undirected_size()
    );

    println!("9 - Inclusão de um novo vértice");
    println!("9.1 - Lista de Adjacência");
    let mut l9 = l4.clone();
    println!("- Temos a seguinte lista: ");
    print_list(&l9);
    println!("- Adicionemos dois novos vértices inicialmente sem arestas: ");
    l9.add_node(11);
    l9.add_node(12);
    println!("- Agora, a lista é: ");
    print_list(&l9);

    println!("9.2 - Matriz de Adjacência");
    let mut m9 = m4.clone();
    println!("Temos a seguinte matriz: ");
    print_matrix(&m9);
    println!("- Adicionemos dois novos vértices inicialmente sem arestas.");
    m9.add_node(11);
    m9.add_node(12);
    println!("- Agora, a matriz é: ");
    print_matrix(&m9);

    println!("10 - Exclusão de um vértice");
    println!("10.1 - Lista de Adjacência");
    println!("- Considerando a lista passada, vamos eliminar os nós 5 e 11. ");
    l9.remove_node(5);
    l9.remove_node(11);
    println!("- Agora, a lista com os nós atualizados é: ");
    print_list(&l9);

    println!("10.2 - Matriz de Adjacência");
    println!("- Considerando a matriz passada, vamos eliminar os nós 5 e 11. ");
    m9.remove_node(5);
    m9.remove_node(11);
    println!("- Agora, a matriz com os nós atualizados é: ");
    print_matrix(&m9);

    println!("11 - Função que determina se um grafo é conexo");
    println!("- Considerando a lista do tópico 10, vamos investigar. ");
    println!("- O grafo é conexo: {}", l9.connected());
    println!("- Adicionando arestas para os nós 5, 9 e 10.");
    l9.add_undirected_edge(1, 5);
    l9.add_undirected_edge(4, 5);
    l9.add_undirected_edge(5, 10);
    l9.add_undirected_edge(9, 0);
    println!("- O grafo é conexo: {}", l9.connected());

    println!("12 - Determinar se um grafo é bipartido");
    println!("- Considerando a seguinte lista: ");
    print_list(&l9);
    println!("- O grafo é bipartido: {}", l9.bipartite());

    println!("13 - Busca em Largura");
    println!("- Vamos aplicar a BFS na lista anterior a partir de dois vértices.");
    println!("- Vértice 0: resultado salvo em examples/dot/demo/bfs1.dot");
    let _ = l9.export_undirected_bfs_to_dot(0, "examples/dot/demo/bfs1.dot".to_string());
    println!("- Vértice 9: resultado salvo em examples/dot/demo/bfs2.dot");
    let _ = l9.export_undirected_bfs_to_dot(9, "examples/dot/demo/bfs2.dot".to_string());
    println!("- Para visualizar, compile os .dot com 'make png' no terminal!");

    println!("14 - Busca em Profundidade com Arestas de Retorno");
    println!("- Vamos aplicar a DFS na lista anterior a partir de dois vértices.");
    println!("- Vértice 0: resultado salvo em examples/dot/demo/dfs1.dot");
    let _ = l9.export_undirected_dfs_to_dot(0, "examples/dot/demo/dfs1.dot".to_string());
    println!("- Vértice 9: resultado salvo em examples/dot/demo/dfs2.dot");
    let _ = l9.export_undirected_dfs_to_dot(9, "examples/dot/demo/dfs2.dot".to_string());
    println!("- Para visualizar, compile os .dot com 'make png' no terminal!");

    println!("15 - Determinação de articulações e blocos (biconectividade)");
    println!(
        "- Vamos aplicar o algoritmo de biconectividade na lista anterior a partir do vértice 0."
    );
    let mut iter = l9.biconnected_components(0);
    let mut components = Vec::new();
    while let Some(component) = iter.next() {
        components.push(component);
    }
    println!("Componentes biconectados (arestas):");
    for (i, comp) in components.iter().enumerate() {
        println!("  Componente {}:", i + 1);
        for (u, v) in comp {
            println!("    ({}, {})", u, v);
        }
    }

    println!("FIM DA SEÇÃO DE GRAFOS NÃO ORIENTADOS");
}
