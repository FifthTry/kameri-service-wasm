mod types;

wit_bindgen_guest_rust::import!("../fpm-utils/wits/host.wit");

#[fpm_utils_macro::wasm_backend]
fn handlerequest(a: guest_backend::Httprequest) -> guest_backend::Httpresponse {
    let project_id_header_key = String::from("X-FPM-KAMERI-SUPABASE-PROJECT-ID");
    let apikey_header_key = String::from("X-FPM-KAMERI-SUPABASE-API-KEY");
    let (_, project_id) = a
        .headers
        .iter()
        .find(|(key, _)| key == &project_id_header_key)
        .expect(
            format!(
                "{project_id_header_key} not found in the request. Please configure app properly"
            )
            .as_str(),
        );
    let base_url = format!("https://{project_id}.supabase.co/rest/v1");
    let (_, apikey) = a
        .headers
        .iter()
        .find(|(key, _)| key == &apikey_header_key)
        .expect(
            format!("{apikey_header_key} not found in the request. Please configure app properly")
                .as_str(),
        );
    let header_map = [("Content-Type", "application/json"), ("apiKey", apikey)];
    let resp = match a.path.as_str() {
        "/api/todos/" => host::http(host::Httprequest {
            path: format!("{base_url}/todo").as_str(),
            method: "GET",
            payload: "",
            headers: &header_map,
        }),
        "/api/add-todo/" => {
            let req: types::NewTodoRequest = serde_json::from_str(a.payload.as_str()).unwrap();
            host::http(host::Httprequest {
                path: format!("{base_url}/todo").as_str(),
                method: "POST",
                payload: serde_json::to_string(&req).unwrap().as_str(),
                headers: &header_map,
            })
        }
        "/api/update-todo/" => {
            let payload: types::UpdateTodoRequest =
                serde_json::from_str(a.payload.as_str()).unwrap();
            let instance_id = payload.id;
            host::http(host::Httprequest {
                path: format!("{base_url}/todo?id=eq.{instance_id}").as_str(),
                method: "PATCH",
                payload: serde_json::to_string(&payload).unwrap().as_str(),
                headers: &header_map,
            })
        }
        x => {
            return guest_backend::Httpresponse {
                data: format!("Unhandled route {x}"),
                success: false,
            }
        }
    };
    guest_backend::Httpresponse {
        data: resp.data,
        success: true,
    }
}
