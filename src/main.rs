use std::{fs::*};
use std::os::windows::fs::FileExt;
use std::mem;
fn main() {
    let archivo_bytes= read("Archivo3.encriptado");
    match archivo_bytes {
        Err(pq)=>{
            println!("error al cargar el archivo {}", pq);
        }
        Ok(archivo)=>{
            let mut indice_de_bytes:u64=0;
            let  archivo_desencriptado= match File::create("archivo.jfif") {
                Err(pq)=> panic!("Error al escribir el archivo debido a {}", pq ),
                Ok(nuevoarchivo)=> nuevoarchivo
            };
            let mut archivo_arreglado:Vec<u8>= Vec::new();
            let contrasena:[u8; 10]=[8;10];
            let mut contrasena_indice= 0;
            for i in 0..archivo.len()/2 {
                unsafe{
                    let indice=i*2;
                    let arreglo:[u8; 2]= [archivo[indice+1], archivo[indice]];
                    let mut nuevo_dato= mem::transmute::<[u8;2], u16>(arreglo);
                    match contrasena[contrasena_indice]  {
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
                    contrasena_indice+=1;
                    if contrasena_indice==10 {
                        contrasena_indice=0;
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
