# AKS ALGORITHM

_Just a basic implementation of the AKS primality test algorithm in Rust._

The AKS primality test is a deterministic primality-proving algorithm, it was first presented by **Manindra Agrawal**, **Neeraj Kayal**, and **Nitin Saxena** in 2002.

This Rust implementation is just for educational purposes, it's main purpose is to understand the core concepts of the algorithm. It is not fully optimized and it is not intended to be used in production. Developed by Álvaro Buj (@buj8) for the Advanced Theory of Computation course at the UC3M (Universidad Carlos III de Madrid).

**Work in progress.**

Steps implemented:

- [x] Step 1: If $n = a^b$ with $a \in \mathbb{N}$ \and $b \gt 1$, output **COMPOSITE**.
- [ ] Step 2: Find the smallest $r \in \mathbb{N}$ such that $o_r(n) \gt \log^2 n$ with $r \lt n$.
- [ ] Step 3: If $1 \lt \gcd(a, n) \lt n$ for some $a \leq r$, output **COMPOSITE**.
- [ ] Step 4: If $n \leq r$, output **PRIME**.q
- [ ] Step 5: For $a = 1$ to $\lfloor \sqrt{\phi(r)} \log_2 n \rfloor$, if $(X + a)^n \neq X^n + a \mod (X^r - 1, n)$, output **COMPOSITE**.
