mod types;

wit_bindgen_guest_rust::import!("/Users/shobhitsharma/repos/fifthtry/fpm-utils/wits/host.wit");

#[fpm_utils_macro::wasm_backend]
fn handlerequest(a: guest_backend::Httprequest) -> guest_backend::Httpresponse {
    let base_url_header_key = String::from("X-FPM-KAMERI-SUPABASE-BASE-URL");
    let apikey_header_key = String::from("X-FPM-KAMERI-SUPABASE-API-KEY");
    let (_, base_url) = a
        .headers
        .iter()
        .find(|(key, _)| key == &base_url_header_key)
        .expect(
            format!(
                "{base_url_header_key} not found in the request. Please configure app properly"
            )
            .as_str(),
        );
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
