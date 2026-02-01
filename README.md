# Requisição de Funcionalidades – Sistema de API Simples

## 1. Visão Geral

O cliente solicita o desenvolvimento de uma **API simples**, com foco em aprendizado e validação técnica, que permita o gerenciamento básico de entidades através de requisições HTTP.

O sistema não possui requisitos de interface gráfica (frontend). Todo o acesso será realizado via API REST, utilizando o formato JSON para entrada e saída de dados.

---

## 2. Objetivo do Sistema

O objetivo do sistema é:

* Permitir operações básicas de **criação, leitura, atualização e exclusão (CRUD)**
* Servir como base para estudos de arquitetura, organização de código e persistência de dados
* Ser simples, claro e bem estruturado

---

## 3. Escopo Inicial

O sistema deverá gerenciar **Usuários**.

Cada usuário representa um registro independente no sistema.

### Entidade: Usuário

Campos obrigatórios:

* `id` (inteiro, gerado automaticamente)
* `name` (texto)
* `email` (texto, único)
* `created_at` (data/hora de criação)

---

## 4. Funcionalidades Requeridas

### 4.1 Criar Usuário

O sistema deve permitir a criação de um novo usuário.

**Requisitos:**

* O email não pode ser duplicado
* Todos os campos obrigatórios devem ser validados

**Entrada:**

* Nome
* Email

**Saída esperada:**

* Usuário criado com `id` e `created_at`

---

### 4.2 Listar Usuários

O sistema deve permitir a listagem de todos os usuários cadastrados.

**Requisitos:**

* Retornar uma lista em formato JSON
* Não exigir parâmetros obrigatórios

---

### 4.3 Buscar Usuário por ID

O sistema deve permitir a consulta de um usuário específico.

**Requisitos:**

* Retornar erro caso o usuário não exista

---

### 4.4 Atualizar Usuário

O sistema deve permitir a atualização dos dados de um usuário existente.

**Requisitos:**

* Apenas usuários existentes podem ser atualizados
* O email atualizado não pode entrar em conflito com outro usuário

---

### 4.5 Remover Usuário

O sistema deve permitir a remoção de um usuário.

**Requisitos:**

* A operação deve ser irreversível
* Retornar confirmação de sucesso ou erro

---

## 5. Requisitos Técnicos

* API REST
* Comunicação via JSON
* Persistência de dados local (banco relacional)
* Tratamento adequado de erros
* Código organizado em módulos

---

## 6. Requisitos Não Funcionais

* O sistema deve ser simples de manter
* O código deve priorizar clareza em vez de complexidade
* O projeto deve permitir expansão futura (ex: autenticação)

---

## 7. Fora de Escopo (Neste Momento)

* Autenticação e autorização
* Interface gráfica
* Paginação
* Logs avançados
* Deploy em produção

---

## 8. Considerações Finais

Este documento representa uma **requisição inicial do cliente**. O sistema poderá evoluir conforme novas necessidades forem identificadas durante o desenvolvimento.
