use megastore_search::produto::Produto;
use megastore_search::sistema_busca::SistemaBusca;

fn main() {
    let mut sistema = SistemaBusca::new();

    sistema.adicionar_produto(Produto {
        id: 1,
        nome: "Smartphone".to_string(),
        marca: "Samsung".to_string(),
        categoria: "Eletrônicos".to_string(),
    });

    sistema.adicionar_produto(Produto {
        id: 2,
        nome: "Smartphone".to_string(),
        marca: "Apple".to_string(),
        categoria: "Eletrônicos".to_string(),
    });

    sistema.adicionar_produto(Produto {
        id: 3,
        nome: "Camiseta".to_string(),
        marca: "Nike".to_string(),
        categoria: "Vestuário".to_string(),
    });

    let resultados = sistema.buscar_por_nome("Smartphone");
    println!("Resultado da busca por nome: {:?}", resultados);

    let resultados = sistema.buscar_por_marca("Nike");
    println!("Resultado da busca por marca: {:?}", resultados);

    let resultados = sistema.buscar_por_categoria("Eletrônicos");
    println!("Resultado da busca por categoria: {:?}", resultados);
}
