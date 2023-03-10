use entity::tasks;
use rocket::{response::{Redirect, Flash}, serde::json::{Json, json}};
use rocket_dyn_templates::Template;
use sea_orm::{EntityTrait, Set, ActiveModelTrait, QueryOrder};
use sea_orm_rocket::Connection;
use rocket::form::Form;

use entity::tasks::Entity as Tasks;

use crate::{pool::Db, user_routes::AuthenticatedUser, render_routes::DatabaseError};

pub fn redirect_to_login() -> Redirect {
    Redirect::to("/login")
}

// ------------------------------------------------TASKS-----------------------------------------------------
#[post("/addtask", data="<task_form>")]
pub async fn add_task(conn: Connection<'_, Db>, task_form: Form<tasks::Model>, user: AuthenticatedUser) -> Flash<Redirect> {
    let db = conn.into_inner();
    let task = task_form.into_inner();

    let active_task: tasks::ActiveModel = tasks::ActiveModel {
        creator: Set(task.creator),
        title: Set(task.title),
        body: Set(task.body),
        completed: Set(task.completed),
        user_id: Set(user.user_id),
        ..Default::default()
    };

    match active_task.insert(db).await {
        Ok(result) => result,
        Err(_) => {
            return Flash::error(Redirect::to("/"), "Issue creating the task");
        }
    };

    Flash::success(Redirect::to("/"), "Task created!")
}

#[post("/addtask", rank = 2)]
pub async fn add_task_redirect() -> Redirect {
    redirect_to_login()
}


// Index will do the same thing, but good to have for debugging purposes
#[get("/readtasks")]
pub async fn read_tasks(conn: Connection<'_, Db>) -> Result<Json<Vec<tasks::Model>>, DatabaseError> {
    let db = conn.into_inner();

    Ok(Json(
        Tasks::find()
            .order_by_asc(tasks::Column::Id)
            .all(db)
            .await?
    ))
}

#[put("/edittask", data="<task_form>")]
pub async fn edit_task(conn: Connection<'_, Db>, task_form: Form<tasks::Model>, _user: AuthenticatedUser) -> Flash<Redirect> {
    let db = conn.into_inner();
    let task = task_form.into_inner();

    let task_to_update = match Tasks::find_by_id(task.id).one(db).await {
        Ok(result) => result,
        Err(_) => {
            return Flash::error(Redirect::to("/"), "Issue editing the task");
        }
    };

    let mut task_to_update: tasks::ActiveModel = task_to_update.unwrap().into();
    task_to_update.creator = Set(task.creator);
    task_to_update.title = Set(task.title);
    task_to_update.body = Set(task.body);
    task_to_update.completed = Set(task.completed);

    match task_to_update.update(db).await {
        Ok(result) => result,
        Err(_) => {
            return Flash::error(Redirect::to("/"), "Issue editing the task");
        }
    };

    Flash::success(Redirect::to("/"), "Task edited succesfully!")
}

#[put("/edittask", rank = 2)]
pub async fn edit_task_redirect() -> Redirect {
    redirect_to_login()
}


#[get("/edit/<id>")]
pub async fn edit_task_page(conn: Connection<'_, Db>, id: i32, _user: AuthenticatedUser) -> Result<Template, DatabaseError> {
    let db = conn.into_inner();
    let task = Tasks::find_by_id(id).one(db).await?.unwrap();

    Ok(Template::render(
        "edit_task_form", 
        json!({
            "task": task
        })
    ))
}

#[get("/edit/<id>", rank = 2)]
pub async fn edit_task_page_redirect(id: i32) -> Redirect {
    redirect_to_login()
}


#[delete("/deletetask/<id>")]
pub async fn delete_task(conn: Connection<'_, Db>, id: i32, _user: AuthenticatedUser) -> Flash<Redirect> {
    let db = conn.into_inner();
    let _result = match Tasks::delete_by_id(id).exec(db).await {
        Ok(value) => value,
        Err(_) => {
            return Flash::error(Redirect::to("/"), "Issue deleting the task");
        }
    };

    Flash::success(Redirect::to("/"), "Task succesfully deleted!")
}

#[delete("/deletetask/<id>", rank = 2)]
pub async fn delete_task_redirect(id: i32) -> Redirect {
    redirect_to_login()
}