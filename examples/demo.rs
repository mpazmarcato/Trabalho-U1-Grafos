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
}
