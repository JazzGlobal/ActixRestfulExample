use actix_web::web::Redirect;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::fmt::Display;

struct Person {
    id: i32,
    name: String,
    age: i32,
}

impl Serialize for Person {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Person", 3)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("age", &self.age)?;
        state.end()
    }
}

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let serialized = serde_json::to_string_pretty(&self);
        match serialized {
            Ok(serialized_person) => write!(f, "{}", &serialized_person),
            Err(e) => write!(
                f,
                "id: {}, name: {}, age: {}",
                &self.id, &self.name, &self.age
            ),
        }
    }
}

// Mock our data source -- This is also a repository call Mock.
fn get_person_list() -> Vec<Person> {
    let person1 = Person {
        id: 1,
        name: String::from("chris"),
        age: 26,
    };
    let person2 = Person {
        id: 2,
        name: String::from("karli"),
        age: 24,
    };
    vec![person1, person2]
}

// Mock our get by id repository call.
fn get_person_by_id(id: i32) -> Option<Person> {
    for x in get_person_list() {
        if x.id == id {
            return Some(x);
        }
    }
    None
}

#[get("/person")]
async fn person() -> impl Responder {
    let person_list = get_person_list();

    let serialized = serde_json::to_string_pretty(&person_list).unwrap();
    HttpResponse::Ok().body(serialized)
}

#[get("/person/")]
async fn person_redirect() -> impl Responder {
    Redirect::to("/person")
}

#[get("/person/{name}")]
async fn echo(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    match id.parse::<i32>() {
        Ok(n) => {
            let found_person = get_person_by_id(n);
            match found_person {
                Some(x) => HttpResponse::Ok().body(x.to_string()),
                // Handle Person not found.
                None => {
                    return HttpResponse::NotFound()
                        .body(String::from("Did not find Person with given id."))
                }
            }
        }
        // Generic handling of invalid i32 parsing.
        Err(e) => HttpResponse::InternalServerError()
            .body(String::from("Failed to parse Person.Id argument.")),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(person)
            .service(echo)
            .service(person_redirect)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
