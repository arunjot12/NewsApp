
use std::{error::Error};
use dotenv::dotenv;

use colour::{dark_blue,yellow};

use newsapi::{Articles, get_articles};

fn render_items(articles:&Articles){

   
    // theme.print_text("# /Top headlines \n\n");
    for i in &articles.articles{
    //     theme.print_text(&format!("`{}`",i.title));
    //    theme.print_text(&format!("> *{}*",i.url));
    //    theme.print_text("---");

        dark_blue!(">{}\n",i.title);
        yellow!("> {}\n",i.url);

    }
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv();

    let api_key=std::env::var("API_KEY")?;
    let url=" https://newsapi.org/v2/top-headlines?country=us&apiKey=";

    let url=format!("{}{}",url,api_key  );
    let articles= get_articles(&url)?;



    render_items(&articles);

    Ok(())
   
}



// #[derive(Deserialize,Debug)]
// pub struct Articles{
//    pub articles:Vec<Article>
// }
// #[derive(Deserialize,Debug)]
// pub struct Article{
//    pub title: String,
//    pub url:String

// }
// fn get_articles(url:&str) -> Result<Articles, Box<dyn Error>>{
//     let response =ureq::get(url).call()?.into_string()?;
//     let articles: Articles= serde_json::from_str(&response)?;
//    Ok(articles)

// }