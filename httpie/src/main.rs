use std::{collections::HashMap,str::FromStr};
use clap::{AppSettings,Parser};
use colored::*;
use mime::Mime;
use reqwest::{header,Client,Response,Url};
use anyhow::{anyhow,Result};
// 定义 HTTPie 的 CLI 的主入口，它包含若干个子命令
// 下面 /// 的注释是文档，clap 会将其作为 CLI 的帮助

/// A naive httpie implementation with Rust, can you imagine how easy it is?
#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "hoysics")]
#[clap(setting=AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

// 子命令分别对应不同的 HTTP 方法，目前只支持 get / post
#[derive(Parser, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
    // 我们暂且不支持其它 HTTP 方法
}

// get 子命令

/// feed get with an url and we will retrieve the response for you
#[derive(Parser, Debug)]
struct Get {
    /// HTTP 请求的 URL
    #[clap(parse(try_from_str=parse_url))]
    url: String,
}

// post 子命令。需要输入一个 URL，和若干个可选的 key=value，用于提供 json body

/// feed post with an url and optional key=value pairs. We will post the data
/// as JSON, and retrieve the response for you
#[derive(Parser, Debug)]
struct Post {
    /// HTTP 请求的 URL
    #[clap(parse(try_from_str=parse_url))]
    url: String,
    /// HTTP 请求的 body
    #[clap(parse(try_from_str=parse_kv_pair))]
    body: Vec<KvPair>,
}

#[derive(Debug,PartialEq)]
struct KvPair{
    k:String,
    v:String,
}

impl FromStr for KvPair {
   type Err=anyhow::Error;
   fn from_str(s: &str)-> Result<Self,Self::Err>{
    let mut split=s.split("=");
    let err=|| anyhow!(format!("Failed to parse {}",s));
    Ok(Self { 
        k: (split.next().ok_or_else(err)?).to_string(), 
        v: (split.next().ok_or_else(err)?).to_string(),
    })
   } 
}

fn parse_kv_pair(s: &str)->Result<KvPair> {
   Ok(s.parse()?) 
}

/// feed post with an url and optional key=value pairs. We will post the data
/// as JSON, and retrieve the response for you
fn parse_url(s: &str) ->  Result<String>{
   let _url:Url=s.parse()?; 
   Ok(s.into())
}

async fn get(client:Client,args:&Get)->Result<()>{
    let resp=client.get(&args.url).send().await?;
    // println!("{:?}",resp.text().await?);
    Ok(print_resp(resp).await?)
}

async fn post(client:Client,args:&Post)->Result<()>{
    let mut body=HashMap::new();
    for pair in  args.body.iter(){
       body.insert(&pair.k, &pair.v);
    }
    let resp=client.post(&args.url).json(&body).send().await?;
    // println!("{:?}",resp.text().await?);
    Ok(print_resp(resp).await?)
}

fn print_status(resp: &Response) {
   let status=format!("{:?} {}",resp.version(),resp.status()).blue();
   println!("{}\n",status); 
}

fn print_headers(resp: &Response) {
   for (name,value) in  resp.headers(){
      println!("{}: {:?}",name.to_string().green(),value);
   }
   print!("\n") ;
}

fn print_body(m: Option<Mime>,body:&String) {
   match m {
       Some(v) if v==mime::APPLICATION_JSON=>{
        println!("{}",jsonxf::pretty_print(body).unwrap().cyan())
       }
       _=>println!("{}",body),
   }
    print!("\n") ;
}

async fn print_resp(resp: Response) ->Result<()>{
   print_status(&resp);
   print_headers(&resp);
   let mime=get_content_type(&resp);
   let body=resp.text().await?;
   print_body(mime, &body);
   Ok(())
}

fn get_content_type(resp: &Response)->Option<Mime> {
   resp.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap()) 
}

#[tokio::main]
async fn main() ->Result<()>{
    let opts: Opts = Opts::parse();
    let mut headers=header::HeaderMap::new();
    headers.insert("X-POWERED-BY", "Rust".parse()?);
    headers.insert(header::USER_AGENT, "Rust Httpie".parse()?);
    let client=reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    let result=match opts.subcmd {
       SubCommand::Get(ref args) =>get(client,args).await?,
       SubCommand::Post(ref args) =>post(client,args).await?,
    };
    Ok(result)
} 

// 仅在 cargo test 时才编译
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url_works() {
        assert!(parse_url("abc").is_err());
        assert!(parse_url("http://abc.xyz").is_ok());
        assert!(parse_url("https://httpbin.org/post").is_ok());
    }

    #[test]
    fn parse_kv_pair_works() {
        assert!(parse_kv_pair("a").is_err());
        assert_eq!(
            parse_kv_pair("a=1").unwrap(),
            KvPair {
                k: "a".into(),
                v: "1".into()
            }
        );

        assert_eq!(
            parse_kv_pair("b=").unwrap(),
            KvPair {
                k: "b".into(),
                v: "".into()
            }
        );
    }
}