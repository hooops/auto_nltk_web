use hex_literal::hex;
use jwt::{AlgorithmType, Header, SignWithKey, Token};
use md5::{Digest, Md5};
use std::process::Command;
pub fn md5s(data: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(data);
    let hash = hasher.finalize();
    let d = format!("{:x}", hash);
    d
}
fn main() {
    jwt_make();
    let mut index = 0;
    for i in 1..2 {
        let d = &md5s(&md5s(&i.to_string()))[1..7];

        do_shell(d);
        index += 1;
        if index > 1000 {
            println!("{:?}", 1)
        }
    }
}
pub fn do_shell(code: &str) {
    // cmd_str可以是从输入流读取或从文件里读取

    let cmd_str = format!(
        r#"
    curl "https://search.google.com/_/SearchConsoleAggReportUi/data/batchexecute?rpcids=RVtklb&source-path=^%^2Fsearch-console^%^2Finspect&f.sid=2564535257712016514&bl=boq_searchconsoleserver_20240131.00_p0&hl=zh-CN&soc-app=1&soc-platform=1&soc-device=1&_reqid=2136249&rt=c" ^
  -H "authority: search.google.com" ^
  -H "accept: */*" ^
  -H "accept-language: zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6" ^
  -H "content-type: application/x-www-form-urlencoded;charset=UTF-8" ^
  -H "cookie: SID=g.a000fwjKKOjLrazTgta7USa1HuDtDZ4kkfdWQLfrS6wBFa_sdxq3YSjg2RwG3sFoNTKjEJDIiAACgYKAfESAQASFQHGX2MiNNleUaVqoLbcmlTEXHq-QBoVAUF8yKov_7aMHtkBP_6Tj2QWSl1i0076; __Secure-1PSID=g.a000fwjKKOjLrazTgta7USa1HuDtDZ4kkfdWQLfrS6wBFa_sdxq32DiPrcl41I_cU9Z6p2YcMgACgYKAXESAQASFQHGX2MiI1FUqJmUoidyKSRYdMzpwRoVAUF8yKrDtUx26uTobsi6BSN1soF50076; __Secure-3PSID=g.a000fwjKKOjLrazTgta7USa1HuDtDZ4kkfdWQLfrS6wBFa_sdxq3rf2PqIro2FosTg_Pt58EWQACgYKAQYSAQASFQHGX2MitAyOIyeygiPriaA203fBDRoVAUF8yKpUBbshSewyz33d-UKvfhXn0076; HSID=Aaa1DPxAtpvOPyf3c; SSID=AdXTyvsIdqqMVawnS; APISID=hjz6LA_NMITeX7xl/AaqNaMf8W9q3FN_jR; SAPISID=RcZ5ZwDcZ2noJSrB/AcLs2j_LKKfPF0OFr; __Secure-1PAPISID=RcZ5ZwDcZ2noJSrB/AcLs2j_LKKfPF0OFr; __Secure-3PAPISID=RcZ5ZwDcZ2noJSrB/AcLs2j_LKKfPF0OFr; OTZ=7404596_88_88_104280_84_446940; _ga_MKSWTKHW3M=GS1.1.1706580001.1.1.1706580175.0.0.0; _ga=GA1.1.1226687806.1706579733; SEARCH_SAMESITE=CgQIqpoB; 1P_JAR=2024-02-08-07; AEC=Ae3NU9M7r_qN4ONj3WyA3hfiXmjry6MIhFrBRDNJycnXCvVRhIkAitrxUIk; NID=511=Y0u1r_GZw8j86mDe3pMVCYuBUzfVSAH069PdCDnyy8lReK7wVyInbF2U_dtK3GkoYkOW6B5AioUlFHAc1aUlvAMCYRdoIeyXrnZ0HPdbROyXLXIM0TOoFr18iNy8a8maG0FQ0yRVU5vBh0toegxYWpmuT1ru8xhGK6NxX9RgKTiyy0M2e3aAZvklm5YbmxUDLGpU7f3YmJO1rzM_KlEpPLhsDy7JyEzy_etJQDxfzW5kfKNkjJZ4YxEUGMqYOazwv8GNsnLnllaR0oJou_gBf2kQKFiYqSqi0fdpYg_bWtIEt_5d9f6LBy9ofMZEPsk5I7AajRsjhlY0I6SCyo-Cc0SZEhtkzyjlkJPxqOgh; __Secure-1PSIDTS=sidts-CjIBYfD7Z4KHpf3AYKewkMOHTRCmCUBcbzQsggOuRsDPffSWz4vQL2DJRYJAvjWBS4oqLxAA; __Secure-3PSIDTS=sidts-CjIBYfD7Z4KHpf3AYKewkMOHTRCmCUBcbzQsggOuRsDPffSWz4vQL2DJRYJAvjWBS4oqLxAA; SIDCC=ABTWhQFdvXVh_n9EOCD7axlv_sILUp7Agg61fEO3pc9eCHaMUcIarJ5i4Mh51ftak742MU3SiA; __Secure-1PSIDCC=ABTWhQH2I4y5y2jIU5wPLNJrJMJjPy0uPpJ0UYlwE01H3qtAnSdGb8Q42mQ9eem2MxeQ96nF_Q; __Secure-3PSIDCC=ABTWhQFWc8LHpZe6GgU6xRybLgzlTAR0gfZjUjkuWHF_nZQ9VxXgSDXs3pNW_5IQJHpN3OPUrD0; _ga_QX2LK1FZEG=GS1.1.1707703241.30.1.1707703639.0.0.0" ^
  -H "origin: https://search.google.com" ^
  -H "referer: https://search.google.com/" ^
  -H "sec-ch-ua: ^\^"Not A(Brand^\^";v=^\^"99^\^", ^\^"Chromium^\^";v=^\^"121^\^", ^\^"Google Chrome^\^";v=^\^"121^\^"" ^
  -H "sec-ch-ua-arch: ^\^"x86^\^"" ^
  -H "sec-ch-ua-bitness: ^\^"64^\^"" ^
  -H "sec-ch-ua-full-version: ^\^"121.0.6167.160^\^"" ^
  -H "sec-ch-ua-full-version-list: ^\^"Not A(Brand^\^";v=^\^"99.0.0.0^\^", ^\^"Chromium^\^";v=^\^"121.0.6167.160^\^", ^\^"Google Chrome^\^";v=^\^"121.0.6167.160^\^"" ^
  -H "sec-ch-ua-mobile: ?0" ^
  -H "sec-ch-ua-model: ^\^"^\^"" ^
  -H "sec-ch-ua-platform: ^\^"Windows^\^"" ^
  -H "sec-ch-ua-platform-version: ^\^"15.0.0^\^"" ^
  -H "sec-ch-ua-wow64: ?0" ^
  -H "sec-fetch-dest: empty" ^
  -H "sec-fetch-mode: cors" ^
  -H "sec-fetch-site: same-origin" ^
  -H "user-agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36" ^
  -H "x-same-domain: 1" ^
  --data-raw "f.req=^%^5B^%^5B^%^5B^%^22RVtklb^%^22^%^2C^%^22^%^5B^%^5C^%^22http^%^3A^%^2F^%^2Fen0.figureaspect.com^%^2Fhtml^%^2F {}! .html^%^5C^%^22^%^2Cnull^%^2C4^%^2C1^%^2C^%^5C^%^22sc-domain^%^3Afigureaspect.com^%^5C^%^22^%^5D^%^22^%^2Cnull^%^2C^%^22generic^%^22^%^5D^%^5D^%^5D&at=AJDi_Mhphaem6MJlcTQfIsV_1POb^%^3A1707703445491&" ^
  --compressed
    "#,
        code
    );

    let output = if cfg!(target_os = "windows") {
        // println!("{:?}",Command::new("cmd").arg("/c").arg(cmd_str).output().ok());
    } else {
        //println!("{:?}",Command::new("sh").arg("-c").arg(cmd_str).output().ok());
    };
}
