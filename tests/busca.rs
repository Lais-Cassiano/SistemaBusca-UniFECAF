use megastore_search::produto::Produto;
use megastore_search::sistema_busca::SistemaBusca;

#[test]
fn teste_busca_por_nome() {
    let mut sistema = SistemaBusca::new();

    sistema.adicionar_produto(Produto {
        id: 1,
        nome: "Notebook".to_string(),
        marca: "Dell".to_string(),
        categoria: "Eletrônicos".to_string(),
    });

    let resultados = sistema.buscar_por_nome("Notebook");
    assert_eq!(resultados.len(), 1);
    assert_eq!(resultados[0].nome, "Notebook");
}

#[test]
fn teste_busca_por_categoria_sem_resultado() {
    let sistema = SistemaBusca::new();
    let resultados = sistema.buscar_por_categoria("Móveis");
    assert!(resultados.is_empty());
}
