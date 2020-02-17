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
    recipe: HashMap<String, String>,
    missing: HashSet<String>,
    ingredients: HashMap<String, Ingredient>,
}

struct Drinks {
    ids: Vec<String>,
    display_names: Vec<String>,
    ingredient_ids: Vec<HashSet<String>>,
    recipes: Vec<HashMap<String, String>>,
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
    ingredient!("rum", "Rum");
    ingredient!("simpleSyrup", "Simple Syrup");
    ingredient!("sodaWater", "Soda Water");
    ingredient!("sugarCube", "Sugar Cube");
    ingredient!("sweetVermouth", "Sweet Vermouth");
    ingredient!("tripleSec", "Triple Sec");
    ingredient!("vanillaExtract", "Vanilla Extract");
    ingredient!("vodka", "Vodka");
    ingredient!("water", "Water");
    ingredient!("whiteRum", "White Rum");

    ingredients
}

fn build_drinks() -> Drinks {
    let mut ids: Vec<String> = vec![];
    let mut display_names: Vec<String> = vec![];
    let mut drink_ingredients: Vec<HashSet<String>> = vec![];
    let mut drink_recipes: Vec<HashMap<String, String>> = vec![];

    macro_rules! drink {
        ($id:expr, $dn:expr, [$($ingvec:expr),*]) => {
            ids.push(String::from($id));
            display_names.push(String::from($dn));
            drink_ingredients.push(HashSet::from_iter(vec![$(String::from($ingvec.0)),*]));
            let mut recipe = HashMap::new();
            $(
                recipe.insert(String::from($ingvec.0), String::from($ingvec.1));
            )*
            drink_recipes.push(recipe);
        };
    }

    drink!(
        "alexander",
        "Alexander",
        [
            ("cognac", "3 cl"),
            ("cremeDeCacao", "3 cl"),
            ("cream", "3 cl")
        ]
    );
    drink!(
        "americano",
        "Americano",
        [
            ("campari", "3 cl"),
            ("sweetVermouth", "3 cl"),
            ("sodaWater", "splash")
        ]
    );
    drink!(
        "angelFace",
        "Angel Face",
        [
            ("gin", "3 cl"),
            ("apricotBrandy", "3 cl"),
            ("calvados", "3 cl")
        ]
    );
    drink!(
        "aviation",
        "Aviation",
        [
            ("gin", "4.5 cl"),
            ("lemonJuice", "1.5 cl"),
            ("maraschino", "1.5 cl")
        ]
    );
    drink!(
        "bacardi",
        "Bacardi",
        [
            ("whiteRum", "4.5 cl"),
            ("limeJuice", "2 cl"),
            ("grenadine", "1 cl")
        ]
    );
    drink!(
        "betweenTheSheets",
        "Between The Sheets",
        [
            ("whiteRum", "3 cl"),
            ("cognac", "3 cl"),
            ("tripleSec", "3 cl"),
            ("lemonJuice", "2 cl")
        ]
    );
    drink!(
        "casino",
        "Casino",
        [
            ("gin", "4 cl"),
            ("maraschino", "1 cl"),
            ("orangeBitters", "1 cl"),
            ("lemonJuice", "1 cl")
        ]
    );
    drink!(
        "cloverClub",
        "Clover Club",
        [
            ("gin", "4.5 cl"),
            ("lemonJuice", "1.5 cl"),
            ("raspberrySyrup", "1.5 cl"),
            ("eggWhite", "1 cl")
        ]
    );
    drink!(
        "daiquiri",
        "Daiquiri",
        [
            ("whiteRum", "4.5 cl"),
            ("limeJuice", "2.5 cl"),
            ("simpleSyrup", "1.5 cl")
        ]
    );
    drink!(
        "derby",
        "Derby",
        [
            ("gin", "6 cl"),
            ("peachBitters", "2 cl"),
            ("mintLeaves", "2 cl")
        ]
    );
    drink!(
        "martini",
        "Martini",
        [("gin", "6 cl"), ("dryVermouth", "1 cl")]
    );
    drink!(
        "ginFizz",
        "Gin Fizz",
        [
            ("gin", "4.5 cl"),
            ("lemonJuice", "3 cl"),
            ("gommeSyrup", "1 cl"),
            ("sodaWater", "8 cl")
        ]
    );
    drink!(
        "johnCollins",
        "John Collins",
        [
            ("gin", "4.5 cl"),
            ("lemonJuice", "3 cl"),
            ("simpleSyrup", "1.5 cl"),
            ("sodaWater", "6 cl")
        ]
    );
    drink!(
        "manhattan",
        "Manhattan",
        [
            ("rye", "5 cl"),
            ("sweetVermouth", "2 cl"),
            ("angosturaBitters", "1 dash")
        ]
    );
    drink!(
        "maryPickford",
        "Mary Pickford",
        [
            ("whiteRum", "6 cl"),
            ("pineappleJuice", "6 cl"),
            ("grenadine", "1 cl"),
            ("maraschino", "1 cl")
        ]
    );
    drink!(
        "monkeyGland",
        "Monkey Gland",
        [
            ("gin", "5 cl"),
            ("orangeJuice", "3 cl"),
            ("absinthe", "2 cl"),
            ("grenadine", "2 cl")
        ]
    );
    drink!(
        "negroni",
        "Negroni",
        [
            ("gin", "3 cl"),
            ("sweetVermouth", "3 cl"),
            ("campari", "3 cl")
        ]
    );
    drink!(
        "oldFashioned",
        "Old Fashioned",
        [
            ("bourbon", "4.5 cl"),
            ("angosturaBitters", "2 dashes"),
            ("sugarCube", "1"),
            ("water", "few dashes")
        ]
    );
    drink!(
        "paradise",
        "Paradise",
        [
            ("gin", "3.5 cl"),
            ("apricotBrandy", "2 cl"),
            ("orangeJuice", "1.5 cl")
        ]
    );
    drink!(
        "plantersPunch",
        "Planter's Punch",
        [
            ("rum", "4.5 cl"),
            ("orangeJuice", "3.5 cl"),
            ("pineappleJuice", "3.5 cl"),
            ("lemonJuice", "2 cl"),
            ("grenadine", "1 cl"),
            ("simpleSyrup", "1 cl"),
            ("angosturaBitters", "few dashes")
        ]
    );
    drink!(
        "portoFlip",
        "Porto Flip",
        [
            ("brandy", "1.5 cl"),
            ("port", "4.5 cl"),
            ("eggYolk", "1 cl")
        ]
    );
    drink!(
        "ramosGinFizz",
        "Ramos Gin Fizz",
        [
            ("gin", "4.5 cl"),
            ("limeJuice", "1.5 cl"),
            ("lemonJuice", "1.5 cl"),
            ("simpleSyrup", "3 cl"),
            ("cream", "6 cl"),
            ("eggWhite", "1"),
            ("orangeFlowerWater", "3 dashes"),
            ("vanillaExtract", "2 drops"),
            ("sodaWater", "top")
        ]
    );
    drink!(
        "rustyNail",
        "Rusty Nail",
        [("scotch", "4.5 cl"), ("drambuie", "2.5 cl")]
    );
    drink!(
        "sazerac",
        "Sazerac",
        [
            ("cognac", "5 cl"),
            ("absinthe", "1 cl"),
            ("sugarCube", "1"),
            ("peychaudsBitters", "2 dashes")
        ]
    );
    drink!(
        "screwdriver",
        "Screwdriver",
        [("vodka", "5 cl"), ("orangeJuice", "10 cl")]
    );
    drink!(
        "sidecar",
        "Sidecar",
        [
            ("cognac", "5 cl"),
            ("tripleSec", "2 cl"),
            ("lemonJuice", "2 cl")
        ]
    );
    drink!(
        "stinger",
        "Stinger",
        [("cognac", "5 cl"), ("cremeDeMenthe", "2 cl")]
    );
    drink!(
        "tuxedo",
        "Tuxedo",
        [
            ("gin", "3 cl"),
            ("dryVermouth", "3 cl"),
            ("maraschino", "0.5 barspoon"),
            ("absinthe", "0.25 barspoon"),
            ("orangeBitters", "3 dashes")
        ]
    );
    drink!(
        "whiskeySour",
        "Whiskey Sour",
        [
            ("bourbon", "4.5 cl"),
            ("lemonJuice", "3 cl"),
            ("gommeSyrup", "1.5 cl"),
            ("eggWhite", "1 dash")
        ]
    );
    drink!(
        "whiteLady",
        "White Lady",
        [
            ("gin", "4 cl"),
            ("tripleSec", "3 cl"),
            ("lemonJuice", "2 cl")
        ]
    );

    Drinks {
        ids: ids,
        display_names: display_names,
        ingredient_ids: drink_ingredients,
        recipes: drink_recipes,
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

        let missing_ingredients = ingredients
            .difference(&req.ingredients)
            .cloned()
            .map(|id| ctx.ingredients.get(&id).unwrap().to_owned())
            .map(|ingredient| ingredient.id)
            .collect::<Vec<String>>();

        let mut display_map: HashMap<String, Ingredient> = HashMap::new();
        for ing in ingredients {
            display_map.insert(ing.to_owned(), ctx.ingredients.get(ing).unwrap().to_owned());
        }

        drinks.push(DrinkResponse {
            id: ctx.drinks.ids[i].to_owned(),
            display_name: ctx.drinks.display_names[i].to_owned(),
            missing: HashSet::from_iter(missing_ingredients.into_iter()),
            recipe: ctx.drinks.recipes[i].to_owned(),
            ingredients: display_map,
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
    .bind("0.0.0.0:8086")?
    .run()
}
