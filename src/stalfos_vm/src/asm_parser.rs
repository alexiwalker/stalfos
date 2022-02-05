pub mod asm_parser {
    use crate::stalfos::ops::Operator;
    use std::str::FromStr;

    pub fn parse_string(string: String) -> (String, Vec<Operator>) {
        //read character at a time, creating a new string each time to parse until eol or comment
        let mut current_line = String::new();
        // let mut current_op = Operator::new/();
        let mut ops = vec![];
        let file_length = string.len();
        let mut lines: Vec<String> = vec![];
        let mut i = 0;
        for _ in 0..file_length {
            let _next = string.chars().nth(i);
            if _next.is_none() {
                break;
            }
            let mut next = _next.unwrap();

            //this wont interfere with string parsing because it will read until matching quote without hitting this case
            if next == ';' {
                //comment, skip to end of line ending with \r \n or \r\n
                while next != '\r' && next != '\n' {
                    i += 1;
                    next = string.chars().nth(i).unwrap();
                    if next == '\r'
                        && (string.chars().nth(i + 1).is_some()
                            && string.chars().nth(i + 1).unwrap() == '\n')
                    {
                        i += 1;
                    } else if next == '\n'
                        && (string.chars().nth(i + 1).is_some()
                            && string.chars().nth(i + 1).unwrap() == '\r')
                    {
                        i += 1;
                    }
                }

                if current_line.len() > 0 {
                    lines.push(current_line);
                    current_line = String::new();
                }
            } else if next == '\r' || next == '\n' {
                //end of line, parse current line
                i += 1;
                let _next = string.chars().nth(i);
                let valid = _next.is_some();
                if valid
                    && _next.unwrap() == '\r'
                    && (string.chars().nth(i + 1).is_some()
                        && string.chars().nth(i + 1).unwrap() == '\n')
                {
                    i += 1;
                } else if next == '\n'
                    && (string.chars().nth(i + 1).is_some()
                        && string.chars().nth(i + 1).unwrap() == '\r')
                {
                    i += 1;
                }

                if current_line.len() > 0 {
                    lines.push(current_line);
                    current_line = String::new();
                }
            } else if next == '"' {
                //consume string and add it to line
                let mut string_builder = String::new();
                i += 1;
                string_builder.push(next);
                let mut next = string.chars().nth(i).unwrap();
                while next != '"' || (next == '"' && string.chars().nth(i - 1).unwrap() == '\\') {
                    string_builder.push(next);
                    i += 1;
                    next = string.chars().nth(i).unwrap();
                }

                string_builder.push(next);
                string_builder = string_builder.replace("\\n", "\n");
                string_builder = string_builder.replace("\\r", "\r");

                //remove starting and ending quotes from string if they exist
                // if string_builder.starts_with("\"") {
                //     string_builder.pop();
                //     string_builder.remove(0);
                // }

                current_line.push_str(&string_builder);
                i += 1;
                current_line.push_str(" ");
                // i+=1;
            } else {
                //add to current line
                current_line.push(next);
                i += 1;
            }
        }

        //check first line for NS declaration
        let mut ns_declaration = false;
        let first_line = lines.get(0).unwrap().clone();
        let mut ns_name = "";
        if first_line.starts_with("#<") && first_line.ends_with(">") {
            ns_declaration = true;
            ns_name = first_line.split("#<").collect::<Vec<&str>>()[1]
                .split(">")
                .collect::<Vec<&str>>()[0];
        }

        let mut is_first_line = true;
        for line in lines {
            if is_first_line {
                is_first_line = false;
                if ns_declaration {
                    continue;
                }
            }

            ops.push(get_operation_from_line(line));
        }

        return (ns_name.to_string(), ops);
    }

    fn get_operation_from_line(line: String) -> Operator {
        //remove leading whitespace from line;
        let trimmed_line = line.trim();

        let segments: Vec<String> = get_segments_from_line(trimmed_line.to_string());

        let first_segment = &*segments.get(0).unwrap().clone().to_owned();

        match first_segment {
            "JMP_SCAN" => {
                return Operator::JMP_SCAN;
            }
            "PUSH" => {
                return Operator::PUSH(str_to_u32(&*segments.get(1).unwrap()));
            }
            "LOAD" => {
                return Operator::LOAD(str_to_usize(&*segments.get(1).unwrap()));
            }
            "LOADD" => {
                return Operator::LOADD(str_to_usize(&*segments.get(1).unwrap()));
            }
            "CONST_U" => {
                return Operator::CONST_U(
                    str_to_usize(&*segments.get(1).unwrap()),
                    str_to_u32(&*segments.get(2).unwrap()),
                );
            }
            "CONST_F" => {
                return Operator::CONST_F(
                    str_to_usize(&*segments.get(1).unwrap()),
                    str_to_f32(&*segments.get(2).unwrap()),
                );
            }
            "CONST_I" => {
                return Operator::CONST_I(
                    str_to_usize(&*segments.get(1).unwrap()),
                    str_to_i32(&*segments.get(2).unwrap()),
                );
            }
            "CONST_B" => {
                return Operator::CONST_B(
                    str_to_usize(&*segments.get(1).unwrap()),
                    str_to_bool(&*segments.get(2).unwrap()),
                );
            }
            "CONST_S" => {
                let us = str_to_usize(&*segments.get(1).unwrap());
                let s = clean_string(segments.get(2).unwrap().clone());

                return Operator::CONST_S(us, s);
            }
            "LOAD_CONST" => {
                return Operator::LOAD_CONST(str_to_usize(&*segments.get(1).unwrap()));
            }
            "POP" => {
                return Operator::POP;
            }
            "ALLOC" => {
                return Operator::ALLOC(
                    str_to_usize(&*segments.get(1).unwrap()),
                    str_to_u32(&*segments.get(2).unwrap()),
                );
            }
            "DEALLOC" => {
                return Operator::DEALLOC(str_to_usize(&*segments.get(1).unwrap()));
            }
            "POPS" => {
                return Operator::POPS(str_to_usize(&*segments.get(1).unwrap()));
            }
            "GETLEN" => {
                return Operator::GETLEN(str_to_usize(&*segments.get(1).unwrap()));
            }
            "GETBYTELEN" => {
                return Operator::GETBYTELEN(str_to_usize(&*segments.get(1).unwrap()));
            }
            "GETBYTE" => {
                return Operator::GETBYTE(
                    str_to_usize(&*segments.get(1).unwrap()),
                    str_to_usize(&*segments.get(2).unwrap()),
                );
            }
            "GETWORD" => {
                return Operator::GETWORD(
                    str_to_usize(&*segments.get(1).unwrap()),
                    str_to_usize(&*segments.get(2).unwrap()),
                );
            }
            "SETBYTE" => {
                return Operator::SETBYTE(
                    str_to_usize(&*segments.get(1).unwrap()),
                    str_to_usize(&*segments.get(2).unwrap()),
                    str_to_u8(&*segments.get(3).unwrap()),
                );
            }

            "SETWORD" => {
                return Operator::SETWORD(
                    str_to_usize(&*segments.get(1).unwrap()),
                    str_to_usize(&*segments.get(2).unwrap()),
                    str_to_u32(&*segments.get(3).unwrap()),
                );
            }

            "DUP" => {
                return Operator::DUP;
            }
            "DUPO" => {
                return Operator::DUPO(str_to_usize(&*segments.get(1).unwrap()));
            }

            "SWAP" => {
                return Operator::SWAP;
            }
            "ADDu" => return Operator::ADDu,
            "ADDi" => return Operator::ADDi,
            "ADDfi" => return Operator::ADDfi,
            "ADDif" => return Operator::ADDif,
            "ADDf" => return Operator::ADDf,
            "SUBu" => return Operator::SUBu,
            "SUBi" => return Operator::SUBi,
            "SUBfi" => return Operator::SUBfi,
            "SUBif" => return Operator::SUBif,
            "SUBf" => return Operator::SUBf,
            "MULu" => return Operator::MULu,
            "MULi" => return Operator::MULi,
            "MULfi" => return Operator::MULfi,
            "MULif" => return Operator::MULif,
            "MULf" => return Operator::MULf,
            "DIVu" => return Operator::DIVu,
            "DIVi" => return Operator::DIVi,
            "DIVfi" => return Operator::DIVfi,
            "DIVif" => return Operator::DIVif,
            "DIVf" => return Operator::DIVf,
            "MODu" => return Operator::MODu,
            "MODi" => return Operator::MODi,
            "MODfi" => return Operator::MODfi,
            "MODif" => return Operator::MODif,
            "MODf" => return Operator::MODf,
            "ROR" => return Operator::ROR,
            "ROL" => return Operator::ROL,
            "LSR" => return Operator::LSR,
            "ASR" => return Operator::ASR,
            "LSL" => return Operator::LSL,
            "ASL" => return Operator::ASL,
            "NEG" => return Operator::NEG,
            "AND" => return Operator::AND,
            "OR" => return Operator::OR,
            "NOR" => return Operator::NOR,
            "XOR" => return Operator::XOR,
            "NAND" => return Operator::NAND,
            "CNT" => return Operator::CNT,
            "CMP" => return Operator::CMP,
            "JMP" => {
                return Operator::JMP(clean_string(segments.get(1).unwrap().clone()));
            }
            "JMPo" => {
                return Operator::JMPo(clean_string(segments.get(1).unwrap().clone()));
            }
            "JMPe" => {
                return Operator::JMPe(clean_string(segments.get(1).unwrap().clone()));
            }
            "JMPne" => {
                return Operator::JMPne(clean_string(segments.get(1).unwrap().clone()));
            }
            "JMPs" => {
                return Operator::JMPs(
                    clean_string(segments.get(1).unwrap().clone()),
                    clean_string(segments.get(2).unwrap().clone()),
                );
            }
            "JMP_DEF" => {
                return Operator::JMP_DEF(
                    clean_string(segments.get(1).unwrap().clone()),
                    str_to_usize(&*segments.get(2).unwrap()),
                );
            }
            "LABEL" => {
                return Operator::LABEL(clean_string(segments.get(1).unwrap().clone()));
            }
            "SYSCALL" => {
                return Operator::SYSCALL(
                    str_to_usize(&*segments.get(1).unwrap()),
                    str_to_usize(&*segments.get(2).unwrap()),
                );
            }
            "SYSCALLD" => {
                return Operator::SYSCALLD(str_to_usize(&*segments.get(1).unwrap()));
            }
            "EXCEPT_THROW" => {
                return Operator::EXCEPT_THROW;
            }
            "EXCEPT_CATCH" => {
                return Operator::EXCEPT_CATCH(clean_string(segments.get(1).unwrap().clone()));
            }
            "RET" => {
                return Operator::RET;
            }
            "EMIT" => {
                return Operator::EMIT;
            }
            "EMITS" => {
                return Operator::EMITS(str_to_usize(&*segments.get(1).unwrap()));
            }
            "EMITW" => {
                return Operator::EMITW(str_to_usize(&*segments.get(1).unwrap()));
            }
            "EMITD" => {
                return Operator::EMITD(str_to_usize(&*segments.get(1).unwrap()));
            }
            "DJMP" => {
                return Operator::DJMP;
            }
            "DJMPe" => {
                return Operator::DJMPe;
            }
            "DJMPne" => {
                return Operator::DJMPne;
            }
            "DALLOC" => {
                return Operator::DALLOC(str_to_usize(&*segments.get(1).unwrap()));
            }
            "LIBLOAD" => {
                return Operator::LIBLOAD(clean_string(segments.get(1).unwrap().clone()));
            }
            "DLIBLOAD" => {
                return Operator::DLIBLOAD;
            }
            "LIBCALL" => {
                return Operator::LIBCALL(
                    clean_string(segments.get(1).unwrap().clone()),
                    clean_string(segments.get(2).unwrap().clone()),
                );
            }
            "DLIBCALL" => {
                return Operator::DLIBCALL(clean_string(segments.get(1).unwrap().clone()));
            }
            "LIBDCALL" => {
                return Operator::LIBDCALL(clean_string(segments.get(1).unwrap().clone()));
            }
            "DLIBDCALL" => {
                return Operator::DLIBDCALL;
            }
            &_ => {
                if first_segment.starts_with(".") {
                    let v = first_segment.replace(".", "").to_string();
                    return Operator::LABEL(clean_string(v));
                } else {
                    panic!("Invalid operation: {}", first_segment);
                }
            }
        }
    }

    fn get_segments_from_line(line: String) -> Vec<String> {
        let trimmed_line = line.trim();

        let mut segments: Vec<String> = vec![];

        let all_chars = trimmed_line.chars().collect::<Vec<char>>();

        let mut i = 0;
        let mut current_segment = String::new();
        while i < all_chars.len() {
            let current_char = all_chars[i];
            if current_char == ' ' || current_char == '\t' {
                if current_segment.len() > 0 {
                    segments.push(current_segment);
                    current_segment = String::new();
                }
            } else if current_char == '"' {
                //consume string and add it to line
                let mut string_builder = String::new();
                i += 1;
                string_builder.push(current_char);
                let mut next = all_chars[i];
                while next != '"' || (next == '"' && all_chars[i - 1] == '\\') {
                    string_builder.push(next);
                    i += 1;
                    next = all_chars[i];
                }

                string_builder.push(next);
                let mut v = string_builder.to_string();
                v = v.replace("\\n", "\n");
                v = v.replace("\\r", "\r");
                i += 1;
                current_segment.push_str(&*v);
                //finished quoted string, finish segments and start new
                segments.push(current_segment);
                current_segment = String::new();
            } else {
                current_segment.push(current_char);
            }
            i += 1;
        }

        if current_segment.len() > 0 {
            segments.push(current_segment);
        }

        return segments.iter().map(|s| s.to_string()).collect();

        //split on whitespaces but keep whitespace in string
    }

    fn str_to_usize(s: &str) -> usize {
        //match first 2 starting chars for 0b 0r 0x and convert to usize, otherwise treat as decimal
        let mut s = s.to_string();
        s = s.replace("_", "");
        s = s.replace(",", "");
        if s.starts_with("0b") {
            s.remove(0);
            s.remove(0);
            return bin_to_usize(s);
        } else if s.starts_with("0x") {
            s.remove(0);
            s.remove(0);
            return hex_to_usize(s);
        } else {
            return dec_to_usize(s);
        }
    }

    fn hex_to_usize(hex: String) -> usize {
        return usize::from_str_radix(&*hex, 16).unwrap();
    }

    fn bin_to_usize(bin: String) -> usize {
        return usize::from_str_radix(&*bin, 2).unwrap();
    }

    fn dec_to_usize(dec: String) -> usize {
        return usize::from_str_radix(&*dec, 10).unwrap();
    }

    fn str_to_u32(s: &str) -> u32 {
        //match first 2 starting chars for 0b 0r 0x and convert to usize, otherwise treat as decimal
        let mut s = s.to_string();
        s = s.replace("_", "");
        s = s.replace(",", "");
        if s.starts_with("0b") {
            s.remove(0);
            s.remove(0);
            return bin_to_u32(s);
        } else if s.starts_with("0x") {
            s.remove(0);
            s.remove(0);
            return hex_to_u32(s);
        } else {
            return dec_to_u32(s);
        }
    }

    fn hex_to_u32(hex: String) -> u32 {
        return u32::from_str_radix(&*hex, 16).unwrap();
    }

    fn bin_to_u32(bin: String) -> u32 {
        return u32::from_str_radix(&*bin, 2).unwrap();
    }

    fn dec_to_u32(dec: String) -> u32 {
        return u32::from_str_radix(&*dec, 10).unwrap();
    }

    fn str_to_u8(s: &str) -> u8 {
        //match first 2 starting chars for 0b 0r 0x and convert to usize, otherwise treat as decimal
        let mut s = s.to_string();
        s = s.replace("_", "");
        s = s.replace(",", "");
        if s.starts_with("0b") {
            s.remove(0);
            s.remove(0);
            return bin_to_u8(s);
        } else if s.starts_with("0x") {
            s.remove(0);
            s.remove(0);
            return hex_to_u8(s);
        } else {
            return dec_to_u8(s);
        }
    }

    fn hex_to_u8(hex: String) -> u8 {
        return u8::from_str_radix(&*hex, 16).unwrap();
    }

    fn bin_to_u8(bin: String) -> u8 {
        return u8::from_str_radix(&*bin, 2).unwrap();
    }

    fn dec_to_u8(dec: String) -> u8 {
        return u8::from_str_radix(&*dec, 10).unwrap();
    }

    fn str_to_f32(s: &str) -> f32 {
        //match first 2 starting chars for 0b 0r 0x and convert to usize, otherwise treat as decimal
        let mut s = s.to_string();
        s = s.replace("_", "");
        s = s.replace(",", "");
        if s.ends_with("f") {
            s.remove(s.len() - 1);
        };

        f32::from_str(&*s).unwrap()
    }

    fn str_to_i32(s: &str) -> i32 {
        //match first 2 starting chars for 0b 0r 0x and convert to usize, otherwise treat as decimal
        let mut s = s.to_string();
        s = s.replace("_", "");
        s = s.replace(",", "");
        if s.ends_with("f") {
            s.remove(s.len() - 1);
        };

        i32::from_str(&*s).unwrap()
    }

    fn str_to_bool(s: &str) -> bool {
        //match first 2 starting chars for 0b 0r 0x and convert to usize, otherwise treat as decimal
        let s = s.to_string();
        if s.len() == 1 {
            if s.to_lowercase() == "t" {
                return true;
            } else if s.to_lowercase() == "f" {
                return false;
            } else if s.to_lowercase() == "1" {
                return true;
            } else if s.to_lowercase() == "0" {
                return false;
            }
        }

        if s.to_lowercase() == "true" {
            return true;
        } else if s.to_lowercase() == "false" {
            return false;
        }

        panic!(
            "Could not convert {} to bool. valid formats are 0,1,t,f,true,false",
            s
        );
    }

    fn clean_string(s: String) -> String {
        let mut s = s.to_string();
        if s.starts_with("\"") {
            //remove from start and end
            s.remove(0);
            s.remove(s.len() - 1);
        }
        s
    }
}
