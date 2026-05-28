# Cryptography Proofs

## Intro

Of course the topic of formal cryptographic proofs is way broader than to be described in a note, I wanted to have a solid categorization and example and explanation of what types they have and when they are used.

### What different types of proofs do we have?

#### Reduction Proofs

In their definition they're so simple tbh. We are reducing a bigger claim to a smaller one and get to a contradiction meaning this could not happen. Does it make sense?
In cryptography terms the claim would be if $Y$ is secure (obviously when security has been defined correctly) then $X$ is secure. If we show that if $X$ is insecure then $Y$ is insecure would do no?
$Y$ is usually something known to be secure that we try to get to, since by trying to prove that $Y$ is insecure will result in a contradiction hence proving $X$ is secure. Am I making sense?

## Questions

- Proof of what? What security definitions do we have that we need to prove a scheme fitting in that definition?
All I'm saying is that there is some claim we are proving, proving that something is secure, but then what are the definitions of that security?
- In this [video](https://www.youtube.com/watch?v=ZwCYXPH-lu4&list=PL9mNSKC0i-d8VKahrLPoEbUJgo9BwfMQ5&index=3) minutes 3:30 he mentions that we **construct** an adversary hence this is a **constructive proof**. What other types of reduction proofs do we have?
- Are all reduction proofs game-based? what would the other type be?

## Summary

## Extra Reading

## Resources
