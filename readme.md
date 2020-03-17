# UpChallenge API
API Rest escrita em Rust

# Rodando localmente
## Dependências do Rust 
  (Aqui estou considerando que o ambiente tenha o `rustup` (Rust 1.42.0)  e MySQL instalados):
  
  - Instalação do Diesel:
    `cargo install diesel_cli --no-default-features --features mysql`
    (Obs.: O Diesel usa algumas bibliotecas compartilhadas em C. 
    A instalação no sistema operacional dessas bibliotecas pode ser necessária.)
  
  - Na raiz do projeto, copie o arquivo `.env.example` para `.env` 
    e adicione suas credenciais de banco de dados, host e porta para a API.
    `cp .env.example .env`
    
    Exemplo:
    
    ```(env)
    DATABASE_URL=mysql://username:passwd@localhost:3306/database_name
    HOST=127.0.0.1
    PORT=8081
    ```
    
   - Configurada a conexão ao banco de dados, é hora de executar as migrations. 
     Execute o comando abaixo na raiz do projeto
     `diesel setup` 
   
   - O banco já terá alguns usuários e tweets criados. 
   
   Usuários
   ```
   |id |      name       |        email       |
   |---|:---------------:|-------------------:|
   | 1 | Freddie Mercury | freddie@queen.com  |
   | 2 |   Elton John    |  elton@rocket.com  |
   | 3 | David Bowie     | bowie@startman.com |
   ```
  - Após isso, é só rodar o build e executar!
  `cargo run`
  
## API

**Endpoints:**  
* **Users**
    * [GET api/v1/users/:user/tweets](#) - Lista os tweets de um usuário
    * [POST api/v1/users/:user/tweets](#) - Cria um novo tweet para o usuário
    * [GET api/v1/users/:user/tweets/:tweet](#) - Exibe um tweet
    
**Listar tweets**
----
  _Lista os tweets de um usuário_

* **URL**  
  _/api/v1/users/:user/tweets_
  
* **Method:**  
  `GET`  

*  **URL Params**  
   **Obrigatório:**  
   `user=[integer]` - ID do usuário  
   
* **Success Response:**
  
  * **Code:** 200 <br/>
    **Content:** 
    ```(json)
        [
          {
            "id": 1,
            "user_id": 1,
            "body": "Ay-ooooooh"
          },
          {
            "id": 2,
            "user_id": 1,
            "body": "Anyway the wind blows"
          },
          {
            "id": 3,
            "user_id": 1,
            "body": "We are the champions, my friends!"
          }
        ]
    ```
 
* **Error Response:**

  * **Code:** 500 INTERNAL SERVER ERROR <br />
    **Content:**
    ```(json)
    {
      "message": "Error message"
    }
    ```

**Criar tweet**
----
  _Cria um novo tweet para o usuário_

* **URL**  
  _/api/v1/users/:user/tweets_
  
* **Method:**  
  `POST`  

*  **URL Params**  
   **Obrigatório:**  
   `user=[integer]` - ID do usuário  

*  **Headers**
   **Obrigatório:**  
   `Content-Type` - Deve ser `application/json`  

*  **Body Params**

   **Required:**  
   `body=[string]` - Conteúdo do Tweet.  

   **Example:**
    ```(json)
        {
        	"body": "I lost my keys again"
        }
    ```
 
* **Success Response:**
  
  * **Code:** 201 CREATED <br/>
    **Content:** 
    ```(json)
      {
        "id": 3,
        "user_id": 1,
        "body": "I lost my keys again"
      }    
    ```
 
* **Error Response:**

  * **Code:** 400 BAD REQUEST <br />
    **Content:**
    ```(json)
    ```

  * **Code:** 500 INTERNAL SERVER ERROR <br />
    **Content:**
    ```(json)
    {
      "message": "Error message"
    }
    ```

**Exibir um tweet**
----
  _Exibe um tweet_

* **URL**  
  _/api/v1/users/:user/tweets/:tweet_
  
* **Method:**  
  `GET`  

*  **URL Params**  
   **Obrigatório:**  
   `user=[integer]` - ID do usuário  
   `tweet=[integer]` - ID do tweet  
   
* **Success Response:**
  
  * **Code:** 200 <br/>
    **Content:** 
    ```(json)
        {
          "id": 3,
          "user_id": 1,
          "body": "We are the champions, my friends!"
        }
    ```
 
* **Error Response:**

  * **Code:** 404 <br/>
    **Content:** 
    ```(json)
        {
          "message": "Tweet not found"
        }
    ```

  * **Code:** 500 INTERNAL SERVER ERROR <br />
    **Content:**
    ```(json)
    {
      "message": "Error message"
    }
    ```