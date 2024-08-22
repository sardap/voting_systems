use actix_files::NamedFile;
use actix_web::{get, web, HttpRequest, Scope};
use actix_web::{
    http::header::{ContentDisposition, DispositionType},
    Error,
};
use std::path::Path;

lazy_static! {
    static ref BUILD_DIR: String = std::env::var("BUILD_DIR").unwrap();
}

#[get("/{filename:.*}")]
async fn get_static_file(req: HttpRequest) -> Result<actix_files::NamedFile, Error> {
    let path: std::path::PathBuf = req.match_info().query("filename").parse().unwrap();
    match actix_files::NamedFile::open(Path::new(BUILD_DIR.as_str()).join(path)) {
        Ok(file) => Ok(file
            .use_last_modified(true)
            .set_content_disposition(ContentDisposition {
                disposition: DispositionType::Attachment,
                parameters: vec![],
            })),
        Err(_) => Err(actix_web::error::ErrorNotFound("Not found")),
    }
}

async fn get_index() -> NamedFile {
    let path = Path::new(BUILD_DIR.as_str()).join("index.html");
    NamedFile::open(path).unwrap()
}

macro_rules! generate_get_functions {
    ($($func_name:ident, $path:literal),*) => {
        $(
            #[get($path)]
            async fn $func_name(_id: web::Path<String>) -> actix_web::Result<NamedFile> {
                Ok(get_index().await)
            }
        )*
    }
}

generate_get_functions!(
    preferential_voting_get,
    "/preferential_voting/{_id}",
    preferential_voting_get_results,
    "/preferential_voting/{_id}/results",
    single_transferable_vote_get,
    "/single_transferable_vote/{_id}",
    single_transferable_vote_get_results,
    "/single_transferable_vote/{_id}/results",
    borda_count_vote_get,
    "/borda_count/{_id}",
    borda_count_vote_get_results,
    "/borda_count/{_id}/results",
    approval_vote_get,
    "/approval/{_id}",
    approval_vote_get_results,
    "/approval/{_id}/results",
    star_vote_get,
    "/star/{_id}",
    star_vote_get_results,
    "/star/{_id}/results",
    cumulative_vote_get,
    "/cumulative/{_id}",
    cumulative_vote_get_results,
    "/cumulative/{_id}/results",
    anti_plurality_vote_get,
    "/anti_plurality/{_id}",
    anti_plurality_vote_get_results,
    "/anti_plurality/{_id}/results",
    single_party_vote_get,
    "/single_party/{_id}",
    single_party_vote_get_results,
    "/single_party/{_id}/results",
    three_two_one_vote_get,
    "/three_two_one/{_id}",
    three_two_one_vote_get_results,
    "/three_two_one/{_id}/results",
    condorcet_method_vote_get,
    "/condorcet_method/{_id}",
    condorcet_method_vote_get_results,
    "/condorcet_method/{_id}/results",
    majority_judgment_vote_get,
    "/majority_judgment/{_id}",
    majority_judgment_vote_get_results,
    "/majority_judgment/{_id}/results",
    score_vote_get,
    "/score/{_id}",
    score_vote_get_results,
    "/score/{_id}/results",
    usual_judgment_vote_get,
    "/usual_judgment/{_id}",
    usual_judgment_vote_get_results,
    "/usual_judgment/{_id}/results",
    single_non_transferable_vote_get,
    "/single_non_transferable/{_id}",
    single_non_transferable_vote_get_results,
    "/single_non_transferable/{_id}/results",
    quota_preferential_vic_labor_2024_get,
    "/quota_preferential_vic_labor_2024/{_id}",
    quota_preferential_vic_labor_2024_get_results,
    "/quota_preferential_vic_labor_2024/{_id}/results"
);

pub fn routes() -> Scope {
    web::scope("")
        .route("/", web::get().to(get_index))
        .route("/create", web::get().to(get_index))
        .route("/test", web::get().to(get_index))
        .route("/preferential_voting/create", web::get().to(get_index))
        .service(preferential_voting_get)
        .service(preferential_voting_get_results)
        .service(single_transferable_vote_get)
        .service(single_transferable_vote_get_results)
        .service(borda_count_vote_get)
        .service(borda_count_vote_get_results)
        .service(approval_vote_get)
        .service(approval_vote_get_results)
        .service(star_vote_get)
        .service(star_vote_get_results)
        .service(cumulative_vote_get)
        .service(cumulative_vote_get_results)
        .service(anti_plurality_vote_get)
        .service(anti_plurality_vote_get_results)
        .service(single_party_vote_get)
        .service(single_party_vote_get_results)
        .service(three_two_one_vote_get)
        .service(three_two_one_vote_get_results)
        .service(condorcet_method_vote_get)
        .service(condorcet_method_vote_get_results)
        .service(majority_judgment_vote_get)
        .service(majority_judgment_vote_get_results)
        .service(score_vote_get)
        .service(score_vote_get_results)
        .service(usual_judgment_vote_get)
        .service(usual_judgment_vote_get_results)
        .service(single_non_transferable_vote_get)
        .service(single_non_transferable_vote_get_results)
        .service(quota_preferential_vic_labor_2024_get)
        .service(quota_preferential_vic_labor_2024_get_results)
        .service(get_static_file)
}
