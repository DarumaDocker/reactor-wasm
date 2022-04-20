use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;
use serde_json::{Result, Value};

#[wasmedge_bindgen]
pub fn run(s: String) -> String {
	let result: Result<Value> = serde_json::from_str(s.as_str());

	if let Ok(pl) = result {
		match pl.get("sender_email") {
			Some(email) => {
				if let Some(action) = pl["action"].as_str() {
					if action == "created" {
						return format!(r#"From: me
To: {}
Subject: Thanks for your star!

Hi {}, we have received your star to our repository {}.
We are so appreciative and wish you have more fun with open source.

Best regards"#,
							email.as_str().unwrap(),
							pl["sender"]["login"].as_str().unwrap(),
							pl["repository"]["full_name"].as_str().unwrap()
						);
					} else {
						return format!(r#"From: me
To: {}
Subject: Sorry to lose you

Hi {}, we are very disappointed that you have unstarred our repository {}.
Hope you can give us more advice to make us getting better.

Best regards"#,
							email.as_str().unwrap(),
							pl["sender"]["login"].as_str().unwrap(),
							pl["repository"]["full_name"].as_str().unwrap()
						)
					}
				}
			}
			None => {
				return "".to_string();
			}
		}
	}

	return "".to_string();
}