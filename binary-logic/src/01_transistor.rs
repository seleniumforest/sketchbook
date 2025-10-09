#[derive(Copy, Clone)]
pub struct Transistor {
    source: bool,
    drain: bool,
    gate: bool,
}

impl Transistor {
    pub fn new(source: bool, gate: bool) -> Transistor {
        return Transistor {
            source,
            drain: source && gate,
            gate,
        };
    }

    pub fn is_open(self) -> bool {
        return self.gate;
    }

    pub fn input(self) -> bool {
        return self.source;
    }

    pub fn output(self) -> bool {
        return self.drain;
    }
}

#[cfg(test)]
mod Test {
    use crate::transistor::Transistor;

    #[test]
    fn test() {
        let t1 = Transistor::new(true, true);
        assert_eq!(t1.output(), true);

        let t2 = Transistor::new(false, true);
        assert_eq!(t2.output(), false);

        let t3 = Transistor::new(true, false);
        assert_eq!(t3.output(), false);

        let t4 = Transistor::new(false, false);
        assert_eq!(t4.output(), false);
    }
}
