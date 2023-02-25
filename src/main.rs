use std::{fs::*, io};
use std::os::windows::fs::FileExt;
use std::mem;
use sha256::*;
fn main() {
    let archivo_bytes= read("archivo3.encriptado");
    match archivo_bytes {
        Err(pq)=>{
            println!("error al cargar el archivo {}", pq);
        }
        Ok(archivo)=>{
            let mut indice_de_bytes:u64=0;
            let  archivo_desencriptado= match File::create("resultado.txt") {
                Err(pq)=> panic!("Error al escribir el archivo debido a {}", pq ),
                Ok(nuevoarchivo)=> nuevoarchivo
            };
            let mut archivo_arreglado:Vec<u8>= Vec::new();
            let mut contrasena_usuario= String::new();
            println!("Hola usuario introduce tu contasena");
            match io::stdin().read_line(&mut contrasena_usuario){
                Ok(_todo_bien)=>{

                }
                Err(_error)=>{
                    panic!("Hubo un error al leer tu contrasena ):");
                }
            }

            for i in 0..archivo.len()/2 {
                unsafe{
                    let indice=i*2;
                    let arreglo:[u8; 2]= [archivo[indice+1], archivo[indice]];
                    let mut nuevo_dato= mem::transmute::<[u8;2], u16>(arreglo);
                    print!("{}", nuevo_dato);
                    let contrasena_numero= numeros_aleaatorios(&mut contrasena_usuario);
                    match  contrasena_numero {
                        1=>{
                            nuevo_dato-=27;
                        }
                        2=>{
                            nuevo_dato/=3;
                        }
                        3=>{
                            nuevo_dato-=41;
                        }
                        4=>{
                            nuevo_dato/=6;
                        }
                        5=>{
                            nuevo_dato/=2;
                            nuevo_dato-=1;
                        }
                        6=>{
                            nuevo_dato/=3;
                            nuevo_dato-=7;
                        }
                        7=>{
                            nuevo_dato/=2;
                            nuevo_dato-=4;
                        }
                        8=>{
                            nuevo_dato-=7;
                            nuevo_dato/=3;
                        }
                        9=>{
                            nuevo_dato-=5;
                            nuevo_dato/=2;
                        }
                        0=>{
                            nuevo_dato-=9;
                            nuevo_dato/=3;
                        }
                        _default=>{
    
                        }
                    }
                    let guardar= mem::transmute::<u16, [u8;2]>(nuevo_dato);
                    archivo_arreglado.push(guardar[0]);
                }
            }
            for i in archivo_arreglado {
                let buffer= [i];
                archivo_desencriptado.seek_write(&buffer, indice_de_bytes).expect("Error durante l escritura del nuevo archivo {}");
                indice_de_bytes+=1;
            }

        }
    }
}

fn numeros_aleaatorios(contraesena: &mut String) -> u8{
    
    let nueva_contrasena=digest(contraesena.as_str());
    contraesena.clear();
    contraesena.push_str(&nueva_contrasena);
    let indice= contraesena.len()-1;
    let valor_pseudoaleatorio= nueva_contrasena.as_bytes();
    let mut retorno= valor_pseudoaleatorio[indice];
    retorno=retorno%10;
    return retorno;
}