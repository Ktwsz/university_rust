use std::env;

fn get_url() -> Option<String> {
    let mut args = env::args();
    let _ = args.next();
    args.next()
}

fn get_url_data(url: &String) -> (&str, &str, &str) {
    let mut dashes = [0; 3];
    let mut dash_ix = 0;
    let mut query_params_ix = 0;

    for (i, val) in url.chars().enumerate() {
        if val == '/' && dash_ix < 3{
            dashes[dash_ix] = i;
            dash_ix += 1;
        }
        if val == '?' {
            query_params_ix = i;
        }
    }

    let i1 = dashes[1]+1;
    let i2 = dashes[2];
    let url_len = url.len();

    if i2 == 0 {
        return (&url[i1..url_len], "", "");
    }

    let domain = &url[i1..i2];
    if query_params_ix == 0 {
        return (domain, &url[i2..url_len], "");
    }
    
    let query_path = &url[i2..query_params_ix];
    let query_params = &url[(query_params_ix+1)..url_len];

    (domain, query_path, query_params)
}

fn main() {
    let url = get_url().unwrap();

    let (domain, query_path, query_params) = get_url_data(&url);

    println!("domain is {}", domain);
    println!("query path is {}", query_path);
    println!("query params is {}", query_params);
}
