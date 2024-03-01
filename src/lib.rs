use anyhow::Result;
use serde::Serialize;
use spin_sdk::{
    http::{IntoResponse, Params, Request, Response},
    http_component, http_router,
    sqlite::Connection,
};

#[http_component]
fn handle_fermy_todo(req: Request) -> Response {
    println!("Handling request to {:?}", req.header("spin-full-url"));
    let router = http_router! {
        GET "api/todos" => get_todos
    };
    router.handle(req)
}

const DATE_FORMAT: &str = "[year]-[month]-[day]";

#[derive(serde::Deserialize)]
struct GetParams {
    #[serde(default)]
    due: Option<bool>,
    #[serde(default)]
    complete: Option<bool>,
}

pub fn get_todos(req: Request, _params: Params) -> Result<impl IntoResponse> {
    let query = req.query();
    let params: GetParams = serde_qs::from_str(query)?;

    let due_date = params.due.map(|due| {
        let format = time::format_description::parse(DATE_FORMAT).unwrap();
        let today = time::OffsetDateTime::now_utc()
            .date()
            .format(&format)
            .unwrap();
        if due {
            format!("due_date <= '{today}'")
        } else {
            format!("(due_date > '{today}' OR due_date is NULL)")
        }
    });

    let incomplete = params.complete.map(|complete| {
        if complete {
            "is_completed == TRUE"
        } else {
            "is_completed == FALSE"
        }
    });

    let w = match (due_date, incomplete) {
        (Some(due_date), Some(incomplete)) => format!("WHERE {due_date} AND {incomplete}"),
        (Some(due_date), None) => format!("WHERE {due_date}"),
        (None, Some(incomplete)) => format!("WHERE {incomplete}"),
        (None, None) => String::new(),
    };

    let conn = Connection::open_default()?;
    let rowset = conn.execute(&format!("SELECT * FROM todos {w};"), &[])?;
    let todos: Vec<_> = rowset
        .rows()
        .map(|row| ToDo {
            item: row.get::<u32>("id").unwrap(),
        })
        .collect();
    let body = serde_json::to_vec(&todos)?;

    Ok(Response::builder().status(200).body(body).build())
}

#[derive(Serialize)]
struct ToDo {
    item: u32,
}
