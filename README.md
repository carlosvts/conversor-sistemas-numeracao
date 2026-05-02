# Conversor Universal de Sistemas de Numeração

Alunos: Carlos Vinícius Teixeira de Souza, João Vitor Pereira Gomes

Professor: Rafael Seraphilha Durelli

Departamento de Ciência da Computação da UFLA - Introdução à Computação

Tecnologias utilizadas: Rust, [Ratatui](https://ratatui.rs/) 

## Divisao de Responsabilidades: Projeto Conversor de Bases

Este documento detalha a distribuicao de tarefas e modulos tecnicos entre os desenvolvedores do projeto, focando em robustez de backend e experiencia do usuario (UX).

### Joao Vitor Pereira Gomes: O Mestre do Backend e Logica Arbitraria

* **Conversoes Base (F1 e F2):** Implementar divisoes sucessivas e o somatorio posicional $n=\sum S_{i} 	imes b^{i}$.
* **Desafio das Bases Arbitrarias:** Expandir a logica para aceitar qualquer base entre 2 e 36, utilizando digitos 0-9 e letras A-Z.
* **Validacao e Autodeteccao (F5 + Desafio):** Criar o parser que valida a entrada e implementa a deteccao automatica de prefixos como 0b ou 0x para inferir a base.
* **Calculadora de Maximos (F10):** Logica para calcular o maior valor representavel $(2^{k}-1)$ em todas as bases.
* **Testes de Logica:** Criar metade dos 30 casos de teste, focando em limites de bases (ex: converter entre base 3 e base 35).

### Carlos Vinicius Teixeira de Souza: O Arquiteto de UX e Fluxos Complexos

* **Agrupamento e Intermediarios (F3 e F4):** Implementar conversoes diretas entre binario, octal e hexadecimal sem passar pelo decimal.
* **Numeros Fracionarios (F6):** Implementar a logica de multiplicacoes sucessivas para a parte fracionaria com limite de 16 casas.
* **Modo Passo-a-Passo (F7):** Desenvolver o formatador que gera a tabela de divisoes e o trace do algoritmo.
* **Interface Grafica (Desafio):** Desenvolver a GUI (Tkinter se for Python, JavaFX se for Java) que integre todas as funcoes da CLI.
* **Processamento Batch e Quiz (F8 e F9):** Logica para leitura de CSV e o sistema de pontuacao com 5 niveis de dificuldade.
