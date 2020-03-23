---
title: 'Contrainte globale abstractXOR : résultats de complexité et algorithmes de propagation'
authors: [Loïc Rouquette, Christine Solnon]
publication_date: 2020-01-01
resources:
 - name: Article
   icon: pdf
   url: todo
 - name: Results
   icon: time
   url: todo
 - name: Source code
   icon: code
   url: todo
 - name: Cite me
   icon: bookmark
   url: todo
 - name: Replikate configuration
   icon: cogs
   url: https://raw.githubusercontent.com/rloic/thesis-experiments/master/public/2020/02/aes/truncated/advanced.yml
---
Block cipher algorithms use shared secret keys to cipher texts. The aim of the related-key attack is to evaluate if it is possible to find out the key by studying difference propagations. To this aim, cryptanalysts need to compute maximum differential paths. This problem is solved in two steps :

* In a first step, each byte is abstracted by a boolean indicating if the byte contains a difference or not and we search for abstract solutions,
* In a second step, we search for an optimal concrete solution for each abstract solution.

Some abstract solutions may not accept concrete solutions and our goal is to limit as much as possible the number of inconsistent abstract solutions at the concrete level. These inconsistencies mainly come from the fact that ciphering massively relies on XOR operations (bitwise exclusive or), and this operation is poorly abstracted during the first step. To improve that, we introduce a constraint for propagating a set of XORs in a global way.

First we study the complexity of the problem aiming at deciding if a XOR set accepts a concrete solution. We show that if the problem is polynomial when variables are not constrained, it becomes NP-complete if we constrain some variables to take values between 1 and a given upper bound. Then, we introduce the global constraint abstractXOR which is satisfied if there exists an abstract solution that can be concretized in the case where variables are not constrained by an upper bound and we introduce polynomial algorithms to propagate this constraint. We show how to use this global constraint to model a maximal differential path problem for Midori and we compare the performance of the model using this global constraint with other models.

```bash
# Requirements
# Git
# Python3
# Java 1.8
# Gradle
git clone https://github.com/rloic/replikate
python3 replikate/replikate.py url=[https://raw.githubusercontent.com/rloic/thesis-experiments/master/public/2020/02/aes/truncated/]advanced.yml -g -b -r
```