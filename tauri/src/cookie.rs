use tauri::webview::Cookie;

/// Standard header used by many tools for Netscape cookies.txt files.
pub const NETSCAPE_COOKIE_FILE_HEADER: &str = "\
# Netscape HTTP Cookie File
# http://curl.haxx.se/rfc/cookie_spec.html
# This is a generated file!  Do not edit.

";

pub fn cookies_to_netscape_format(cookies: Vec<Cookie<'static>>) -> String {
    let mut netscape_format = String::new();
    netscape_format.push_str(NETSCAPE_COOKIE_FILE_HEADER);

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
            None => "",
        };
        //some cookies are setted as false even if they are true
        let sub_domain = match cookie.same_site() {
            Some(x) => match x {
                tauri::webview::cookie::SameSite::Lax => "TRUE",
                _ => "TRUE",
            },
            None => "TRUE",
        };
        let path = match cookie.path() {
            Some(x) => x,
            None => "/",
        };
        let secure = match cookie.secure() {
            Some(_) => "TRUE",
            None => "FALSE",
        };
        let expires = match cookie.expires() {
            Some(x) => match x.datetime() {
                Some(y) => y.unix_timestamp(),
                None => 0,
            },
            None => 0,
        };
        let name = cookie.name();
        let value = cookie.value();

        let cookie_string = format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}",
            String::from(".") + domain,
            sub_domain,
            path,
            secure,
            expires,
            name,
            value
        );

        netscape_format.push_str(&format!("{}\n", cookie_string));
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
