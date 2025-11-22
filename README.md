create a my_ed25519_priv_key_for_sign and put your private key for sign in it. it must be ed25519/EdDSA

create a file call msg.json, and put msg in it for sign:

```
{

	"msg": "this is a good thing"

}
```

and run:
cargo run


send send alice.json or content of it to Bob to verify
