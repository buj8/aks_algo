# aks_algo

Just a basic implementation of the AKS primality test algorithm in Rust.

**Work in progress.**

Steps implemented:

- [x] Step 1: If $n = a^b$ with $a \in \mathbb{N}$ \and $b \gt 1$, output **COMPOSITE**.
- [ ] Step 2: Find the smallest $r \in \mathbb{N}$ such that $o_r(n) \gt \log^2 n$ with $r \lt n$.
- [ ] Step 3: If $1 \lt \gcd(a, n) \lt n$ for some $a \leq r$, output **COMPOSITE**.
- [ ] Step 4: If $n \leq r$, output **PRIME**.q
- [ ] Step 5: For $a = 1$ to $\lfloor \sqrt{\phi(r)} \log_2 n \rfloor$, if $(X + a)^n \neq X^n + a \mod (X^r - 1, n)$, output **COMPOSITE**.
