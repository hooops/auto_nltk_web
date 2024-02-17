// use hmac::{Hmac, Mac};
// use jwt::{AlgorithmType, Header, SignWithKey, Token,VerifyWithKey};
// use sha2::Sha384;
// use std::collections::BTreeMap;
// use sha2::Sha256;
// pub fn jwt_make(uuid:&str,exp:&str,ip:&str,clientMd5:&str){

//     let key: Hmac<Sha384> = Hmac::new_from_slice(b"some-secret").unwrap();
//     let header = Header {
//         algorithm: AlgorithmType::Hs384,
//         ..Default::default()
//     };
//     let mut claims = BTreeMap::new();
//     claims.insert("UUID", "");
//     claims.insert("IP", "");
//     claims.insert("EXP", "");
//     claims.insert("CMD5", "");

//     let token = Token::new(header, claims).sign_with_key(&key).unwrap();

//     assert_eq!(token.as_str(), "eyJhbGciOiJIUzM4NCJ9.eyJzdWIiOiJzb21lb25lIn0.WM_WnPUkHK6zm6Wz7zk1kmIxz990Te7nlDjQ3vzcye29szZ-Sj47rLNSTJNzpQd_");
// }
// pub fn verify_token(key:&[u8]) -> &str{

//    let token_str="bbb";
//     let token: Token<Header, BTreeMap<String, String>, _> = VerifyWithKey::verify_with_key(token_str, &key).unwrap();
// let header = token.header();
// let claims = token.claims();
//     return claims["UUID"];
// }
