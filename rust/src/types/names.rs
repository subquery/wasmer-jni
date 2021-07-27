pub const UNREACHABLE: u8 = 0x00;
pub const NOP: u8 = 0x01;
pub const BLOCK: u8 = 0x02;
pub const LOOP: u8 = 0x03;
pub const IF: u8 = 0x04;
pub const ELSE: u8 = 0x05;
pub const END: u8 = 0x0b;
pub const BR: u8 = 0x0c;
pub const BRIF: u8 = 0x0d;
pub const BRTABLE: u8 = 0x0e;
pub const RETURN: u8 = 0x0f;
pub const CALL: u8 = 0x10;
pub const CALLINDIRECT: u8 = 0x11;
pub const DROP: u8 = 0x1a;
pub const SELECT: u8 = 0x1b;
pub const GETLOCAL: u8 = 0x20;
pub const SETLOCAL: u8 = 0x21;
pub const TEELOCAL: u8 = 0x22;
pub const GETGLOBAL: u8 = 0x23;
pub const SETGLOBAL: u8 = 0x24;
pub const I32LOAD: u8 = 0x28;
pub const I64LOAD: u8 = 0x29;
pub const F32LOAD: u8 = 0x2a;
pub const F64LOAD: u8 = 0x2b;
pub const I32LOAD8S: u8 = 0x2c;
pub const I32LOAD8U: u8 = 0x2d;
pub const I32LOAD16S: u8 = 0x2e;
pub const I32LOAD16U: u8 = 0x2f;
pub const I64LOAD8S: u8 = 0x30;
pub const I64LOAD8U: u8 = 0x31;
pub const I64LOAD16S: u8 = 0x32;
pub const I64LOAD16U: u8 = 0x33;
pub const I64LOAD32S: u8 = 0x34;
pub const I64LOAD32U: u8 = 0x35;
pub const I32STORE: u8 = 0x36;
pub const I64STORE: u8 = 0x37;
pub const F32STORE: u8 = 0x38;
pub const F64STORE: u8 = 0x39;
pub const I32STORE8: u8 = 0x3a;
pub const I32STORE16: u8 = 0x3b;
pub const I64STORE8: u8 = 0x3c;
pub const I64STORE16: u8 = 0x3d;
pub const I64STORE32: u8 = 0x3e;
pub const CURRENTMEMORY: u8 = 0x3f;
pub const GROWMEMORY: u8 = 0x40;
pub const I32CONST: u8 = 0x41;
pub const I64CONST: u8 = 0x42;
pub const F32CONST: u8 = 0x43;
pub const F64CONST: u8 = 0x44;
pub const I32EQZ: u8 = 0x45;
pub const I32EQ: u8 = 0x46;
pub const I32NE: u8 = 0x47;
pub const I32LTS: u8 = 0x48;
pub const I32LTU: u8 = 0x49;
pub const I32GTS: u8 = 0x4a;
pub const I32GTU: u8 = 0x4b;
pub const I32LES: u8 = 0x4c;
pub const I32LEU: u8 = 0x4d;
pub const I32GES: u8 = 0x4e;
pub const I32GEU: u8 = 0x4f;
pub const I64EQZ: u8 = 0x50;
pub const I64EQ: u8 = 0x51;
pub const I64NE: u8 = 0x52;
pub const I64LTS: u8 = 0x53;
pub const I64LTU: u8 = 0x54;
pub const I64GTS: u8 = 0x55;
pub const I64GTU: u8 = 0x56;
pub const I64LES: u8 = 0x57;
pub const I64LEU: u8 = 0x58;
pub const I64GES: u8 = 0x59;
pub const I64GEU: u8 = 0x5a;

pub const F32EQ: u8 = 0x5b;
pub const F32NE: u8 = 0x5c;
pub const F32LT: u8 = 0x5d;
pub const F32GT: u8 = 0x5e;
pub const F32LE: u8 = 0x5f;
pub const F32GE: u8 = 0x60;

pub const F64EQ: u8 = 0x61;
pub const F64NE: u8 = 0x62;
pub const F64LT: u8 = 0x63;
pub const F64GT: u8 = 0x64;
pub const F64LE: u8 = 0x65;
pub const F64GE: u8 = 0x66;

