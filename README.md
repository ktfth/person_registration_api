# Person Registration Rest API

## Descrption

Rest API for the person registration module, with that resource
you can generate CPF/CNPJ for test data population.

## Usage

```
[GET] /
{"ping": "pong"}
```

```
[GET] /?kind=physical
{"data": "999.999.999-99"}
```

```
[GET] /?kind=juridic
{"data": "99.999.999/9999-99"}
```

That's it simple enough to get started!