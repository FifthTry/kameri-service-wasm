mod types;

wit_bindgen_guest_rust::import!("/Users/shobhitsharma/repos/fifthtry/fpm-utils/wits/host.wit");

#[fpm_utils_macro::wasm_backend]
fn handlerequest(a: guest_backend::Httprequest) -> guest_backend::Httpresponse {
    let base_url = "https://nxxvgyrffdviepwradwr.supabase.co/rest/v1";
    let apikey = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6Im54eHZneXJmZmR2aWVwd3JhZHdyIiwicm9sZSI6ImFub24iLCJpYXQiOjE2NjU1NzY1NDMsImV4cCI6MTk4MTE1MjU0M30.VKvU_1UNlAiH5qHUppYslgs2sTRWUBY0tSLCc5VY3dE";
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
