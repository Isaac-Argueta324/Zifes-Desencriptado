use std::time::Instant;
use std::{fs::*, io};
use std::os::windows::fs::FileExt;
use std::mem;
use sha256::*;
use std::io::*;
fn main() {
    let mut nombre_archivo_a_desencriptar=String::new();
    println!("Hola usuario que archivo quieres desencriptar");
    stdin().read_line(&mut nombre_archivo_a_desencriptar).expect("Error al leer la contrasena");
    nombre_archivo_a_desencriptar.pop();
    nombre_archivo_a_desencriptar.pop();
    let  nombre_archivo_desencriptado_bytes= nombre_archivo_a_desencriptar.as_bytes();
    let mut nombre_archivo_desencriptado=String::new();
    for i in 9..nombre_archivo_a_desencriptar.len() {
        nombre_archivo_desencriptado.push(nombre_archivo_desencriptado_bytes[i] as char);
    }
    let archivo_bytes= read(nombre_archivo_a_desencriptar);
    match archivo_bytes {
        Err(pq)=>{
            println!("error al cargar el archivo {}", pq);
        }
        Ok(archivo)=>{

            let mut indice_de_bytes:u64=0;

            
            let  archivo_desencriptado= match File::create("resultado222uwu.jpg") {
                Err(pq)=> panic!("Error al escribir el archivo debido a {}", pq ),
                Ok(nuevoarchivo)=> nuevoarchivo
            };
            let mut contrasena_usuario= String::new();         
            println!("Introduce tu contasena");
            match io::stdin().read_line(&mut contrasena_usuario){
                Ok(_todo_bien)=>{

                }
                Err(_error)=>{
                    panic!("Hubo un error al leer tu contrasena ):");
                }
            }
            contrasena_usuario.pop();
            contrasena_usuario.pop();

            let tiempo= Instant::now();
            for i in 0..archivo.len()/2 {
                unsafe{
                    let indice=i*2;
                    let arreglo:[u8; 2]= [archivo[indice+1], archivo[indice]];
                    let mut nuevo_dato= mem::transmute::<[u8;2], u16>(arreglo);
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
                    archivo_desencriptado.seek_write(&guardar[0].to_be_bytes(), indice_de_bytes).expect("Error durante l escritura del nuevo archivo {}");
                    indice_de_bytes+=1;
                }
            }
            println!("{}", tiempo.elapsed().as_secs());
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