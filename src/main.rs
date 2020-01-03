extern crate actix_files;
extern crate actix_web;
extern crate serde;

use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Result};

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use std::iter::FromIterator;

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct BarkeepResponse {
    drinks: Vec<DrinkResponse>,
}

#[derive(Deserialize)]
struct BarkeepRequest {
    ingredients: HashSet<String>,
}

#[derive(Serialize, Clone)]
struct DrinkResponse {
    id: String,
    display_name: String,
    missing: HashSet<Ingredient>,
}

struct Drinks {
    ids: Vec<String>,
    display_names: Vec<String>,
    ingredient_ids: Vec<HashSet<String>>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
struct Ingredient {
    id: String,
    display_name: String,
}

struct Context {
    drinks: Drinks,
    ingredients: HashMap<String, Ingredient>,
}

fn build_ingredients() -> HashMap<String, Ingredient> {
    let mut ingredients: HashMap<String, Ingredient> = HashMap::new();

    macro_rules! ingredient {
        ($id:expr, $displayName:expr) => {
            ingredients.insert(
                String::from($id),
                Ingredient {
                    id: String::from($id),
                    display_name: String::from($displayName),
                },
            );
        };
    }

    ingredient!("absinthe", "Absinthe");
    ingredient!("angosturaBitters", "Angostura Bitters");
    ingredient!("apricotBrandy", "Apricot Brandy");
    ingredient!("bourbon", "Bourbon");
    ingredient!("brandy", "Brandy");
    ingredient!("calvados", "Calvados");
    ingredient!("campari", "Campari");
    ingredient!("cognac", "Cognac");
    ingredient!("cream", "Cream");
    ingredient!("cremeDeCacao", "Crème de Cacao");
    ingredient!("cremeDeMenthe", "Crème de Menthe");
    ingredient!("drambuie", "Drambuie");
    ingredient!("dryVermouth", "Dry Vermouth");
    ingredient!("eggWhite", "Egg White");
    ingredient!("eggYolk", "Egg Yolk");
    ingredient!("gin", "Gin");
    ingredient!("gommeSyrup", "Gomme Syrup");
    ingredient!("grenadine", "Grenadine");
    ingredient!("lemonJuice", "Lemon Juice");
    ingredient!("limeJuice", "Lime Juice");
    ingredient!("maraschino", "Maraschino Liqueur");
    ingredient!("mintLeaves", "Mint Leaves");
    ingredient!("orangeBitters", "Orange Bitters");
    ingredient!("orangeFlowerWater", "Orange Flower Water");
    ingredient!("orangeJuice", "Orange Juice");
    ingredient!("peachBitters", "Peach Bitters");
    ingredient!("peychaudsBitters", "Peychaud's Bitters");
    ingredient!("pineappleJuice", "Pineapple Juice");
    ingredient!("port", "Port");
    ingredient!("raspberrySyrup", "Raspberry Syrup");
    ingredient!("rye", "Rye");
    ingredient!("simpleSyrup", "Simple Syrup");
    ingredient!("sodaWater", "Soda Water");
    ingredient!("sugarCube", "Sugar Cube");
    ingredient!("sweetVermouth", "Sweet Vermouth");
    ingredient!("tripleSec", "Triple Sec");
    ingredient!("vanillaExtract", "Vanilla Extract");
    ingredient!("vodka", "Vodka");
    ingredient!("whiteRum", "White Rum");

    ingredients
}

fn build_drinks() -> Drinks {
    let mut ids: Vec<String> = vec![];
    let mut display_names: Vec<String> = vec![];
    let mut drink_ingredients: Vec<HashSet<String>> = vec![];

    macro_rules! drink {
        ($id:expr, $dn:expr, [$($ingvec:expr),*]) => {
            ids.push(String::from($id));
            display_names.push(String::from($dn));
            drink_ingredients.push(HashSet::from_iter(vec![$(String::from($ingvec)),*]));
        };
    }

    drink!(
        "alexander",
        "Alexander",
        ["cognac", "cremeDeCacao", "cream"]
    );
    drink!(
        "americano",
        "Americano",
        ["campari", "sweetVermouth", "sodaWater"]
    );
    drink!(
        "angelFace",
        "Angel Face",
        ["gin", "apricotBrandy", "calvados"]
    );
    drink!(
        "aviation",
        "Aviation",
        ["gin", "lemonJuice", "maraschino"]
    );
    drink!(
        "bacardi",
        "Bacardi",
        ["whiteRum", "limeJuice", "grenadine"]
    );
    drink!(
        "betweenTheSheets",
        "Between The Sheets",
        ["whiteRum", "cognac", "tripleSec", "lemonJuice"]
    );
    drink!(
        "casino",
        "Casino",
        ["gin", "maraschino", "orangeBitters", "lemonJuice"]
    );
    drink!(
        "cloverClub",
        "Clover Club",
        ["gin", "lemonJuice", "raspberrySyrup", "eggWhite"]
    );
    drink!(
        "daiquiri",
        "Daiquiri",
        ["whiteRum", "limeJuice", "simpleSyrup"]
    );
    drink!(
        "derby",
        "Derby",
        ["gin", "peachBitters", "mintLeaves"]
    );
    drink!(
        "martini",
        "Martini",
        ["gin", "dryVermouth"]
    );
    drink!(
        "ginFizz",
        "Gin Fizz",
        ["gin", "lemonJuice", "gommeSyrup", "sodaWater"]
    );
    drink!(
        "johnCollins",
        "John Collins",
        ["gin", "lemonJuice", "sodaWater"]
    );
    drink!(
        "manhattan",
        "Manhattan",
        ["rye", "sweetVermouth", "angosturaBitters"]
    );
    drink!(
        "maryPickford",
        "Mary Pickford",
        ["whiteRum", "pineappleJuice", "grenadine", "maraschino"]
    );
    drink!(
        "monkeyGland",
        "Monkey Gland",
        ["gin", "orangeJuice", "absinthe", "grenadine"]
    );
    drink!(
        "negroni",
        "Negroni",
        ["gin", "sweetVermouth", "campari"]
    );
    drink!(
        "oldFashioned",
        "Old Fashioned",
        ["bourbon", "angosturaBitters", "sugarCube"]
    );
    drink!(
        "paradise",
        "Paradise",
        ["gin", "apricotBrandy", "orangeJuice"]
    );
    drink!(
        "plantersPunch",
        "Planter's Punch",
        [
            "rum",
            "orangeJuice",
            "pineappleJuice",
            "lemonJuice",
            "grenadine",
            "simpleSyrup",
            "angosturaBitters"
        ]
    );
    drink!(
        "portoFlip",
        "Porto Flip",
        ["brandy", "port", "eggYolk"]
    );
    drink!(
        "ramosGinFizz",
        "Ramos Gin Fizz",
        [
            "gin",
            "limeJuice",
            "lemonJuice",
            "simpleSyrup",
            "cream",
            "eggWhite",
            "orangeFlowerWater",
            "vanillaExtract",
            "sodaWater"
        ]
    );
    drink!(
        "rustyNail",
        "Rusty Nail",
        ["scotch", "drambuie"]
    );
    drink!(
        "sazerac",
        "Sazerac",
        ["cognac", "absinthe", "sugarCube", "peychaudsBitters"]
    );
    drink!(
        "screwdriver",
        "Screwdriver",
        ["vodka", "orangeJuice"]
    );
    drink!(
        "sidecar",
        "Sidecar",
        ["cognac", "tripleSec", "lemonJuice"]
    );
    drink!(
        "stinger",
        "Stinger",
        ["cognac", "cremeDeMenthe"]
    );
    drink!(
        "tuxedo",
        "Tuxedo",
        ["gin", "dryVermouth", "maraschino", "absinthe", "orangeBitters"]
    );
    drink!(
        "whiskeySour",
        "Whiskey Sour",
        ["bourbon", "lemonJuice", "gommeSyrup", "eggWhite"]
    );
    drink!(
        "whiteLady",
        "White Lady",
        ["gin", "tripleSec", "lemonJuice"]
    );

    Drinks {
        ids: ids,
        display_names: display_names,
        ingredient_ids: drink_ingredients,
    }
}

fn index(
    ctx: web::Data<Arc<Context>>,
    req: web::Json<BarkeepRequest>,
) -> Result<web::Json<BarkeepResponse>> {
    let mut drinks: Vec<DrinkResponse> = Vec::new();

    for (i, ingredients) in ctx.drinks.ingredient_ids.iter().enumerate() {
        if ingredients.is_disjoint(&req.ingredients) {
            continue;
        }

        let missing_ingredients: Vec<Ingredient> = ingredients
            .difference(&req.ingredients)
            .cloned()
            .map(|id| ctx.ingredients.get(&id).unwrap().to_owned())
            .collect::<Vec<Ingredient>>();

        drinks.push(DrinkResponse {
            id: ctx.drinks.ids[i].to_owned(),
            display_name: ctx.drinks.display_names[i].to_owned(),
            missing: HashSet::from_iter(missing_ingredients.into_iter()),
        });
    }

    drinks.sort_by(|a, b| a.missing.len().cmp(&b.missing.len()));

    Ok(web::Json(BarkeepResponse { drinks: drinks }))
}

fn main() -> std::io::Result<()> {
    let data: web::Data<Arc<Context>> = web::Data::new(Arc::new(Context {
        drinks: build_drinks(),
        ingredients: build_ingredients(),
    }));

    HttpServer::new(move || {
        App::new()
            .service(web::resource("/v1").route(web::post().to(index)))
            .service(Files::new("", "./public").index_file("index.html"))
            .register_data(data.clone())
            .default_service(web::route().to(|| HttpResponse::NotFound()))
    })
    .bind("127.0.0.1:8080")?
    .run()
}
