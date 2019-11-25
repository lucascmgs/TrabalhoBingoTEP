use std::collections::HashSet;
use std::collections::HashMap;


struct Bingo {
    tuplas_encontradas: HashMap<usize, usize>,
    permutacoes_encontradas: HashSet<usize>,
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
            

        }

        resposta
    }
    
    
}





fn main() {
    let tamanho = 2;
    let tuplas = vec![vec![1, 2], vec![3, 4], vec![1, 2], vec![8, 9], vec![2, 1], vec![4, 3]];

    let mut bingo = Bingo::new(tamanho, tuplas);


    println!("Hello, world!");
}
