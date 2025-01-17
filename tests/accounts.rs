use axum::extract::Request;
use common::RequestBuilderExt as _;
use sqlx::PgPool;

use bromide::{router::router, schema::*};
use tower::ServiceExt;

mod common;

const SECRET: &str = "Wmfv3899gc9";

#[sqlx::test]
async fn register_fail_if_invalid_secret(pool: PgPool) {
    let mut server = router(pool);

    let response = (&mut server)
        .oneshot(
            Request::post("/accounts/registerGJAccount.php").form(&RegisterGJAccount {
                secret: "".into(),
                user_name: "testing".into(),
                password: "testing".into(),
                email: "testing@test.test".into(),
            }),
        )
        .await
        .unwrap();

    assert_eq!(common::body_into_string(response.into_body()).await, "-1")
}

#[sqlx::test]
async fn register_fail_if_invalid_user_name(pool: PgPool) {
    let mut server = router(pool);

    const INVALID_USER_NAMES: [&str; 6] = [
        "sasa_ffsaas!",
        "$#!$*@!&Ssf",
        "SSSSS--_SS!SSS",
        ")91219$--*@&*!",
        "21312132__1312312313",
        "__$@!$@%!#@(#!@(*))",
    ];

    for user_name in INVALID_USER_NAMES {
        let response = (&mut server)
            .oneshot(
                Request::post("/accounts/registerGJAccount.php").form(&RegisterGJAccount {
                    secret: SECRET.into(),
                    user_name: user_name.into(),
                    password: "test".into(),
                    email: "testing@test.test".into(),
                }),
            )
            .await
            .unwrap();

        assert_eq!(common::body_into_string(response.into_body()).await, "-1");
    }
}

#[sqlx::test]
async fn register_fail_on_user_name_length(pool: PgPool) {
    let mut server = router(pool);

    // too short
    let response = (&mut server)
        .oneshot(
            Request::post("/accounts/registerGJAccount.php").form(&RegisterGJAccount {
                secret: SECRET.into(),
                user_name: "t".into(),
                password: "testing".into(),
                email: "testing@test.test".into(),
            }),
        )
        .await
        .unwrap();

    assert_eq!(common::body_into_string(response.into_body()).await, "-9");

    // too long
    let response = (&mut server)
        .oneshot(
            Request::post("/accounts/registerGJAccount.php").form(&RegisterGJAccount {
                secret: SECRET.into(),
                user_name: "testtesttesttesttestt".into(), // 1 more than allowed
                password: "testing".into(),
                email: "testing@test.test".into(),
            }),
        )
        .await
        .unwrap();

    assert_eq!(common::body_into_string(response.into_body()).await, "-4");
}

#[sqlx::test]
async fn register_fail_if_invalid_password(pool: PgPool) {
    let mut server = router(pool);

    const INVALID_PASSWORDS: [&str; 6] = [
        "sasa_ffsaas!",
        "$#!$*@!&Ssf",
        "SSSSS--_SS!SSS",
        ")91219$--*@&*!",
        "213!2132__131!*$@",
        "__$@!$@%!#@(#!@(*))",
    ];

    for password in INVALID_PASSWORDS {
        let response = (&mut server)
            .oneshot(
                Request::post("/accounts/registerGJAccount.php").form(&RegisterGJAccount {
                    secret: SECRET.into(),
                    user_name: "test".into(),
                    password: password.into(),
                    email: "testing@test.test".into(),
                }),
            )
            .await
            .unwrap();

        println!("testing invalid password: {}", password);
        assert_eq!(common::body_into_string(response.into_body()).await, "-5");
    }
}

#[sqlx::test]
async fn register_fail_on_password_length(pool: PgPool) {
    let mut server = router(pool);

    // too short
    let response = (&mut server)
        .oneshot(
            Request::post("/accounts/registerGJAccount.php").form(&RegisterGJAccount {
                secret: SECRET.into(),
                user_name: "test".into(),
                password: "t".into(),
                email: "testing@test.test".into(),
            }),
        )
        .await
        .unwrap();

    assert_eq!(common::body_into_string(response.into_body()).await, "-8");

    // too long
    let response = (&mut server)
        .oneshot(
            Request::post("/accounts/registerGJAccount.php").form(&RegisterGJAccount {
                secret: SECRET.into(),
                user_name: "test".into(),
                password: "testingtestingtesting".into(), // exactly 1 more than allowed
                email: "testing@test.test".into(),
            }),
        )
        .await
        .unwrap();

    assert_eq!(common::body_into_string(response.into_body()).await, "-5");
}

