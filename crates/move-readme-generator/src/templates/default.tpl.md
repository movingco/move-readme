# {{ title }}

{{ body }}

## Installation

To use {{ idl.name }} in your code, add the following to the `[addresses]` section of your `Move.toml`:

```toml
[addresses]
{{ idl.name }} = "{{ address }}"
```

{% if license %}

## License

{{ license }}
{% endif %}
