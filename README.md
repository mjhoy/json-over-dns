json-over-dns
=============

Get base64 encoded strings from DNS TXT records.

## Example

```
% json-over-dns encode "{\"val\":\"foo\"}"
Put this in your TXT record (including the quotes):
"eyJ2YWwiOiJmb28ifQ=="
```

After adding a TXT record with this value:

```
% json-over-dns fetch _txt.mikey.bike | jq
{
  "val": "foo"
}
```