#[sqlx::test]
async fn register_fail_if_invalid_email(pool: PgPool) {
    let mut server = router(pool);

    const INVALID_EMAILS: [&str; 7] = [
        "plainaddress",
        "@%^%#$@#$@#.com",
        "@example.com",
        "email.example.com",
        "email..email@example.com",
        "Abc..123@example.com",
        "”(),:;<>[\\]@example.com",
    ];

    for email in INVALID_EMAILS {
        let response = (&mut server)
            .oneshot(
                Request::post("/accounts/registerGJAccount.php").form(&RegisterGJAccount {
                    secret: SECRET.into(),
                    user_name: "test".into(),
                    password: "testing".into(),
                    email: email.into(),
                }),
            )
            .await
            .unwrap();

        println!("testing invalid email: {}", email);
        assert_eq!(common::body_into_string(response.into_body()).await, "-6");
    }
}

#[sqlx::test]
async fn register_fail_if_user_name_exists(pool: PgPool) {
    let mut server = router(pool);

    let response = (&mut server)
        .oneshot(
            Request::post("/accounts/registerGJAccount.php").form(&RegisterGJAccount {
                secret: SECRET.into(),
                user_name: "testing".into(),
                password: "testing".into(),
                email: "testing1@test.test".into(),
            }),
        )
        .await
        .unwrap();

    assert_eq!(common::body_into_string(response.into_body()).await, "1");

    let response = (&mut server)
        .oneshot(
            Request::post("/accounts/registerGJAccount.php").form(&RegisterGJAccount {
                secret: SECRET.into(),
                user_name: "testing".into(),
                password: "testing".into(),
                email: "testing2@test.test".into(),
            }),
        )
        .await
        .unwrap();

    assert_eq!(common::body_into_string(response.into_body()).await, "-2");
}

#[sqlx::test]
async fn register_fail_if_email_exists(pool: PgPool) {
    let mut server = router(pool);

    let response = (&mut server)
        .oneshot(
            Request::post("/accounts/registerGJAccount.php").form(&RegisterGJAccount {
                secret: SECRET.into(),
                user_name: "testingemail".into(),
                password: "testing".into(),
                email: "testingemail@test.test".into(),
            }),
        )
        .await
        .unwrap();

    assert_eq!(common::body_into_string(response.into_body()).await, "1");

    let response = (&mut server)
        .oneshot(
            Request::post("/accounts/registerGJAccount.php").form(&RegisterGJAccount {
                secret: SECRET.into(),
                user_name: "testingemail2".into(),
                password: "testing".into(),
                email: "testingemail@test.test".into(),
            }),
        )
        .await
        .unwrap();

    assert_eq!(common::body_into_string(response.into_body()).await, "-3");
}

#[sqlx::test]
async fn login_fail_if_invalid_secret(pool: PgPool) {
    let mut server = router(pool);

    let response = (&mut server)
        .oneshot(
            Request::post("/accounts/loginGJAccount.php").form(&LoginGJAccount {
                user_name: "testing".into(),
                gjp2: "testing".into(), // it's invalid but it doesn't matter here
                secret: "".into(),
            }),
        )
        .await
        .unwrap();

    assert_eq!(common::body_into_string(response.into_body()).await, "-1")
}

#[sqlx::test]
async fn login_fail_on_user_name_length(pool: PgPool) {
    let mut server = router(pool);

    // too short
    let response = (&mut server)
        .oneshot(
            Request::post("/accounts/loginGJAccount.php").form(&LoginGJAccount {
                user_name: "testa".into(),
                gjp2: "testing".into(), // it's invalid but it doesn't matter here
                secret: SECRET.into(),
            }),
        )
        .await
        .unwrap();

    assert_eq!(common::body_into_string(response.into_body()).await, "-9")
}

#[sqlx::test(fixtures("accounts", "users"))]
async fn login_success(pool: PgPool) {
    let mut server = router(pool);

    let mut accounts = std::collections::HashMap::new();
    accounts.insert((1, "test1aa"), "daa5183ce7724fbb18cc7fdb6f001cdc125dfd36");
    accounts.insert((2, "test2aa"), "dab00cb25233b69987a07278915efa3e01c3bcaf");
    accounts.insert((3, "test3aa"), "e72d536b3a1b518f34218db513015ab86107c879");

    for ((id, user_name), gjp2) in accounts {
        let response = (&mut server)
            .oneshot(
                Request::post("/accounts/loginGJAccount.php").form(&LoginGJAccount {
                    user_name: user_name.into(),
                    gjp2: gjp2.into(), // it's invalid but it doesn't matter here
                    secret: SECRET.into(),
                }),
            )
            .await
            .unwrap();

        assert_eq!(
            common::body_into_string(response.into_body()).await,
            format!("{id},{id}")
        )
    }
}
