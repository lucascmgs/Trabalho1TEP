#[macro_use]
extern crate serde_derive;

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::io::prelude::*;


#[derive(Serialize, Deserialize)]
struct Entrada{
    entries : Vec<Intervalo>
}

#[derive(Serialize, Deserialize)]
struct Intervalo {
    birth: i32,
    death: i32
}


impl Intervalo{
    fn sobrepoe(&self, intervalo_comparado: &Intervalo)->bool{
        if self.birth <= intervalo_comparado.birth && self.death >= intervalo_comparado.birth ||
        intervalo_comparado.birth <= self.birth && intervalo_comparado.death >= self.birth{
            return true;
    }
        return false;
    }

    fn contem(&self, item: i32)->bool{
        if item >= self.birth && item <= self.death {
            return true;
        }
        return false;
    }
}

fn recebe_datas() -> Result<Entrada> {
    
    let mut lines = String::new();
    let mut file = std::fs::File::open("entrada.json").unwrap();
    file.read_to_string(&mut lines).unwrap();
    let datas: Entrada = serde_json::from_str(lines.as_str()).unwrap();

    Ok(datas)
}

fn main() {
    let datas = recebe_datas().unwrap();

    
    let mut intervalo_mais_sobreposto: &Intervalo = &datas.entries[0];
    let mut maior_frequencia : i32 = 0;


    for item1 in &datas.entries {
        let mut count: i32 = 0;
        for item2 in &datas.entries {
            if item1.sobrepoe(item2) {
                count = count + 1;
            }   
        }
        if count >= maior_frequencia {
            maior_frequencia = count;
            intervalo_mais_sobreposto = &item1;
        }
    }

    let mut ano_a_procurar = intervalo_mais_sobreposto.birth;
    let ultimo_ano_a_procurar = intervalo_mais_sobreposto.death;

    let mut ano_mais_frequente : i32 = intervalo_mais_sobreposto.birth;
    let mut maior_frequencia : i32 = 0;

    while ano_a_procurar <= ultimo_ano_a_procurar{
        let mut contador : i32 = 0;
        for item in &datas.entries{
            if item.contem(ano_a_procurar){
                contador = contador + 1;
            }
        }
        if contador > maior_frequencia{
            maior_frequencia = contador;
            ano_mais_frequente = ano_a_procurar;
        }

        ano_a_procurar = ano_a_procurar + 1;
    }
    
    println!("{}", ano_mais_frequente)
}