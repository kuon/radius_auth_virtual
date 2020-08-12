#[macro_use]
extern crate pamsm;

use pamsm::{Pam, PamError, PamFlag, PamLibExt, PamServiceModule};

use radius_virtual::prelude::*;

struct PamTime;

impl PamServiceModule for PamTime {
    fn authenticate(
        pamh: Pam,
        _flags: PamFlag,
        _args: Vec<String>,
    ) -> PamError {
        let pass = match pamh.get_authtok(None) {
            Ok(Some(p)) => p,
            Ok(None) => return PamError::AUTH_ERR,
            Err(e) => return e,
        };

        let user = match pamh.get_user(None) {
            Ok(Some(u)) => u,
            Ok(None) => return PamError::USER_UNKNOWN,
            Err(e) => return e,
        };

        let user = match user.to_str() {
            Ok(u) => u,
            _ => return PamError::USER_UNKNOWN,
        };

        let pass = match pass.to_str() {
            Ok(p) => p,
            _ => return PamError::AUTH_ERR,
        };

        let config = Config::system();

        let config = match config {
            Ok(config) => config,
            _ => return PamError::SERVICE_ERR,
        };

        let client = Client::with_config(&config);

        let client = match client {
            Ok(client) => client,
            _ => return PamError::SERVICE_ERR,
        };

        let db = Db::with_config(&config);

        let mut db = match db {
            Ok(db) => db,
            _ => return PamError::SERVICE_ERR,
        };

        let cred = Credentials::with_username_password(user, pass);

        let res = client.authenticate(&cred);

        let user = match res {
            Ok(user) => user,
            Err(Error::AuthReject) => return PamError::AUTH_ERR,
            _ => return PamError::SERVICE_ERR,
        };

        let res = db::User::lookup(&config, &user);

        let user = match res {
            Some(user) => user,
            _ => return PamError::AUTH_ERR,
        };

        let res = db.store_user(&user);

        match res {
            Ok(_) => return PamError::SUCCESS,
            _ => return PamError::SERVICE_ERR,
        };
    }

    fn setcred(pamh: Pam, _flags: PamFlag, _args: Vec<String>) -> PamError {

        let config = Config::system();

        let config = match config {
            Ok(config) => config,
            _ => return PamError::SERVICE_ERR,
        };


        let db = Db::with_config(&config);

        let mut db = match db {
            Ok(db) => db,
            _ => return PamError::SERVICE_ERR,
        };

        let user = match pamh.get_user(None) {
            Ok(Some(u)) => u,
            Ok(None) => return PamError::USER_UNKNOWN,
            Err(e) => return e,
        };

        let user = match user.to_str() {
            Ok(u) => u,
            _ => return PamError::USER_UNKNOWN,
        };

        let user = db.get_user(user);

        match user {
            Ok(_user) => PamError::SUCCESS,
            _ => PamError::AUTH_ERR
        }
    }
}

pam_module!(PamTime);
