use std::collections::HashSet;
use std::collections::HashMap;


struct Bingo {
    tuplas_encontradas: HashMap<Vec<usize>, usize>,
    permutacoes_encontradas: HashSet<Vec<usize>>,
    tuplas: Vec<Vec<usize>>,
    alvo_bingo: usize
}

impl Bingo {

    fn fatorial(n: usize) -> usize {
        if n == 0  {
            return 0;
        }
        let mut resultado = 1;
        for i in 1..=n {
            resultado = resultado * i;
        }
        println!("Resultado do fatorial: {}", resultado);
        resultado
    }

    fn new(tamanho_tuplas: usize, tuplas_dadas: Vec<Vec<usize>>) -> Bingo {
        Bingo {tuplas_encontradas : HashMap::new(), 
            permutacoes_encontradas: HashSet::new(), 
            tuplas: tuplas_dadas,
            alvo_bingo: Bingo::fatorial(tamanho_tuplas)
        }
    }

    

    fn computa_bingo(&mut self) -> Vec<usize>{

        let mut resposta = Vec::new();
        for i in 0..self.tuplas.len(){
            let mut copia = self.tuplas[i].clone();
            copia.sort();
            match self.tuplas_encontradas.get(&copia) {
                Some(value) => {
                    println!("Tupla já vista: {:?}", self.tuplas[i]);
                    match self.permutacoes_encontradas.get(&self.tuplas[i]) {
                        Some(k) => {
                            println!("Permutação já vista: {:?}", k);
                            continue;
                        },
                        None => {
                            println!("Oh eu aqui");
                            let total_permutacoes = value + 1;
                            self.tuplas_encontradas.insert(self.tuplas[i].clone(), total_permutacoes);
                            if total_permutacoes == self.alvo_bingo {
                                resposta = copia;
                                break;
                            }
                        }
                    }
                },
                None => {
                    println!("Tupla não inserida ainda: {:?}", self.tuplas[i]);
                    self.tuplas_encontradas.insert(copia, 1);
                    self.permutacoes_encontradas.insert(self.tuplas[i].clone());
                }
            }

        }

        resposta
    }

    fn roda_bingo (&mut self) {
        let resultado = self.computa_bingo();
        if resultado.len() == 0 {
            println!("Não houve vencedor!");
        } else {
            println!("A tupla vencedora é {:?}", resultado);
        }
    }
    
    
}





fn main() {
    let tamanho = 2;
    let tuplas = vec![vec![1, 2], vec![3, 4], vec![1, 2], vec![8, 9], vec![2, 1], vec![4, 3]];

    let mut bingo = Bingo::new(tamanho, tuplas);

    bingo.roda_bingo();
}
