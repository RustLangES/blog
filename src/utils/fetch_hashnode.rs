use crate::models::hashnode_article::{Data, HashNodeArticle, HashnodeResponse};
use serde::Serialize;

#[derive(Serialize)]
struct GraphQLQuery<'a> {
    query: &'a str,
}

pub async fn fetch_hashnode() -> Result<Vec<HashNodeArticle>, Box<dyn std::error::Error>> {
    let url = "https://gql.hashnode.com/";
    let client = reqwest::Client::new();

    let graphql_query = r#"
        {
            publication(host: "miuler.com"){
                posts(first: 1, filter: {
                    tagSlugs: ["rust"]
                }){
                    edges{
                        node {
                            slug
                            title
                            tags {
                              name
                            }
                            publishedAt
                            content {
                              markdown
                            }
                            brief
                            publication {
                              links {
                                hashnode
                                website
                                github
                                twitter
                              }
                              author {
                                username
                              }
                            }
                        }
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

    let Some(Data { publication }) = resp.data else {
        return Err("No data found".into());
    };

    let articles = publication
        .posts
        .edges
        .iter()
        .map(|post| post.node.clone())
        .collect::<Vec<_>>();

    Ok(articles)
}
