use crate::modular_arithmetic::{mod_inv, mod_power};
use num_bigint::BigUint;
use rand::{Rng, rng};

pub struct ElGamal {
    pub prime: BigUint,
    pub generator: BigUint,
}
pub struct ElGamalKeyPair {
    pub secret_key: BigUint,
    pub public_key: BigUint,
}

pub struct ElGamalCipherText {
    pub cipher_one: BigUint,
    pub cipher_two: BigUint,
}

impl ElGamal {
    pub fn key_gen_random(&self) -> ElGamalKeyPair {
        let secret_key = self.get_rand_in_field();
        let public_key = mod_power(&self.generator, &self.prime, &secret_key);
        return ElGamalKeyPair {
            secret_key,
            public_key,
        };
    }
    pub fn key_gen_with_sk(&self, secret_key: &BigUint) -> ElGamalKeyPair {
        let public_key = mod_power(&self.generator, &self.prime, &secret_key);
        return ElGamalKeyPair {
            secret_key: secret_key.clone(),
            public_key: public_key,
        };
    }
    pub fn elgamal_enc(
        &self,
        public_key: &BigUint,
        message: &BigUint,
        randomness_input: Option<&BigUint>,
    ) -> ElGamalCipherText {
        if *message >= self.prime {
            panic!("Message is bigger than the field prime");
        }
        let randomness = match randomness_input {
            Some(_) => randomness_input.unwrap(),
            None => &self.get_rand_in_field(),
        };
        let ciphertext_one = mod_power(&self.generator, &self.prime, randomness);
        let ciphertext_two =
            (mod_power(&public_key, &self.prime, randomness) * message) % &self.prime;
        return ElGamalCipherText {
            cipher_one: ciphertext_one,
            cipher_two: ciphertext_two,
        };
    }
    pub fn elgamal_dec(&self, secret_key: &BigUint, cipher_text: &ElGamalCipherText) -> BigUint {
        let pub_keys_multiplied = mod_power(&cipher_text.cipher_one, &self.prime, &secret_key);
        let pub_keys_inv = mod_inv(&pub_keys_multiplied, &self.prime);
        let message = (pub_keys_inv * &cipher_text.cipher_two) % &self.prime;
        return message;
    }
    fn get_rand_in_field(&self) -> BigUint {
        let mut randomness_bytes = vec![0u8; (self.prime.bits() / 8) as usize];
        rng().fill_bytes(&mut randomness_bytes);
        let randomness = BigUint::from_bytes_be(&randomness_bytes) % &self.prime;
        return randomness;
    }
}
#[cfg(test)]

mod tests {
    use num_bigint::{BigUint, ToBigUint};
    use std::str::FromStr;

    use crate::elgamal_enc::ElGamal;
    #[test]
    fn test_elgamal_correctness() {
        let elgamal = ElGamal {
            prime: 71.to_biguint().unwrap(),
            generator: 33.to_biguint().unwrap(),
        };
        let key_pair = elgamal.key_gen_random();
        let message = 54.to_biguint().unwrap();
        assert!(
            elgamal.elgamal_dec(
                &key_pair.secret_key,
                &elgamal.elgamal_enc(&key_pair.public_key, &message, None)
            ) == message
        );
    }
    #[test]
    fn test_elgamal_test_cases() {
        let elgamal = ElGamal{
            prime: BigUint::from_str("12658517083168187407924345155971956101250996576825115113297001855799796437288935576230034157578333666497170430505565580165565829633685607504706642034926119").unwrap(),
            generator: 7_u64.to_biguint().unwrap()
        };
        let key_pair = elgamal.key_gen_with_sk(
            &BigUint::from_str("2001688878140630728014209681954697141876038523595247208").unwrap(),
        );
        let message = BigUint::from_str("87521618088882658227876453").unwrap();
        let cipher_text = elgamal.elgamal_enc(
            &key_pair.public_key,
            &message,
            Some(
                &BigUint::from_str("5446024688717452254835115775456957961297236108858862823")
                    .unwrap(),
            ),
        );
        assert!(cipher_text.cipher_one == BigUint::from_str("2150519483988769855483983776372336742288374425191291528256965705108393490638750082340115568718132372731853110762124400441550538499580316268601341087676203").unwrap());
        assert!(cipher_text.cipher_two == BigUint::from_str("1540471266850557563382406324432354117072109094950140952195099581066490559252112349492583688225692526496193879919152401794896907007394565292272866724291488").unwrap());
    }
}
