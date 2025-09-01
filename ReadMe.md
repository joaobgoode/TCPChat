
# Chat TCP Cliente-Servidor

## Descrição

Este projeto implementa um chat simples usando **TCP**, permitindo comunicação entre **um servidor** e **vários clientes**. Cada participante escolhe um **nickname** único, e mensagens são enviadas para todos os conectados, exceto o remetente.

## Regras do Sistema

### Servidor

* Deve ser iniciado antes dos clientes.
* Escolhe um **nickname próprio** ao iniciar.
* Aceita múltiplos clientes, mantendo uma lista de nicknames para evitar duplicatas.
* Pode enviar mensagens que são transmitidas a todos os clientes conectados.

### Clientes

* Podem se conectar apenas se o servidor estiver ativo.
* Devem informar o **IP e porta** do servidor.
* Escolhem um **nickname único** ao se conectar.
* Se o nickname já existir (incluindo o do servidor), o cliente deve escolher outro.
* Recebem mensagens de todos os participantes conectados.

## Fluxo de Conexão

### Iniciar Servidor

```bash
$ ./chat_server
Servidor rodando na porta 9000
Digite seu nickname (servidor):
```

* O servidor escolhe seu nickname.
* A partir desse momento, está pronto para aceitar conexões.

### Cliente se conecta

```bash
$ ./chat_client
Digite o IP e porta do servidor (ex: 192.168.0.10:9000):
```

* Cliente envia seu nickname ao servidor.
* Servidor verifica se já existe um participante com o mesmo nickname (incluindo ele próprio).

### Verificação de Nickname

* Se o nickname **não existir** → servidor envia `Ok` → conexão aprovada.
* Se o nickname **existir** → servidor envia mensagem de erro → cliente deve tentar outro nickname.

### Comunicação

* Todos os participantes podem enviar mensagens.
* Mensagens são transmitidas para **todos os outros participantes**, incluindo o servidor.
* O servidor também pode enviar mensagens digitando no console principal.

## Exemplo de Uso

### Servidor

```text
Servidor rodando na porta 9000
Digite seu nickname (servidor): Admin
```

### Cliente

```text
Digite o IP e porta do servidor: 192.168.0.10:9000
Digite seu nickname: João
Bem-vindo João!
```

### Cliente tentando nickname existente

```text
Digite seu nickname: Admin
Já existe um cliente com esse nickname, tente outro.
Digite seu nickname: Maria
Bem-vindo Maria!
```

## Observações

* Apenas **um servidor** pode estar ativo por vez.
* Nicknames **não podem se repetir**, incluindo o do servidor.
* Se um cliente ou servidor se desconectar, os outros participantes são notificados.
