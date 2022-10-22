use super::models::{Media, MediaType};
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::{json, Map, Value};

type Result<T> = std::result::Result<T, AniListError>;

#[derive(Debug)]
pub enum AniListError {
    VariableError,
    ReqwestError(reqwest::Error),
}

impl From<reqwest::Error> for AniListError {
    fn from(err: reqwest::Error) -> AniListError {
        AniListError::ReqwestError(err)
    }
}

#[derive(Deserialize, Debug)]
pub struct QueryError {
    pub message: Option<String>,
    pub status: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct QueryResponse<R> {
    pub data: Option<R>,
    pub errors: Option<Vec<QueryError>>,
}

#[derive(Deserialize, Debug)]
pub struct MediaListPage {
    pub media: Option<Vec<Option<Media>>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PageResponse {
    pub page: MediaListPage,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MediaResponse {
    pub media: Media,
}

pub async fn query_graphql<R>(
    query: &str,
    variables: &Option<Map<String, Value>>,
) -> Result<QueryResponse<R>>
where
    R: DeserializeOwned,
{
    let query = if let Some(vars) = &variables {
        json!({ "query": query, "variables": vars })
    } else {
        json!({ "query": query })
    };

    let client = Client::new();
    let resp = client
        .post("https://graphql.anilist.co")
        .header("ContentType", "application/json")
        .json(&query)
        .send()
        .await?;

    let response: QueryResponse<R> = resp.json().await?;
    Ok(response)
}

pub async fn query_search(
    query: &str,
    media_type: MediaType,
) -> Result<QueryResponse<MediaResponse>> {
    let query_str = "query($search: String, $type: MediaType) {
		  Media(search: $search, type: $type) {
		    id
		    title {
		      romaji
		      english
		    }
			format
			description
		    status
		    episodes
		    chapters
			coverImage {
			  medium
			  color
			}
			bannerImage
		    averageScore
		  }
		}";
    let variables = json!({
        "search": query,
        "type": media_type,
    });
    if let serde_json::Value::Object(variables) = variables {
        query_graphql(query_str, &Some(variables)).await
    } else {
        Err(AniListError::VariableError)
    }
}

pub async fn query_media(id: i32) -> Result<QueryResponse<MediaResponse>> {
    let query_str = "query($id: Int) {
		  Media(id: $id) {
		    id
		    title {
		      romaji
		      english
		    }
			format
			description
		    status
		    episodes
		    chapters
			coverImage {
			  medium
			  color
			}
			bannerImage
		    averageScore
		  }
		}";
    let variables = json!({ "id": id });
    if let serde_json::Value::Object(variables) = variables {
        query_graphql(query_str, &Some(variables)).await
    } else {
        Err(AniListError::VariableError)
    }
}

pub async fn query_titles(
    search: &str,
    media_type: MediaType,
    count: i32,
) -> Result<QueryResponse<PageResponse>> {
    let query_str = "query($search: String, $type: MediaType, $count: Int) {
          Page(perPage: $count) {
		    media(search: $search, type: $type) {
		      id
		      title {
		        romaji
		        english
		      }
			}
		  }
		}";
    let variables = json!({
        "search": search,
        "type": media_type,
        "count": count,
    });
    if let serde_json::Value::Object(variables) = variables {
        query_graphql(query_str, &Some(variables)).await
    } else {
        Err(AniListError::VariableError)
    }
}
