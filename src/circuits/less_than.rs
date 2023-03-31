use super::super::chips::less_than::{LessThanChip, LessThanConfig};

use halo2_proofs::{arithmetic::FieldExt, circuit::*, plonk::*};

#[derive(Default)]

// define circuit struct using array of usernames and balances
struct MyCircuit<F> {
    pub input: Value<F>,
}

impl<F: FieldExt> Circuit<F> for MyCircuit<F> {
    type Config = LessThanConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let input = meta.advice_column();

        LessThanChip::configure(meta, input)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        // We create a new instance of chip using the config passed as input
        let chip = LessThanChip::<F>::construct(config);

        // assign value to the chip
        chip.assign(layouter.namespace(|| "init table"), self.input);

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::MyCircuit;
    use halo2_proofs::{circuit::Value, dev::MockProver, halo2curves::pasta::Fp};
    #[test]
    fn test_less_than_20() {
        let k = 4;

        // initate value
        let value = Value::known(Fp::from(19));

        let circuit = MyCircuit::<Fp> {
            input: value
        };

        let prover = MockProver::run(k, &circuit, vec![]).unwrap();
        prover.assert_satisfied();
    }
}
