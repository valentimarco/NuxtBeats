use tauri::webview::Cookie;

pub fn cookies_to_netscape_format(cookies: Vec<Cookie<'static>>) -> String {
    let mut netscape_format = String::new();
    netscape_format.push_str("# Netscape HTTP Cookie File\n\n");

    let mut filtered = Vec::new();

    for cookie in cookies {
        if let Some(x) = cookie.domain() {
            if x.contains("youtube.com") {
                filtered.push(cookie)
            }
        }
    }

    for cookie in filtered {
        let domain = match cookie.domain() {
            Some(x) => x,
            None => continue,
        };
        let sub_domain = match cookie.same_site() {
            Some(x) => match x {
                tauri::webview::cookie::SameSite::Lax => true,
                _ => false,
            },
            None => false,
        };
        let path = match cookie.path() {
            Some(x) => x,
            None => continue,
        };
        let secure = match cookie.secure() {
            Some(x) => x,
            None => false,
        };
        let expires = match cookie.expires() {
            Some(x) => match x.datetime() {
                Some(y) => y.unix_timestamp(),
                None => 0,
            },
            None => continue,
        };
        let name = cookie.name();
        let value = cookie.value();

        netscape_format.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            String::from(".") + domain,
            sub_domain,
            path,
            secure,
            expires,
            name,
            value
        ));
    }

    netscape_format
}

#[cfg(test)]
mod test {
    use tauri::webview::{
        cookie::{
            time::{macros::datetime, Duration},
            Expiration,
        },
        Cookie,
    };

    use crate::cookie::cookies_to_netscape_format;

    #[test]
    fn test_format() {
        let cookie_test = Cookie::build(("pippo", "morto"))
            .domain("youtube.com")
            .secure(true)
            .http_only(true)
            .path("/")
            .expires(Expiration::from(datetime!(2024-01-01 12:59:59.5 UTC)))
            .max_age(Duration::hours(1))
            .build();

        let result = cookies_to_netscape_format(vec![cookie_test]);
        assert_eq!(
            "# Netscape HTTP Cookie File\n\n.youtube.com\tfalse\t/\ttrue\t1704113999\tpippo\tmorto\n",
            result
        )
    }
}
