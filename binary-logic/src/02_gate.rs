use crate::transistor::Transistor;

pub fn nand(input_a: bool, input_b: bool) -> bool {
    let gate1 = and(input_a, input_b);
    let gate2 = not(gate1);

    return gate2;
}

pub fn xor(input_a: bool, input_b: bool) -> bool {
    let gate1 = or(input_a, input_b);
    let gate2 = nand(input_a, input_b);
    let gate3 = and(gate1, gate2);

    return gate3;
}

pub fn or(input_a: bool, input_b: bool) -> bool {
    let tr1 = Transistor::new(true, input_a);
    let tr2 = Transistor::new(true, input_b);
    let out = tr1.output() || tr2.output();

    return out;
}

pub fn and(input_a: bool, input_b: bool) -> bool {
    let tr1 = Transistor::new(true, input_a);
    let tr2 = Transistor::new(tr1.output(), input_b);

    return tr2.output();
}

pub fn not(input: bool) -> bool {
    let tr = Transistor::new(input, true);

    return !tr.output();
}

#[cfg(test)]
mod GateTest {
    use crate::gate::{and, nand, or, xor};

    #[test]
    fn test_xor() {
        let g1 = xor(true, true);
        let g2 = xor(false, true);
        let g3 = xor(true, false);
        let g4 = xor(false, false);

        assert_eq!(g1, false);
        assert_eq!(g2, true);
        assert_eq!(g3, true);
        assert_eq!(g4, false);
    }

    #[test]
    fn test_nand() {
        let g1 = nand(true, true);
        let g2 = nand(false, true);
        let g3 = nand(true, false);
        let g4 = nand(false, false);

        assert_eq!(g1, false);
        assert_eq!(g2, true);
        assert_eq!(g3, true);
        assert_eq!(g4, true);
    }

    #[test]
    fn test_and() {
        let g1 = and(true, true);
        let g2 = and(false, true);
        let g3 = and(true, false);
        let g4 = and(false, false);

        assert_eq!(g1, true);
        assert_eq!(g2, false);
        assert_eq!(g3, false);
        assert_eq!(g4, false);
    }

    #[test]
    fn test_or() {
        let g1 = or(true, true);
        let g2 = or(false, true);
        let g3 = or(true, false);
        let g4 = or(false, false);

        assert_eq!(g1, true);
        assert_eq!(g2, true);
        assert_eq!(g3, true);
        assert_eq!(g4, false);
    }
}
