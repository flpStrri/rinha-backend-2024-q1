use crate::structs::{api, person};
use axum::extract::{Path, Query, State};
use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};
use uuid::Uuid;

#[tracing::instrument(name = "Looking for a developer", skip(client))]
pub async fn get_person(State(client): State<Database>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let devs_store: Collection<person::Person> = client.collection("devs");
    let found_dev = devs_store.find_one(doc! {"_id": id}, None).await;

    match found_dev {
        Ok(Some(dev)) => Ok((
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            Json(api::PersonBody {
                id: dev.id,
                name: dev.name,
                nickname: dev.nickname,
                birth_date: dev.birth_date,
                stacks: dev.stacks,
            }),
        )),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(error) => {
            println!("get_by_id: {}", error);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[tracing::instrument(name = "Adding a new developer", skip(client, body))]
pub async fn create_person(
    State(client): State<Database>,
    Json(body): Json<api::CreatePersonBody>,
) -> impl IntoResponse {
    let user = person::Person {
        id: Uuid::new_v4(),
        name: body.name,
        nickname: body.nickname,
        birth_date: body.birth_date,
        stacks: body.stacks,
    };
    let devs_store: Collection<person::Person> = client.collection("devs");
    let inserted_result = devs_store.insert_one(&user, None).await;
    match inserted_result {
        Ok(_) => Ok((
            StatusCode::CREATED,
            [
                (header::LOCATION, format!("/pessoas/{}", &user.id)),
                (header::CONTENT_TYPE, String::from("application/json")),
            ],
            Json(api::PersonBody {
                id: user.id,
                name: user.name,
                nickname: user.nickname,
                birth_date: user.birth_date,
                stacks: user.stacks,
            }),
        )),
        Err(error) => {
            println!("post: {}", error);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[tracing::instrument(name = "Searching for a developer", skip(client))]
pub async fn search_persons(
    State(client): State<Database>,
    Query(query): Query<api::SearchPersonQuery>,
) -> impl IntoResponse {
    let devs_store: Collection<person::Person> = client.collection("devs");

    let search_cursor = devs_store
        .find(
            doc! {
                "$or": [
                    {
                        "name": mongodb::bson::Regex{
                            pattern: query.search_term.clone(),
                            options: String::from("i"),
                        }
                    },
                    {
                        "stacks": {
                            "$in": [
                                mongodb::bson::Regex{
                                    pattern: query.search_term.clone(),
                                    options: String::from("i"),
                                }
                            ]
                        }
                    },
                    {
                        "nickname": mongodb::bson::Regex{
                            pattern: query.search_term.clone(),
                            options: String::from("i"),
                        }
                    }
                ]
            },
            None,
        )
        .await;
    match search_cursor {
        Ok(cursor) => {
            let found_devs = cursor.try_collect().await.unwrap_or_else(|_| vec![]);

            Ok((
                StatusCode::OK,
                [(header::CONTENT_TYPE, "application/json")],
                Json(
                    found_devs
                        .iter()
                        .map(|dev| api::PersonBody {
                            id: dev.id,
                            name: dev.name.clone(),
                            nickname: dev.nickname.clone(),
                            birth_date: dev.birth_date,
                            stacks: dev.stacks.clone(),
                        })
                        .collect::<Vec<api::PersonBody>>(),
                ),
            ))
        }
        Err(error) => {
            println!("persons?t=QUERY: {}", error);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