pub const I32CLZ: u8 = 0x67;
pub const I32CTZ: u8 = 0x68;
pub const I32POPCNT: u8 = 0x69;
pub const I32ADD: u8 = 0x6a;
pub const I32SUB: u8 = 0x6b;
pub const I32MUL: u8 = 0x6c;
pub const I32DIVS: u8 = 0x6d;
pub const I32DIVU: u8 = 0x6e;
pub const I32REMS: u8 = 0x6f;
pub const I32REMU: u8 = 0x70;
pub const I32AND: u8 = 0x71;
pub const I32OR: u8 = 0x72;
pub const I32XOR: u8 = 0x73;
pub const I32SHL: u8 = 0x74;
pub const I32SHRS: u8 = 0x75;
pub const I32SHRU: u8 = 0x76;
pub const I32ROTL: u8 = 0x77;
pub const I32ROTR: u8 = 0x78;

pub const I64CLZ: u8 = 0x79;
pub const I64CTZ: u8 = 0x7a;
pub const I64POPCNT: u8 = 0x7b;
pub const I64ADD: u8 = 0x7c;
pub const I64SUB: u8 = 0x7d;
pub const I64MUL: u8 = 0x7e;
pub const I64DIVS: u8 = 0x7f;
pub const I64DIVU: u8 = 0x80;
pub const I64REMS: u8 = 0x81;
pub const I64REMU: u8 = 0x82;
pub const I64AND: u8 = 0x83;
pub const I64OR: u8 = 0x84;
pub const I64XOR: u8 = 0x85;
pub const I64SHL: u8 = 0x86;
pub const I64SHRS: u8 = 0x87;
pub const I64SHRU: u8 = 0x88;
pub const I64ROTL: u8 = 0x89;
pub const I64ROTR: u8 = 0x8a;
pub const F32ABS: u8 = 0x8b;
pub const F32NEG: u8 = 0x8c;
pub const F32CEIL: u8 = 0x8d;
pub const F32FLOOR: u8 = 0x8e;
pub const F32TRUNC: u8 = 0x8f;
pub const F32NEAREST: u8 = 0x90;
pub const F32SQRT: u8 = 0x91;
pub const F32ADD: u8 = 0x92;
pub const F32SUB: u8 = 0x93;
pub const F32MUL: u8 = 0x94;
pub const F32DIV: u8 = 0x95;
pub const F32MIN: u8 = 0x96;
pub const F32MAX: u8 = 0x97;
pub const F32COPYSIGN: u8 = 0x98;
pub const F64ABS: u8 = 0x99;
pub const F64NEG: u8 = 0x9a;
pub const F64CEIL: u8 = 0x9b;
pub const F64FLOOR: u8 = 0x9c;
pub const F64TRUNC: u8 = 0x9d;
pub const F64NEAREST: u8 = 0x9e;
pub const F64SQRT: u8 = 0x9f;
pub const F64ADD: u8 = 0xa0;
pub const F64SUB: u8 = 0xa1;
pub const F64MUL: u8 = 0xa2;
pub const F64DIV: u8 = 0xa3;
pub const F64MIN: u8 = 0xa4;
pub const F64MAX: u8 = 0xa5;
pub const F64COPYSIGN: u8 = 0xa6;

pub const I32WRAPI64: u8 = 0xa7;
pub const I32TRUNCSF32: u8 = 0xa8;
pub const I32TRUNCUF32: u8 = 0xa9;
pub const I32TRUNCSF64: u8 = 0xaa;
pub const I32TRUNCUF64: u8 = 0xab;
pub const I64EXTENDSI32: u8 = 0xac;
pub const I64EXTENDUI32: u8 = 0xad;
pub const I64TRUNCSF32: u8 = 0xae;
pub const I64TRUNCUF32: u8 = 0xaf;
pub const I64TRUNCSF64: u8 = 0xb0;
pub const I64TRUNCUF64: u8 = 0xb1;
pub const F32CONVERTSI32: u8 = 0xb2;
pub const F32CONVERTUI32: u8 = 0xb3;
pub const F32CONVERTSI64: u8 = 0xb4;
pub const F32CONVERTUI64: u8 = 0xb5;
pub const F32DEMOTEF64: u8 = 0xb6;
pub const F64CONVERTSI32: u8 = 0xb7;
pub const F64CONVERTUI32: u8 = 0xb8;
pub const F64CONVERTSI64: u8 = 0xb9;
pub const F64CONVERTUI64: u8 = 0xba;
pub const F64PROMOTEF32: u8 = 0xbb;

