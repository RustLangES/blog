use crate::models::hashnode_article::{
    ArticleFetched, ArticleFetchedPost, Data, HashnodeResponse,
};
use serde::Serialize;

#[derive(Serialize)]
struct GraphQLQuery<'a> {
    query: &'a str,
}

pub async fn fetch_hashnode() -> Result<Vec<ArticleFetchedPost>, Box<dyn std::error::Error>> {
    let url = "https://api.hashnode.com/";
    let client = reqwest::Client::new();

    let graphql_query = r#"
        {
            user(username:"Miuler"){
                publication{
                    posts{
                        slug
                    }
                }
            }
        }
    "#;
    let query = GraphQLQuery {
        query: graphql_query,
    };

    let resp = client
        .post(url)
        .header("Accept", "application/json")
        .header("User-Agent", "RustLangEs")
        .json(&query)
        .send()
        .await?
        .json::<HashnodeResponse>()
        .await?;

    let mut articles = Vec::with_capacity(10);
    let Some(Data { mut user }) = resp.data else {
        return Err("No data found".into());
    };
    user.publication
        .posts
        .truncate(user.publication.posts.len() - 3);

    for article in user.publication.posts {
        let article_complete = get_article_by_slug(article.slug).await?;
        let article_complete = match article_complete.data {
            Some(article_complete) => article_complete,
            None => continue,
        };

        if article_complete
            .post
            .tags
            .iter()
            .any(|tag| tag.name == "Rust")
        {
            articles.push(article_complete.post);
        }
    }

    Ok(articles)
}

pub async fn get_article_by_slug(slug: String) -> Result<ArticleFetched, reqwest::Error> {
    let url = "https://api.hashnode.com/";
    let client = reqwest::Client::new();
    let graphql_query = r#"
    {
        post(slug: "{}", hostname: "") {
          slug
          title
          tags {
            name
          }
          dateAdded
          contentMarkdown
          brief
          publication {
            links {
              hashnode
              website
              github
              twitter
            }
            username
          }
        }
    }
    "#;

    let graphql_query = graphql_query.replace("{}", &slug);

    let query = GraphQLQuery {
        query: &graphql_query,
    };

    let resp = client
        .post(url)
        .json(&query)
        .header("Accept", "application/json")
        .header("User-Agent", "RustLangEs")
        .send()
        .await?
        .json::<ArticleFetched>()
        .await?;

    Ok(resp)
}
