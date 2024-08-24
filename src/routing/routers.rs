// coding=utf-8

// Copyright [2024] [SkywardAI]
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//        http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use axum::{response::Html, routing::get, Router, Json};
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;


pub fn register_routers() -> Router {
    let inference = Router::new().route("/", get(inference));

    Router::new()
        .route("/", get(handler))
        .nest("/inference", inference)
        .merge(SwaggerUi::new("/docs").url("/api-docs/openai.json", ApiDoc::openapi()))
}

#[derive(Serialize, ToSchema)]
struct InferResponse {
    message: String,
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Welcome!</h1>")
}


#[utoipa::path(
    get,
    path="/inference",
    tag = "Inference",
    responses(
        (status=200, description="Inference", body=InferResponse)
    )
)]

async fn inference() -> Json<InferResponse>{
    let response=InferResponse{
        message: "Inference".to_string()
    };
    Json(response)
}

#[derive(OpenApi)]
#[openapi(
    paths(
        inference
    ),
    components(schemas(InferResponse)),
)]
struct ApiDoc;