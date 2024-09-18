use crate::common::http_text;
use crate::sdkman_cli::{get_sdkman_platform, SDKMAN_CANDIDATES_API};

pub fn manage_list(list_matches: &clap::ArgMatches) {
    if let Some(candidate_name) = list_matches.get_one::<String>("candidate") {
        list_candidate(candidate_name);
    } else {
        list_all_candidates();
    }
}

pub fn list_all_candidates() {
    let list_url = format!("{}/candidates/list", SDKMAN_CANDIDATES_API);
    println!("{}", http_text(&list_url));
}
pub fn list_candidate(candidate_name: &str) {
    let list_url = format!("{}/candidates/{}/{}/versions/list?installed=",
                           SDKMAN_CANDIDATES_API,
                           candidate_name,
                           get_sdkman_platform()
    );
    println!("{}", http_text(&list_url));
}
