Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

Este projeto implementa um sistema de busca otimizado para o catálogo de produtos da "MegaStore", uma gigante do varejo online. O sistema tem como objetivo permitir buscas rápidas e precisas por nome, marca e categoria dos produtos, garantindo uma experiência de compra eficiente e satisfatória para os clientes, além de escalabilidade.

Tecnologias e ferramentas utilizadas:

- Linguagem de programação Rust.
- Crate padrão do Rust (std::collections::HashMap), utilizada para estrutura de dados.
- Ferramentas de teste do Rust (cargo test).
- Visual Studio Code como ambiente de desenvolvimento.

Instruções para executar o sistema de busca:

- Certifique-se de que possui Rust instalado.
- Clone o repositório do projeto em seu ambiente de desenvolvimento:
  git clone
  cd megastore_search
- Execute o programa, utilizando dentro do terminal o comando "cargo run".

Instruções para executar os testes:

- Utilizar dentro do terminal o comando "cargo test".

O sistema permite buscas por diferentes critérios, exemplos de uso:

- Busca por nome: "sistema.buscar_por_nome("Smartphone");".
- Busca por marca: "sistema.buscar_por_marca("Nike");".
- Busca por categoria: "sistema.buscar_por_categoria("Eletrônicos");".
- Saída esperada:
  - Resultado da busca por nome: [Produto { id: 1, nome: "Smartphone", marca: "Samsung", categoria: "Eletrônicos" }, Produto { id: 2, nome: "Smartphone", marca: "Apple", categoria: "Eletrônicos" }]
  - Resultado da busca por marca: [Produto { id: 3, nome: "Camiseta", marca: "Nike", categoria: "Vestuário" }]
  - Resultado da busca por categoria: [Produto { id: 1, nome: "Smartphone", marca: "Samsung", categoria: "Eletrônicos" }, Produto { id: 2, nome: "Smartphone", marca: "Apple", categoria: "Eletrônicos" }]

O sistema é organizado em módulos. Sua arquitetura:

- produto: Define a estrutura do produto com campos como id, nome, marca e categoria.
- sistema_busca: Implementa a estrutura principal SistemaBusca, que utiliza tabelas hash para indexar produtos por nome, marca e categoria, permitindo buscas eficientes.
- main.rs: Exemplo de uso do sistema, adicionando produtos e realizando buscas.
- tests: Contém testes unitários e de integração para garantir a confiabilidade do sistema.

Algoritmos e estruturas de dados utilizados:

- O sistema utiliza tabelas hash (HashMap) para indexar e buscar produtos rapidamente. Cada produto é armazenado em múltiplos índices hash baseados em seus atributos (nome, marca, categoria), o que permite buscas em tempo próximo a O(1) para grandes volumes de dados. Essa abordagem garante que o sistema seja eficiente e escalável, mesmo com milhões de produtos.

Considerações sobre desempenho e escalabilidade:

- O sistema de busca desenvolvido foi projetado com foco em desempenho e eficiência, utilizando tabelas hash (HashMap) para indexar produtos por nome, marca e categoria. Essa estrutura oferece tempo médio de busca constante, o que garante respostas rápidas mesmo com grandes volumes de dados em memória.

- Pontos fortes:

  - Alta performance em memória, a busca por produtos é feita de forma rápida, sem necessidade de percorrer todos os itens.
  - Múltiplos índices, os produtos são indexados simultaneamente por diferentes atributos, o que permite buscas eficientes em diferentes contextos.
  - Baixa complexidade algorítmica, inserção e busca são operações rápidas, ideais para aplicações que exigem resposta imediata.
- Apesar das limitações, a solução proposta cumpre bem seu objetivo dentro do escopo acadêmico, demonstrando como estruturas de dados eficientes podem ser aplicadas para otimizar sistemas de busca em catálogos extensos. Com ajustes futuros, ela pode ser estendida para aplicações maiores e mais complexas.

Licença:

- Este projeto está licenciado sob a licença MIT:

Copyright 2025 Lais

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
