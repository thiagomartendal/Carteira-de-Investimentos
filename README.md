# Carteira de Investimentos

Repositório voltado ao projeto final do Bootcamp Santander em **Rust**, que se trata de um simulador de carteira de investimentos em criptoativos. Neste projeto, é possível cadastrar e comprar ativos. Um ativo é cadastrado por um usuário administrador, e só pode ser comprado por um usuário não-administrador.

O projeto utiliza o framework **Axum** para servir a API, junto ao **sqlx** para manipulação de banco de dados PostgreSQL. As demais dependências estão disponíveis no arquivo *Cargo.toml*, mas destaca-se o uso de **tokens JWT** para autenticação de sessão e **PrivateCookieJar** para armazenar cookies criptografados. Para os testes, a biblioteca **insta** é usada para avaliar as saídas das funções com os snapshots gerados.

A funcionalidade básica do projeto não foi modificada, tendo-se dado preferência para melhorar a estrutura arquitetural do projeto, optando-se pela **arquitetura baseada em recursos** (alternativa ao MVC).

## Executando o Projeto

Para executar o projeto, primeiro deve-se criar o banco de dados **wallet** no postgresql, e este deve ser informado em uma string **DATABASE_URL** em um arquivo *.env* para a conexão. Então, acessar o diretório **carteira_investimentos** via terminal.

  + Um modelo para a string de conexão no arquivo *.env*:

      **DATABASE_URL=postgres://usuario:senha@host:porta/wallet**

Após acessar o diretório no terminal, deve-se criar as tabelas necessárias com sqlx através de: **sqlx migrate run**

Caso seja de interesse, os teste unitários podem ser executados com: **cargo test**
  + Caso seja necessário aceitar uma nova atribuição de snapshots, execute: **cargo insta review**

Para executar o servidor da aplicação: **cargo run**

Abaixo seguem algumas capturas de tela que mostram um pouco da aplicação funcional.

## Login

Página de acesso para a carteira.

![Login](telas/login.png)

## Nova Conta

Página de cadastro de conta. Se o tipo de usuário é marcado como comprador, este pode apenas comprar ativos disponíveis. Se marcado como administrador, este pode apenas cadastrar ativos.

![NovaConta](telas/nova_conta.png)

## Painel de Controle do Usuário 1

Página de um usuário administrador. O botão **cadastrar ativo** exibe o formulário de cadastro, enquanto o botão **editar** abre o formulário de edição. 

![Usuario1](telas/usuario1.png)

## Carteira do Usuário 2

Página de um usuário comprador, mostrando ativos disponíveis para compra cadastrados no sistema, junto aos ativos que jpa foram adquiridos pelo usuário.

![Usuario2](telas/usuario2.png)

## Carteira do Usuário 3

Página de outro usuário comprador.

![Usuario3](telas/usuario3.png)
