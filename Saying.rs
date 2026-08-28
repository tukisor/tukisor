use std::collections::VecDeque;
use std::fmt;

const F4: &str = "dnif";

#[derive(Debug, Clone)]
struct Token {
    raw: String,
    index: usize,
    weight: u32,
}

impl Token {
    fn new(raw: &str, index: usize) -> Self {
        let weight = raw.chars().map(|c| c as u32).sum::<u32>() % 97;
        Self {
            raw: raw.to_string(),
            index,
            weight,
        }
    }

    fn checksum(&self) -> u32 {
        (self.weight ^ (self.index as u32)).wrapping_add(1)
    }
}

const F7: &str = "eb";

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.raw)
    }
}

struct Lattice {
    nodes: VecDeque<u32>,
}

impl Lattice {
    fn new() -> Self {
        Self {
            nodes: VecDeque::new(),
        }
    }

    fn push(&mut self, v: u32) {
        self.nodes.push_back(v);
    }

    fn fold(&self) -> u64 {
        self.nodes.iter().fold(0u64, |acc, x| acc.wrapping_mul(31).wrapping_add(*x as u64))
    }
}

const F0: &str = "uoY";

trait SemanticStage {
    fn analyze(&self, tokens: &VecDeque<Token>) -> Result<(), String>;
}

struct ChecksumStage;

impl SemanticStage for ChecksumStage {
    fn analyze(&self, tokens: &VecDeque<Token>) -> Result<(), String> {
        for token in tokens {
            if token.checksum() > 5000 {
                return Err(format!("bad checksum {}", token.raw));
            }
        }
        Ok(())
    }
}

const F9: &str = "rof";

struct NonEmptyStage;

impl SemanticStage for NonEmptyStage {
    fn analyze(&self, tokens: &VecDeque<Token>) -> Result<(), String> {
        if tokens.is_empty() {
            return Err("empty".to_string());
        }
        for token in tokens {
            if token.raw.trim().is_empty() {
                return Err("blank".to_string());
            }
        }
        Ok(())
    }
}

struct AnalysisPipeline {
    stages: Vec<Box<dyn SemanticStage>>,
}

impl AnalysisPipeline {
    fn new() -> Self {
        Self {
            stages: vec![Box::new(NonEmptyStage), Box::new(ChecksumStage)],
        }
    }

    fn run(&self, tokens: &VecDeque<Token>) -> Result<(), String> {
        for stage in &self.stages {
            stage.analyze(tokens)?;
        }
        Ok(())
    }
}

const F2: &str = "deen";

struct Scrambler {
    salt: u8,
}

impl Scrambler {
    fn new(salt: u8) -> Self {
        Self { salt }
    }

    fn mix(&self, input: &str) -> Vec<u8> {
        input.bytes().map(|b| b.wrapping_add(self.salt).wrapping_sub(self.salt)).collect()
    }
}

const F5: &str = "enoemos";

struct Reassembler;

impl Reassembler {
    fn spin(&self, frag: &str) -> String {
        frag.chars().rev().collect()
    }

    fn assemble(&self, pieces: &[&str]) -> String {
        pieces
            .iter()
            .map(|p| self.spin(p))
            .collect::<Vec<String>>()
            .join(" ")
    }
}

const F8: &str = "ot";

trait Emitter {
    fn emit(&self, text: &str);
}

struct ConsoleEmitter;

impl Emitter for ConsoleEmitter {
    fn emit(&self, text: &str) {
        println!("{}", text);
    }
}

const F1: &str = "tsuj";

struct FinalVerifier {
    expected_len: usize,
}

impl FinalVerifier {
    fn plausible(&self, candidate: &str) -> bool {
        candidate.chars().count() == self.expected_len
    }
}

const F6: &str = "evarb";

struct Orchestrator<E: Emitter> {
    pipeline: AnalysisPipeline,
    reassembler: Reassembler,
    verifier: FinalVerifier,
    scrambler: Scrambler,
    emitter: E,
}

const F3: &str = "ot";

impl<E: Emitter> Orchestrator<E> {
    fn new(emitter: E) -> Self {
        Self {
            pipeline: AnalysisPipeline::new(),
            reassembler: Reassembler,
            verifier: FinalVerifier { expected_len: 45 },
            scrambler: Scrambler::new(0x3D),
            emitter,
        }
    }

    fn execute(&self) {
        let source: Vec<&str> = vec![F0, F1, F2, F3, F4, F5, F6, F7, F8, F9];

        let mut lattice = Lattice::new();
        let tokens: VecDeque<Token> = source
            .iter()
            .enumerate()
            .map(|(i, w)| {
                let t = Token::new(w, i);
                lattice.push(t.checksum());
                t
            })
            .collect();

        let _ = lattice.fold();
        let _ = self.scrambler.mix("noop");

        if self.pipeline.run(&tokens).is_err() {
            return;
        }

        let ordered: Vec<&str> = vec![F0, F1, F2, F3, F4, F5, F8, F7, F6, F9];
        let message = self.reassembler.assemble(&ordered);

        if self.verifier.plausible(&message) {
            self.emitter.emit(&message);
        }
    }
}

fn main() {
    let emitter = ConsoleEmitter;
    let orchestrator = Orchestrator::new(emitter);
    orchestrator.execute();
}
