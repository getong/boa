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
                match op {
                    ArithmeticOp::Add => self.emit_opcode(Opcode::Add),
                    ArithmeticOp::Sub => self.emit_opcode(Opcode::Sub),
                    ArithmeticOp::Div => self.emit_opcode(Opcode::Div),
                    ArithmeticOp::Mul => self.emit_opcode(Opcode::Mul),
                    ArithmeticOp::Exp => self.emit_opcode(Opcode::Pow),
                    ArithmeticOp::Mod => self.emit_opcode(Opcode::Mod),
                }

                if !use_expr {
                    self.emit_opcode(Opcode::Pop);
                }
            }
            BinaryOp::Bitwise(op) => {
                self.compile_expr(binary.rhs(), true);
                match op {
                    BitwiseOp::And => self.emit_opcode(Opcode::BitAnd),
                    BitwiseOp::Or => self.emit_opcode(Opcode::BitOr),
                    BitwiseOp::Xor => self.emit_opcode(Opcode::BitXor),
                    BitwiseOp::Shl => self.emit_opcode(Opcode::ShiftLeft),
                    BitwiseOp::Shr => self.emit_opcode(Opcode::ShiftRight),
                    BitwiseOp::UShr => self.emit_opcode(Opcode::UnsignedShiftRight),
                }

                if !use_expr {
                    self.emit_opcode(Opcode::Pop);
                }
            }
            BinaryOp::Relational(op) => {
                self.compile_expr(binary.rhs(), true);
                let needs_use = match op {
                    RelationalOp::Equal => {
                        self.emit_opcode(Opcode::Eq);
                        true
                    }
                    RelationalOp::NotEqual => {
                        self.emit_opcode(Opcode::NotEq);
                        true
                    }
                    RelationalOp::StrictEqual => {
                        let lhs = self.register_allocator.alloc();
                        let rhs = self.register_allocator.alloc();
                        self.emit2(Opcode::PopIntoRegister, &[Operand2::Varying(rhs.index())]);
                        self.emit2(Opcode::PopIntoRegister, &[Operand2::Varying(lhs.index())]);
                        self.emit2(
                            Opcode::StrictEq,
                            &[*output, Operand2::Register(&lhs), Operand2::Register(&rhs)],
                        );
                        self.register_allocator.dealloc(lhs);
                        self.register_allocator.dealloc(rhs);
                        false
                    }
                    RelationalOp::StrictNotEqual => {
                        let lhs = self.register_allocator.alloc();
                        let rhs = self.register_allocator.alloc();
                        self.emit2(Opcode::PopIntoRegister, &[Operand2::Varying(rhs.index())]);
                        self.emit2(Opcode::PopIntoRegister, &[Operand2::Varying(lhs.index())]);
                        self.emit2(
                            Opcode::StrictNotEq,
                            &[*output, Operand2::Register(&lhs), Operand2::Register(&rhs)],
                        );
                        self.register_allocator.dealloc(lhs);
                        self.register_allocator.dealloc(rhs);
                        false
                    }
                    RelationalOp::GreaterThan => {
                        self.emit_opcode(Opcode::GreaterThan);
                        true
                    }
                    RelationalOp::GreaterThanOrEqual => {
                        self.emit_opcode(Opcode::GreaterThanOrEq);
                        true
                    }
                    RelationalOp::LessThan => {
                        self.emit_opcode(Opcode::LessThan);
                        true
                    }
                    RelationalOp::LessThanOrEqual => {
                        self.emit_opcode(Opcode::LessThanOrEq);
                        true
                    }
                    RelationalOp::In => {
                        self.emit_opcode(Opcode::In);
                        true
                    }
                    RelationalOp::InstanceOf => {
                        self.emit_opcode(Opcode::InstanceOf);
                        true
                    }
                };

                if !use_expr && needs_use {
                    self.emit_opcode(Opcode::Pop);
                }

                return needs_use;
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
            }
            BinaryOp::Comma => {
                self.emit_opcode(Opcode::Pop);
                self.compile_expr(binary.rhs(), true);

                if !use_expr {
                    self.emit_opcode(Opcode::Pop);
                }
            }
        };
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
