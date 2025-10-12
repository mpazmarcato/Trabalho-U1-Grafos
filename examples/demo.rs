use graphs_algorithms::graphs::{AdjacencyList, AdjacencyMatrix, IncidenceMatrix};
use graphs_algorithms::utils::print_incidence_matrix;
use graphs_algorithms::{
    Graph, GraphIO, UndirectedGraph, UndirectedGraphIO, print_list, print_matrix,
};
use std::io;
use std::io::{Error, Write, stdin};

fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}


fn main() {
    loop {
        println!("========================================");
        println!("Bem-vindo/a à DEMO do trabalho de Grafos!");
        println!("Escolha a execução que deseja visualizar:");
        println!("1 - Grafos Não Orientados");
        println!("2 - Digrafos");
        println!("0 - Sair");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut opt = String::new();
        io::stdin().read_line(&mut opt).unwrap();
        let opt = opt.trim();

        match opt {
            "1" => demo_undirected(),
            "2" => demo_directed(),
            "0" => {
                println!("Encerrando a execução...");
                break;
            }
            _ => println!("Opção inválida. Tente novamente."),
        }
    }
}

fn demo_directed() {
    println!("SEÇÃO DE DIGRAFOS!");

    println!("1 - Representação do digrafo a partir da Matriz de Adjacência");
    println!("- Importando do arquivo DIGRAFO1.txt...");
    let res1: Result<AdjacencyMatrix, Error> =
        GraphIO::import_from_file("examples/data/DIGRAFO1.txt".to_string());
    let am1 = res1.unwrap();
    println!("- Temos a seguinte matriz: ");
    print_matrix(&am1);

    println!("2 - Representação do digrafo a partir da Matriz de Incidência");
    println!("- Importando do arquivo DIGRAFO1.txt...");
    let im1 = IncidenceMatrix::from_adjacency_matrix(&am1);
    println!("- Temos a seguinte matriz: ");
    print_incidence_matrix(&im1);

    println!("3 - Determinação do grafo subjacente");
    println!("- Criando lista de Adjacência a partir do DIGRAFO2.txt...");
    let res3: Result<AdjacencyList, Error> =
        GraphIO::import_from_file("examples/data/DIGRAFO2.txt".to_string());
    let l3 = res3.unwrap();
    println!("- Temos o seguinte digrafo: ");
    print_list(&l3);
    println!("- Seu grafo subjacente é, portanto: ");
    print_list(&l3.underlying_graph());

    println!("4 - Busca em Largura");
    println!("- Vamos aplicar a BFS na lista anterior a partir de dois vértices.");
    println!("- Vértice 0: resultado salvo em examples/dot/demo/digraph/bfs1.dot");
    let _ = l3.export_directed_bfs_to_dot(0, "examples/dot/demo/digraph/bfs1.dot".to_string());
    println!("- Vértice 9: resultado salvo em examples/dot/demo/digraph/bfs2.dot");
    let _ = l3.export_directed_bfs_to_dot(9, "examples/dot/demo/digraph/bfs2.dot".to_string());
    println!("- Para visualizar, compile os .dot com 'make png' no terminal!");

    println!(
        "5 - Busca em Profundidade com determinação de profundidade de entrada e saída e classificação de arestas"
    );
    println!(
        "- Vamos aplicar a DFS com classificação de arestas na lista anterior a partir de dois vértices."
    );
    println!("- Vértice 0: resultado salvo em examples/dot/demo/digraph/dfs1.dot");
    let _ = l3.export_directed_dfs_to_dot(0, "examples/dot/demo/digraph/dfs1.dot".to_string());
    println!("- Vértice 9: resultado salvo em examples/dot/demo/digraph/dfs2.dot");
    let _ = l3.export_directed_dfs_to_dot(9, "examples/dot/demo/digraph/dfs2.dot".to_string());
    println!("- Para visualizar, compile os .dot com 'make png' no terminal!");
}

fn demo_undirected() {
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
    println!("- Vértice 0: resultado salvo em examples/dot/demo/undirected_graph/bfs1.dot");
    let _ = l9
        .export_undirected_bfs_to_dot(0, "examples/dot/demo/undirected_graph/bfs1.dot".to_string());
    println!("- Vértice 9: resultado salvo em examples/dot/demo/undirected_graph/bfs2.dot");
    let _ = l9
        .export_undirected_bfs_to_dot(9, "examples/dot/demo/undirected_graph/bfs2.dot".to_string());
    println!("- Para visualizar, compile os .dot com 'make png' no terminal!");

    println!("14 - Busca em Profundidade com Arestas de Retorno");
    println!("- Vamos aplicar a DFS na lista anterior a partir de dois vértices.");
    println!("- Vértice 0: resultado salvo em examples/dot/demo/undirected_graph/dfs1.dot");
    let _ = l9
        .export_undirected_dfs_to_dot(0, "examples/dot/demo/undirected_graph/dfs1.dot".to_string());
    println!("- Vértice 9: resultado salvo em examples/dot/demo/undirected_graph/dfs2.dot");
    let _ = l9
        .export_undirected_dfs_to_dot(9, "examples/dot/demo/undirected_graph/dfs2.dot".to_string());
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
