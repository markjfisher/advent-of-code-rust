pub struct Comp {
    pub pc: usize,
    pub reg_a: i64,
    pub reg_b: i64,
    pub reg_c: i64,
    pub program: Vec<usize>,
    pub output: Vec<i64>,
}

impl Comp {
    pub fn new(input: &[usize]) -> Self {
        let reg_a = input[0] as i64;
        let reg_b = input[1] as i64;
        let reg_c = input[2] as i64;
        let program = input[3..].to_vec();
        Self { pc: 0, reg_a, reg_b, reg_c, program, output: vec![] }
    }

    pub fn run(&mut self) {
        while self.pc < self.program.len() {
            self.step();
        }
    }

    fn step(&mut self) {
        let opcode = self.program[self.pc];

        match opcode {
            0 => self.op_adv(),
            1 => self.op_bxl(),
            2 => self.op_bst(),
            3 => self.op_jnz(),
            4 => self.op_bxc(),
            5 => self.op_out(),
            6 => self.op_bdv(),
            7 => self.op_cdv(),
            _ => panic!("Invalid opcode: {}", opcode),
        }
        if opcode != 3 {
            self.pc += 2;
        }
    }

    fn combo_op(&mut self) -> i64 {
        let operand = self.program[self.pc + 1];
        let value = match operand {
            0..=3 => operand as i64,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("Invalid operand: {}", operand),
        };
        value
    }

    fn op_adv(&mut self) {
        self.reg_a = self.reg_a >> self.combo_op();
    }

    fn op_bxl(&mut self) {
        self.reg_b ^= self.program[self.pc + 1] as i64;
    }

    fn op_bst(&mut self) {
        self.reg_b = self.combo_op() & 0x07;
    }

    fn op_jnz(&mut self) {
        if self.reg_a != 0 {
            self.pc = self.program[self.pc + 1] as usize;
        } else {
            self.pc += 2;
        }
    }

    fn op_bxc(&mut self) {
        self.reg_b = self.reg_b ^ self.reg_c;
    }

    fn op_out(&mut self) {
        let value = self.combo_op() & 0x07;
        self.output.push(value);
    }

    fn op_bdv(&mut self) {
        self.reg_b = self.reg_a >> self.combo_op();
    }

    fn op_cdv(&mut self) {
        self.reg_c = self.reg_a >> self.combo_op();
    }

    pub fn get_output(&self) -> String {
        self.output.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(",")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_input() {
        let input = vec![0, 1, 2, 3, 4, 5];
        let comp = Comp::new(&input);
        assert_eq!(comp.pc, 0);
        assert_eq!(comp.reg_a, 0);
        assert_eq!(comp.reg_b, 1);
        assert_eq!(comp.reg_c, 2);
        assert_eq!(comp.program, vec![3, 4, 5]);
    }

    #[test]
    fn can_run_program_1() {
        let input = vec![0, 0, 9, 2, 6];
        let mut comp = Comp::new(&input);
        comp.run();
        assert_eq!(comp.reg_b, 1);
    }

    #[test]
    fn can_run_program_2() {
        let input = vec![10, 0, 0, 5, 0, 5, 1, 5, 4];
        let mut comp = Comp::new(&input);
        comp.run();
        assert_eq!(comp.get_output(), "0,1,2");
    }

    #[test]
    fn can_run_program_3() {
        let input = vec![2024, 0, 0, 0, 1, 5, 4, 3, 0];
        let mut comp = Comp::new(&input);
        comp.run();
        assert_eq!(comp.get_output(), "4,2,5,6,7,7,7,7,3,1,0");
        assert_eq!(comp.reg_a, 0);
    }

    #[test]
    fn can_run_program_4() {
        let input = vec![0, 29, 0, 1, 7];
        let mut comp = Comp::new(&input);
        comp.run();
        assert_eq!(comp.reg_b, 26);
    }

    #[test]
    fn can_run_program_5() {
        let input = vec![0, 2024, 43690, 4, 0];
        let mut comp = Comp::new(&input);
        comp.run();
        assert_eq!(comp.reg_b, 44354);
    }

    #[test]
    fn can_run_program_real() {
        let input = vec![64196994, 0, 0, 2, 4, 1, 1, 7, 5, 1, 5, 4, 0, 0, 3, 5, 5, 3, 0];
        let mut comp = Comp::new(&input);
        comp.run();
        assert_eq!(comp.get_output(), "6,4,6,0,4,5,7,2,7");
    }

    #[test]
    fn copies_self() {
        let input = vec![117440, 0, 0, 0, 3, 5, 4, 3, 0];
        let mut comp = Comp::new(&input);
        comp.run();
        assert_eq!(comp.get_output(), "0,3,5,4,3,0");
    }
}
