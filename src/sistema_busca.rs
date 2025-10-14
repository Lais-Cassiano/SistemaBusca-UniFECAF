use std::collections::HashMap;
use crate::produto::Produto;

pub struct SistemaBusca {
    indice_nome: HashMap<String, Vec<Produto>>,
    indice_marca: HashMap<String, Vec<Produto>>,
    indice_categoria: HashMap<String, Vec<Produto>>,
}

impl SistemaBusca {
    pub fn new() -> Self {
        SistemaBusca {
            indice_nome: HashMap::new(),
            indice_marca: HashMap::new(),
            indice_categoria: HashMap::new(),
        }
    }

    pub fn adicionar_produto(&mut self, produto: Produto) {
        self.indice_nome
            .entry(produto.nome.to_lowercase())
            .or_default()
            .push(produto.clone());

        self.indice_marca
            .entry(produto.marca.to_lowercase())
            .or_default()
            .push(produto.clone());

        self.indice_categoria
            .entry(produto.categoria.to_lowercase())
            .or_default()
            .push(produto);
    }

    pub fn buscar_por_nome(&self, nome: &str) -> Vec<&Produto> {
        self.indice_nome
            .get(&nome.to_lowercase())
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }

    pub fn buscar_por_marca(&self, marca: &str) -> Vec<&Produto> {
        self.indice_marca
            .get(&marca.to_lowercase())
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }

    pub fn buscar_por_categoria(&self, categoria: &str) -> Vec<&Produto> {
        self.indice_categoria
            .get(&categoria.to_lowercase())
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }
}
