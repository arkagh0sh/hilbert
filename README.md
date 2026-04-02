---
math: true
footnotes: true
---

# Oligo-poly

## A library for working with polynomial rings under oligomorphic group actions.

**The most updated version of this document can be found at**


The purpose of this library is to implement (in Rust)the equivariant Buchberger algorithm given in the following paper.

[GL25] : Arka Ghosh, Aliaume Lopez: Computability of Equivariant Gröbner bases.



# Theoretical background

## Relational structures

Let $\mathcal{X} = (X,R_1,\dots,R_n)$ be a **relational structure**.
By this we mean that $X$ is a set (called the **domain** of the relational structure), $R_i$ are relations defined on the set $X$.

For example,
we can consider the structure $\mathcal{Q} = (Q,<)$ where $Q$ is the set of rational numbers and $<$ is the usual ordering.

A graph $\mathcal{G} = (V,E)$ is another example of a relational structure,
with the set $V$ of vertices being the domain and the edge relation $E$ being the only relation.

By abuse of notation we sometime use $\mathcal{X}$ to denote the set underlying $X$.
For instance,
we write $\bar{x}\in\mathcal{X}^n$ to mean that $x$ is an element of $X^n$.

## Automorphism groups

An **automorphism** of the relational structure $\mathcal{X}$ is a bijection from $\mathcal{X}$ to itself that preserves and reflects the structure.

For instance, an automorphism of $\mathcal{Q}$ is an order preserving bijection from $\mathcal{Q}$ to itself.

If $\mathcal{X}$ is a graph,
then an automorphism of $\mathcal{X}$ is an isomorphism of the graph with itself.

The set of automorphisms of a relational structure $\mathcal{X}$ is denoted as $\mathbf{Aut}(\mathcal{X})$.

## Equivariant Ideals

$\mathbb{Q}[\mathcal{X}]$

## Equivariant Gröbner bases

## Equivariant Buchberger algorithm

# How to use

## Relational structures

### Pre-defined relational structures

#### Dense linear order without endpoints

### Defining new relational structures

## References
