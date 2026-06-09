# Number Theory, Abelian Groups, and ElGamal Encryption

## Notes

- fast powering algorithm, idea use the binary expansion of the exponent. We want to compute $g$ to the power of $A$ and we look at the binary expansion of $A$ for that. It's called " Fast Powering Algorithm and the Square-and-Multiply Algorithm".
- Bezout's identity:
    if integers a and b are relatively prime (gcd(a,b)=1), then there exists integers x and y for which $1= ax + by$.

## Exercises

### Fast Exponentiation Algorithm

Most cryptographic schemes use modular arithmetics and require exponentiation of very big numbers; e.g. $(2 ^ {256}) ^ (2^{256}) \; \mod 2^{256}$.
If we were to naively do these exponentiations it would take way more than a couple of years to do it. The fast exponentiation algorithm we saw in this week, is called double and add. Basically if we represent the exponent $e$ in binary form of $\{001011,\cdots,101\}_{256}$ we can write it as:
    $$e = 2^2 + 2^4 + 2^5 + \cdots,  2^{253} + 2 ^ {255}$$
then as we can see we have to do at most 256 doubling of the base and 256 adds.
You can find the implementation of this algorithm in the code/src/modular_arithmetic.rs as mod_pow().

### Fermat's Little Theorem

Basically what Fermat's little theorem says is that if we are in a $\mathbb{Z}_p$ where $p$ is prime and $ a < p $ then $a ^ {p-1} = 1 \mod p$.
To prove this we instead prove that $a ^ {p} = a \mod p$ through induction.
For the base case we can see that $1 ^{p} = 1 \mod p$.
We assume that $a^{p} = a \mod p$ then $ (a+1)^p = a^p +  \binom{p}{1} a ^ {p-1} * 1 +  \binom{p}{2} a ^ {p-2} * 1^2 + \cdots +  \binom{p}{1} a  * 1^{p-1} + 1 ^p $. In each of the binomial phrases we have $p$ multiplied by a value hence in modulus $p$ all these phrases will result to zero. The only two phrases left will be $a ^p + 1 ^ p$ hence:
    $$(a + 1) ^ p = a ^ p + 1 \mod p $$
We assumed $a ^ p = a \mod p$ resulting in:
    $$ (a+ 1) ^ p = a + 1 \mod p$$
By induction we have now proved that $a^p = a \mod p$ for all values of $a$, and going back to our initial claim we have:
    $$a ^{p-1} = 1 \mod p$$

### Modular Inverse

Fermat's little theorem claims that for a prime $p$ and $a ; a<p$, $a ^ {p-1} = 1 \mod p$. Modular inverse in modulus $p$ for $a$ is the value $b$ s.t $ab = 1 \mod p$, since $a ^{p-1} = 1 \mod p$ then $ab = a ^{p-1} = a \times a ^{p-2} \mod p$. The modular inverse implementation can be found in code/src/modular_arithmetic.rs as mod_inv() using the previous logic.

### El Gamal Encryption

#### For Discrete Log Problem

El Gamal is a public key cryptosystem depending on the hardness of solving discrete logarithm problem in large integers. For a prime $p$ and generator $g$ the key pair and the encryption/decryption algorithm for message $m$ are as below:
    $$sk \in \mathbb{Z}_p $$
    $$pk = g ^ {sk} \mod p$$
    $$ k \in \mathbb{Z}_p$$
    $$Enc : \; c_1 = g ^ k \; , \; c_2 = {pk}^k \times m$$
    $$Dec : \; d_1 = c_1 ^ {sk}, \; m = c_2 \times d_1 ^ {-1}$$
The implementation for this algorithm can be found in code/src/elgamal_encryption.rs.
