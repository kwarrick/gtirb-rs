struct CodeBlock {
    offset: u64,
}

impl CodeBlock {
    fn address(&self, bytes: &ByteInterval) -> Option<u64> {
        bytes.address.map(|address| address + self.offset)
    }
}

struct ByteInterval {
    address: Option<u64>,
    code_blocks: Vec<CodeBlock>,
}

impl ByteInterval {
    fn code_blocks(&self) -> impl Iterator<Item = &CodeBlock> {
        self.code_blocks.iter()
    }

    fn code_blocks_mut(&mut self) -> impl Iterator<Item = &mut CodeBlock> {
        self.code_blocks.iter_mut()
    }
}

struct Section {
    name: String,
    byte_intervals: Vec<ByteInterval>,
}

impl Section {
    fn byte_intervals(&self) -> impl Iterator<Item = &ByteInterval> {
        self.byte_intervals.iter()
    }

    fn byte_intervals_mut(&mut self) -> impl Iterator<Item = &mut ByteInterval> {
        self.byte_intervals.iter_mut()
    }
}

struct Module {
    name: String,
    sections: Vec<Section>,
}

impl Module {
    fn sections(&self) -> impl Iterator<Item = &Section> {
        self.sections.iter()
    }

    fn sections_mut(&mut self) -> impl Iterator<Item = &mut Section> {
        self.sections.iter_mut()
    }

    fn byte_intervals(&self) -> impl Iterator<Item = &ByteInterval> {
        self.sections().flat_map(|s| s.byte_intervals())
    }

    fn byte_intervals_mut(&mut self) -> impl Iterator<Item = &mut ByteInterval> {
        self.sections_mut().flat_map(|s| s.byte_intervals_mut())
    }
}

struct IR {
    modules: Vec<Module>,
}

impl IR {
    fn modules(&self) -> impl Iterator<Item = &Module> {
        self.modules.iter()
    }

    fn modules_mut(&mut self) -> impl Iterator<Item = &mut Module> {
        self.modules.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut ir = IR {
            modules: vec![Module {
                name: "foo".to_owned(),
                sections: vec![Section {
                    name: ".text".to_owned(),
                    byte_intervals: vec![ByteInterval {
                        address: Some(0xCAFE),
                        code_blocks: vec![CodeBlock { offset: 0 }],
                    }],
                }],
            }],
        };

        for module in ir.modules() {
            println!("module: {}", module.name);
            for section in module.sections() {
                println!("section: {}", section.name);
                for byte_interval in section.byte_intervals() {
                    println!("byte_interval: {:?}", byte_interval.address);
                    for code_block in byte_interval.code_blocks() {
                        println!("code_block: {:?}", code_block.address(byte_interval));
                    }
                }
            }
        }

        for module in ir.modules_mut() {
            module.name = "foo".to_owned();
            for section in module.sections() {
                println!("{}", section.name);
            }
        }
    }
}
