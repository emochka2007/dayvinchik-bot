Steps to login

- Increase the read timeout on `receive` function in main.rs
- Add code below inside read.rs
- ```rust
     if td_type == "updateAuthorizationState" {
         error!(
             "Use the link from logs to login, your auth is not setup maybe {}",
             json_str
         );
         auth_tdlib(json_str)?;
    }

```
- Scan qr code in telegram app