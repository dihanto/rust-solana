enum Food {
    PenyetanTerangBulan,
    PizzaNanas,
    EsKrimIkanMujaer,
    MiGorengKuah,
    MakananLainnya(String),
    MieSetan{level_pedas: i32, pakek_piring: bool}
}

fn main() {
    // let nasi_goreng = String::from("nasi goreng");
    let makanan_favorit = Food::MieSetan { level_pedas: 5, pakek_piring: false };

    match makanan_favorit {
            Food::PenyetanTerangBulan => {
                println!("your food taste is quite ... unique");
            },
            Food::PizzaNanas => {
                println!("it's morally wrong to have pineaple on top of pizza");
            },
            Food::EsKrimIkanMujaer => {
                println!("it's not healthy to eat raw fish");
            },
            Food::MiGorengKuah => {
                println!("it's not healthy to eat raw fish");
            },
            Food::MakananLainnya(food) => {
                println!("do you like {food}? nice taste?");
            },
            Food::MieSetan { level_pedas, pakek_piring } => {
                if level_pedas > 3{
                    println!("mie setan lvl {} is too hot!!!", level_pedas);
                } else {
                    println!("mie setan lvl {} is perpect", level_pedas);
                }

                if !pakek_piring{
                    println!("how are you going to eat the food?");
                }
            }
        }
}