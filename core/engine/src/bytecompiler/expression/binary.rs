use boa_ast::expression::operator::{
    binary::{ArithmeticOp, BinaryOp, BitwiseOp, LogicalOp, RelationalOp},
    Binary, BinaryInPrivate,
};

use crate::{
    bytecompiler::{ByteCompiler, Operand2},
    vm::Opcode,
};

impl ByteCompiler<'_> {
    pub(crate) fn compile_binary(
        &mut self,
        binary: &Binary,
        output: &mut Operand2<'_>,
        use_expr: bool,
    ) -> bool {
        self.compile_expr(binary.lhs(), true);

        match binary.op() {
            BinaryOp::Arithmetic(op) => {
                self.compile_expr(binary.rhs(), true);

                let rhs = self.register_allocator.alloc();
                let lhs = self.register_allocator.alloc();

                self.emit2(Opcode::PopIntoRegister, &[Operand2::Varying(rhs.index())]);
                self.emit2(Opcode::PopIntoRegister, &[Operand2::Varying(lhs.index())]);

                let opcode = match op {
                    ArithmeticOp::Add => Opcode::Add,
                    ArithmeticOp::Sub => Opcode::Sub,
                    ArithmeticOp::Div => Opcode::Div,
                    ArithmeticOp::Mul => Opcode::Mul,
                    ArithmeticOp::Exp => Opcode::Pow,
                    ArithmeticOp::Mod => Opcode::Mod,
                };

                self.emit2(
                    opcode,
                    &[*output, Operand2::Register(&lhs), Operand2::Register(&rhs)],
                );
                self.register_allocator.dealloc(lhs);
                self.register_allocator.dealloc(rhs);
            }
            BinaryOp::Bitwise(op) => {
                self.compile_expr(binary.rhs(), true);

                let rhs = self.register_allocator.alloc();
                let lhs = self.register_allocator.alloc();

                self.emit2(Opcode::PopIntoRegister, &[Operand2::Varying(rhs.index())]);
                self.emit2(Opcode::PopIntoRegister, &[Operand2::Varying(lhs.index())]);
                let opcode = match op {
                    BitwiseOp::And => Opcode::BitAnd,
                    BitwiseOp::Or => Opcode::BitOr,
                    BitwiseOp::Xor => Opcode::BitXor,
                    BitwiseOp::Shl => Opcode::ShiftLeft,
                    BitwiseOp::Shr => Opcode::ShiftRight,
                    BitwiseOp::UShr => Opcode::UnsignedShiftRight,
                };

                self.emit2(
                    opcode,
                    &[*output, Operand2::Register(&lhs), Operand2::Register(&rhs)],
                );
                self.register_allocator.dealloc(lhs);
                self.register_allocator.dealloc(rhs);
            }
            BinaryOp::Relational(op) => {
                self.compile_expr(binary.rhs(), true);

                let rhs = self.register_allocator.alloc();
                let lhs = self.register_allocator.alloc();

                self.emit2(Opcode::PopIntoRegister, &[Operand2::Varying(rhs.index())]);
                self.emit2(Opcode::PopIntoRegister, &[Operand2::Varying(lhs.index())]);
                let opcode = match op {
                    RelationalOp::Equal => Opcode::Eq,
                    RelationalOp::NotEqual => Opcode::NotEq,
                    RelationalOp::StrictEqual => Opcode::StrictEq,
                    RelationalOp::StrictNotEqual => Opcode::StrictNotEq,
                    RelationalOp::GreaterThan => Opcode::GreaterThan,
                    RelationalOp::GreaterThanOrEqual => Opcode::GreaterThanOrEq,
                    RelationalOp::LessThan => Opcode::LessThan,
                    RelationalOp::LessThanOrEqual => Opcode::LessThanOrEq,
                    RelationalOp::In => Opcode::In,
                    RelationalOp::InstanceOf => Opcode::InstanceOf,
                };

                self.emit2(
                    opcode,
                    &[*output, Operand2::Register(&lhs), Operand2::Register(&rhs)],
                );
                self.register_allocator.dealloc(lhs);
                self.register_allocator.dealloc(rhs);
            }
            BinaryOp::Logical(op) => {
                match op {
                    LogicalOp::And => {
                        let exit = self.emit_opcode_with_operand(Opcode::LogicalAnd);
                        self.compile_expr(binary.rhs(), true);
                        self.patch_jump(exit);
                    }
                    LogicalOp::Or => {
                        let exit = self.emit_opcode_with_operand(Opcode::LogicalOr);
                        self.compile_expr(binary.rhs(), true);
                        self.patch_jump(exit);
                    }
                    LogicalOp::Coalesce => {
                        let exit = self.emit_opcode_with_operand(Opcode::Coalesce);
                        self.compile_expr(binary.rhs(), true);
                        self.patch_jump(exit);
                    }
                };

                if !use_expr {
                    self.emit_opcode(Opcode::Pop);
                }

                return false;
            }
            BinaryOp::Comma => {
                self.emit_opcode(Opcode::Pop);
                self.compile_expr(binary.rhs(), true);

                if !use_expr {
                    self.emit_opcode(Opcode::Pop);
                }
                return false;
            }
        }

        true
    }

    pub(crate) fn compile_binary_in_private(&mut self, binary: &BinaryInPrivate, use_expr: bool) {
        let index = self.get_or_insert_private_name(*binary.lhs());
        self.compile_expr(binary.rhs(), true);
        self.emit_with_varying_operand(Opcode::InPrivate, index);

        if !use_expr {
            self.emit_opcode(Opcode::Pop);
        }
    }
}