pub const I32REINTERPRETF32: u8 = 0xbc;
pub const I64REINTERPRETF64: u8 = 0xbd;
pub const F32REINTERPRETI32: u8 = 0xbe;
pub const F64REINTERPRETI64: u8 = 0xbf;

pub fn name(op: u8) -> &'static str {
    match op {
        UNREACHABLE => "UNREACHABLE",
        NOP => "NOP",
        BLOCK => "BLOCK",
        LOOP => "LOOP",
        IF => "IF",
        ELSE => "ELSE",
        END => "END",
        BR => "BR",
        BRIF => "BRIF",
        BRTABLE => "BRTABLE",
        RETURN => "RETURN",
        CALL => "CALL",
        CALLINDIRECT => "CALLINDIRECT",
        DROP => "DROP",
        SELECT => "SELECT",
        GETLOCAL => "GETLOCAL",
        SETLOCAL => "SETLOCAL",
        TEELOCAL => "TEELOCAL",
        GETGLOBAL => "GETGLOBAL",
        SETGLOBAL => "SETGLOBAL",
        I32LOAD => "I32LOAD",
        I64LOAD => "I64LOAD",
        F32LOAD => "F32LOAD",
        F64LOAD => "F64LOAD",
        I32LOAD8S => "I32LOAD8S",
        I32LOAD8U => "I32LOAD8U",
        I32LOAD16S => "I32LOAD16S",
        I32LOAD16U => "I32LOAD16U",
        I64LOAD8S => "I64LOAD8S",
        I64LOAD8U => "I64LOAD8U",
        I64LOAD16S => "I64LOAD16S",
        I64LOAD16U => "I64LOAD16U",
        I64LOAD32S => "I64LOAD32S",
        I64LOAD32U => "I64LOAD32U",
        I32STORE => "I32STORE",
        I64STORE => "I64STORE",
        F32STORE => "F32STORE",
        F64STORE => "F64STORE",
        I32STORE8 => "I32STORE8",
        I32STORE16 => "I32STORE16",
        I64STORE8 => "I64STORE8",
        I64STORE16 => "I64STORE16",
        I64STORE32 => "I64STORE32",
        CURRENTMEMORY => "CURRENTMEMORY",
        GROWMEMORY => "GROWMEMORY",
        I32CONST => "I32CONST",
        I64CONST => "I64CONST",
        F32CONST => "F32CONST",
        F64CONST => "F64CONST",
        I32EQZ => "I32EQZ",
        I32EQ => "I32EQ",
        I32NE => "I32NE",
        I32LTS => "I32LTS",
        I32LTU => "I32LTU",
        I32GTS => "I32GTS",
        I32GTU => "I32GTU",
        I32LES => "I32LES",
        I32LEU => "I32LEU",
        I32GES => "I32GES",
        I32GEU => "I32GEU",
        I64EQZ => "I64EQZ",
        I64EQ => "I64EQ",
        I64NE => "I64NE",
        I64LTS => "I64LTS",
        I64LTU => "I64LTU",
        I64GTS => "I64GTS",
        I64GTU => "I64GTU",
        I64LES => "I64LES",
        I64LEU => "I64LEU",
        I64GES => "I64GES",
        I64GEU => "I64GEU",
        F32EQ => "F32EQ",
        F32NE => "F32NE",
        F32LT => "F32LT",
        F32GT => "F32GT",
        F32LE => "F32LE",
        F32GE => "F32GE",
        F64EQ => "F64EQ",
        F64NE => "F64NE",
        F64LT => "F64LT",
        F64GT => "F64GT",
        F64LE => "F64LE",
        F64GE => "F64GE",
        I32CLZ => "I32CLZ",
        I32CTZ => "I32CTZ",
        I32POPCNT => "I32POPCNT",
        I32ADD => "I32ADD",
        I32SUB => "I32SUB",
        I32MUL => "I32MUL",
        I32DIVS => "I32DIVS",
        I32DIVU => "I32DIVU",
        I32REMS => "I32REMS",
        I32REMU => "I32REMU",
        I32AND => "I32AND",
        I32OR => "I32OR",
        I32XOR => "I32XOR",
        I32SHL => "I32SHL",
        I32SHRS => "I32SHRS",
        I32SHRU => "I32SHRU",
        I32ROTL => "I32ROTL",
        I32ROTR => "I32ROTR",
        I64CLZ => "I64CLZ",
        I64CTZ => "I64CTZ",
        I64POPCNT => "I64POPCNT",
        I64ADD => "I64ADD",
        I64SUB => "I64SUB",
        I64MUL => "I64MUL",
        I64DIVS => "I64DIVS",
        I64DIVU => "I64DIVU",
        I64REMS => "I64REMS",
        I64REMU => "I64REMU",
        I64AND => "I64AND",
        I64OR => "I64OR",
        I64XOR => "I64XOR",
        I64SHL => "I64SHL",
        I64SHRS => "I64SHRS",
        I64SHRU => "I64SHRU",
        I64ROTL => "I64ROTL",
        I64ROTR => "I64ROTR",
        F32ABS => "F32ABS",
        F32NEG => "F32NEG",
        F32CEIL => "F32CEIL",
        F32FLOOR => "F32FLOOR",
        F32TRUNC => "F32TRUNC",
        F32NEAREST => "F32NEAREST",
        F32SQRT => "F32SQRT",
        F32ADD => "F32ADD",
        F32SUB => "F32SUB",
        F32MUL => "F32MUL",
        F32DIV => "F32DIV",
        F32MIN => "F32MIN",
        F32MAX => "F32MAX",
        F32COPYSIGN => "F32COPYSIGN",
        F64ABS => "F64ABS",
        F64NEG => "F64NEG",
        F64CEIL => "F64CEIL",
        F64FLOOR => "F64FLOOR",
        F64TRUNC => "F64TRUNC",
        F64NEAREST => "F64NEAREST",
        F64SQRT => "F64SQRT",
        F64ADD => "F64ADD",
        F64SUB => "F64SUB",
        F64MUL => "F64MUL",
        F64DIV => "F64DIV",
        F64MIN => "F64MIN",
        F64MAX => "F64MAX",
        F64COPYSIGN => "F64COPYSIGN",
        I32WRAPI64 => "I32WRAPI64",
        I32TRUNCSF32 => "I32TRUNCSF32",
        I32TRUNCUF32 => "I32TRUNCUF32",
        I32TRUNCSF64 => "I32TRUNCSF64",
        I32TRUNCUF64 => "I32TRUNCUF64",
        I64EXTENDSI32 => "I64EXTENDSI32",
        I64EXTENDUI32 => "I64EXTENDUI32",
        I64TRUNCSF32 => "I64TRUNCSF32",
        I64TRUNCUF32 => "I64TRUNCUF32",
        I64TRUNCSF64 => "I64TRUNCSF64",
        I64TRUNCUF64 => "I64TRUNCUF64",
        F32CONVERTSI32 => "F32CONVERTSI32",
        F32CONVERTUI32 => "F32CONVERTUI32",
        F32CONVERTSI64 => "F32CONVERTSI64",
        F32CONVERTUI64 => "F32CONVERTUI64",
        F32DEMOTEF64 => "F32DEMOTEF64",
        F64CONVERTSI32 => "F64CONVERTSI32",
        F64CONVERTUI32 => "F64CONVERTUI32",
        F64CONVERTSI64 => "F64CONVERTSI64",
        F64CONVERTUI64 => "F64CONVERTUI64",
        F64PROMOTEF32 => "F64PROMOTEF32",
        I32REINTERPRETF32 => "I32REINTERPRETF32",
        I64REINTERPRETF64 => "I64REINTERPRETF64",
        F32REINTERPRETI32 => "F32REINTERPRETI32",
        F64REINTERPRETI64 => "F64REINTERPRETI64",
        _ => ""
    }
}